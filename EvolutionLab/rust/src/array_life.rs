use ndarray::{azip, s, Array2, ArrayView2, ArrayViewMut2};
use rand::Rng;

use crate::bridge::{Boundary, LifeAPI, Position, Shape};

pub struct ArrayLife {
    board: Array2<u8>,
    boundary: Boundary,
}

impl ArrayLife {
    pub fn new(shape: Shape, boundary: Boundary) -> Self {
        // 先转置形状，把 ndarray 的列主序变成 RLE 的行主序， 再扩大一圈，方便处理边界
        let shape = shape.t().add_scalar(2);

        Self {
            board: Array2::<u8>::zeros((shape.x, shape.y)),
            boundary,
        }
    }

    fn inner_cells(&self) -> ArrayView2<u8> {
        self.board.slice(s![1..-1, 1..-1])
    }

    fn inner_cells_mut(&mut self) -> ArrayViewMut2<u8> {
        self.board.slice_mut(s![1..-1, 1..-1])
    }

    fn next(&mut self) {
        let Self { board, .. } = self;

        /*
         *  ---------------
         *  | nw | n | ne |
         *  ---------------
         *  |  w | y |  e |
         *  ---------------
         *  | sw | s | se |
         *  ---------------
         *
         *  四周8个邻居的状态决定了中间的细胞 y 的状态:
         *    sum = e + s + w + n + ne + se + sw + nw
         *  如果 sum = 3 或 (sum = 2 且上一轮 y 存活)，本轮 y 存活，否则 y 死亡
         */
        let mut sum = azip! { @build par_map_collect (
            e in board.slice(s![2.., 1..-1]),
            s in board.slice(s![1..-1, 2..]),
            w in board.slice(s![..-2, 1..-1]),
            n in board.slice(s![1..-1, ..-2]),
        ) e + s + w + n};

        azip! { @build par_for_each (
            sum in sum.view_mut(),
            ne in board.slice(s![2.., 2..]),
            se in board.slice(s![2.., ..-2]),
            sw in board.slice(s![..-2, 2..]),
            nw in board.slice(s![..-2, ..-2]),
        ) *sum += ne + se + sw + nw};

        azip! { @build par_for_each (
            &n in sum.view(),
            y in self.inner_cells_mut(),
        ) *y = ((n == 3) || (n == 2 && *y == 1)) as u8};
    }

    fn touch_boundary(&mut self) {
        let turn = match self.boundary {
            Boundary::Sphere => -1,
            Boundary::Mirror => 1,
            Boundary::None => return,
        };

        let board = &mut self.board;
        let (n, m) = board.dim();

        let (mut outer_vertex, inner_vertex) =
            board.multi_slice_mut((s![..;(n - 1), ..;(m - 1)], s![1..-1;(n - 3), 1..-1;(m - 3)]));

        outer_vertex.assign(&inner_vertex.slice(s![..;turn, ..;turn]));

        let (mut outer_border, inner_border) =
            board.multi_slice_mut((s![1..-1, ..;(m - 1)], s![1..-1, 1..-1;(m - 3)]));

        outer_border.assign(&inner_border.slice(s![.., ..;turn]));

        let (mut outer_border, inner_border) =
            board.multi_slice_mut((s![..;(n - 1), 1..-1], s![1..-1;(n - 3), 1..-1]));

        outer_border.assign(&inner_border.slice(s![..;turn, ..]));
    }
}

impl LifeAPI for ArrayLife {
    fn evolve(&mut self, step: Option<u32>) {
        for _ in 0..step.unwrap_or(1) {
            self.touch_boundary();
            self.next();
        }
    }

    fn clean_cells(&mut self) {
        self.board.fill(0);
    }

    fn rand(&mut self, distr: f64) {
        let mut rng = rand::thread_rng();

        self.inner_cells_mut()
            .iter_mut()
            .for_each(|c| *c = (distr > rng.gen()) as u8);
    }

    fn get_cells(&self) -> Vec<Position> {
        let inner_board = self.inner_cells();
        let mut cells = vec![];

        // ndarray 经过转置所以 x 和 y 是反的
        for y in 0..inner_board.nrows() {
            for x in 0..inner_board.ncols() {
                if inner_board[[y, x]] == 1 {
                    cells.push(Position { x, y });
                }
            }
        }

        cells
    }

    fn set_cells(&mut self, cells: Vec<Position>) {
        let mut inner_board = self.inner_cells_mut();

        for Position { x, y } in cells {
            inner_board[[y, x]] = 1; // ndarray 经过转置所以 x 和 y 是反的
        }
    }

    fn set_boundary(&mut self, boundary: Boundary) {
        // 如果切换到截断边界，需要清理前一轮的边界
        if boundary == Boundary::None && self.boundary != Boundary::None {
            let (n, m) = self.board.dim();

            let mut outer = self.board.slice_mut(s![..;(n - 1), ..]);
            outer.fill(0);

            let mut outer = self.board.slice_mut(s![1..-1, ..;(m - 1)]);
            outer.fill(0);
        }

        self.boundary = boundary;
    }

    fn set_shape(&mut self, shape: Shape, clean: Option<bool>) {
        let Shape {x: n, y: m} = shape.t().add_scalar(2);
        let mut new = Array2::<u8>::zeros((n, m));

        if clean == Some(false) {
            let old = &self.board;
            let n = n.min(old.nrows());
            let m = m.min(old.ncols());

            new.slice_mut(s![..n, ..m]).assign(&old.slice(s![..n, ..m]));
        }

        self.board = new;
    }
}

impl Shape {
    /// 转置形状，把 ndarray 的列主序变成 RLE 的行主序
    fn t(self) -> Self {
        Self {x: self.y, y: self.x}
    }

    /// x 和 y 各加上一个值
    fn add_scalar(self, rhs: usize) -> Self {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}
