use std::hash::Hash;

use derive_more::Display;

pub type CubeSize = i32;

/// 魔方的抽象特质
pub trait Cube: Clone + Eq + Hash + PartialEq {
    /// 用指定的大小创建一个已经复原
    fn new(size: CubeSize) -> Self;

    /// 返回魔方大小
    fn size(&self) -> CubeSize;

    /// 用一维表示的魔方状态
    ///
    /// # Examples
    ///
    /// Solved 3x3x3 cube:
    ///
    /// ```rust
    /// use rubiks_solver::prelude::{Cube, Face::*};
    /// use rubiks_solver::FaceletCube;
    ///
    /// let cube = FaceletCube::new(3);
    /// assert_eq!(cube.state(), vec![
    ///     U, U, U, U, U, U, U, U, U,
    ///     R, R, R, R, R, R, R, R, R,
    ///     F, F, F, F, F, F, F, F, F,
    ///     D, D, D, D, D, D, D, D, D,
    ///     L, L, L, L, L, L, L, L, L,
    ///     B, B, B, B, B, B, B, B, B
    /// ]);
    /// ```
    fn state(&self) -> Vec<Face>;

    /// 判断魔方是否被复原
    fn is_solved(&self) -> bool {
        fn all_equal<T: Clone + PartialEq>(arr: &[T]) -> bool {
            arr.iter().all(|x| *x == arr[0])
        }

        let face_length = (self.size() * self.size()) as usize;
        let state = self.state();

        let mut is_solved = true;
        for i in 0..6 {
            let face_start = i * face_length;
            let face_end = face_start + face_length;

            is_solved = is_solved && all_equal(&state[face_start..face_end]);
        }

        is_solved
    }

    /// 根据传入的函数，将魔方的指定部分替换为占位符，加快求解速度
    ///
    ///
    /// # Examples
    ///
    /// Cross Mask
    ///
    /// ```rust
    /// use rubiks_solver::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use rubiks_solver::FaceletCube;
    /// use rubiks_solver::sticker_index;
    ///
    /// let cross_pieces = [
    ///     sticker_index(3, D, 2), sticker_index(3, D, 4),
    ///     sticker_index(3, D, 6), sticker_index(3, D, 8),
    /// ];
    ///
    /// let masked_cube = FaceletCube::new(3).mask(&|i, f| if cross_pieces.contains(&i) { f } else { X });
    /// assert_eq!(masked_cube.state(), vec![
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, D, X, D, X, D, X, D, X,
    ///      X, X, X, X, X, X, X, X, X,
    ///      X, X, X, X, X, X, X, X, X
    /// ]);
    /// ```
    fn mask(&self, mask: &dyn Fn(CubeSize, Face) -> Face) -> Self;

    /// 转动魔方
    ///
    /// # Examples
    ///
    /// 转动上层90°
    ///
    /// ```rust
    /// use rubiks_solver::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use rubiks_solver::FaceletCube;
    ///
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_move(Move::U(MoveVariant::Standard));
    /// assert_eq!(turned_cube.state(), vec![
    ///     U, U, U, U, U, U, U, U, U,
    ///     B, B, B, R, R, R, R, R, R,
    ///     R, R, R, F, F, F, F, F, F,
    ///     D, D, D, D, D, D, D, D, D,
    ///     F, F, F, L, L, L, L, L, L,
    ///     L, L, L, B, B, B, B, B, B
    /// ]);
    /// ```
    fn apply_move(&self, mv: Move) -> Self;

    /// Apply a sequence of moves to a cube.
    ///
    /// # Examples
    ///
    /// Rotate the upper layer by 90 degrees:
    ///
    /// ```rust
    /// use rubiks_solver::prelude::{Cube, Face::*, Move, MoveVariant};
    /// use rubiks_solver::FaceletCube;
    ///
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_moves(&vec![
    ///     Move::U(MoveVariant::Standard),
    ///     Move::R(MoveVariant::Double),
    ///     Move::B(MoveVariant::Inverse),
    /// ]);
    /// assert_eq!(turned_cube.state(), vec![
    ///     L, L, F, U, U, D, U, U, D,
    ///     R, R, U, R, R, U, B, B, D,
    ///     R, R, B, F, F, B, F, F, L,
    ///     D, D, U, D, D, U, B, R, R,
    ///     D, F, F, D, L, L, U, L, L,
    ///     L, B, B, L, B, B, F, F, R
    /// ]);
    /// ```
    fn apply_moves(&self, mvs: &[Move]) -> Self
    where
        Self: Sized,
    {
        let mut cube = self.clone();

        for mv in mvs {
            cube = cube.apply_move(*mv);
        }

        cube
    }
}

/// 用[WCA表示法]表示贴纸的方向
///
/// [WCA表示法]: worldcubeassociation.org/regulations/#article-12-notation
#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum Face {
    /// 上面
    U,
    /// 左面
    L,
    /// 前面
    F,
    /// 右面
    R,
    /// 后面
    B,
    /// 下面
    D,
    /// 占位符
    X,
}

/// A designated ordering of the faces.
pub const ORDERED_FACES: [Face; 6] = [Face::U, Face::R, Face::F, Face::D, Face::L, Face::B];

/// 获取指定面上的指定位置的索引
///
/// # Examples
///
/// 3阶魔方的前面的第一块
///
/// ```rust
/// use rubiks_solver::prelude::{Cube, Face};
/// use rubiks_solver::sticker_index;
///
/// assert_eq!(sticker_index(3, Face::F, 1), 18);
/// ```
pub fn sticker_index(size: CubeSize, face: Face, index: CubeSize) -> CubeSize {
    (ORDERED_FACES.iter().position(|&f| f == face).unwrap() as CubeSize) * size * size + index
        - 1 as CubeSize
}

/// 用[WCA表示法]表示的 NxNxN 魔方的移动方法
///
/// 每个移动方法都有一个移动变量控制移动的方向
/// .
///
/// [WCA表示法]: worldcubeassociation.org/regulations/#article-12-notation
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    /// 旋转顶层
    U(MoveVariant),
    /// 旋转左层
    L(MoveVariant),
    /// 旋转前层
    F(MoveVariant),
    /// 旋转右层
    R(MoveVariant),
    /// 旋转后层
    B(MoveVariant),
    /// 旋转底层
    D(MoveVariant),
    /// 旋转顶层起第n层
    Uw(CubeSize, MoveVariant),
    /// 旋转左层起第n层
    Lw(CubeSize, MoveVariant),
    /// 旋转前层起第n层
    Fw(CubeSize, MoveVariant),
    /// 旋转右层起第n层
    Rw(CubeSize, MoveVariant),
    /// 旋转后层起第n层
    Bw(CubeSize, MoveVariant),
    /// 旋转底层起第n层
    Dw(CubeSize, MoveVariant),
    /// 沿x轴旋转整个魔方
    X(MoveVariant),
    /// 沿y轴旋转整个魔方
    Y(MoveVariant),
    /// 沿z轴旋转整个魔方
    Z(MoveVariant),
}

impl Move {
    /// 提取Move的MoveVariant
    pub fn get_variant(&self) -> MoveVariant {
        match self {
            Move::U(v)
            | Move::L(v)
            | Move::F(v)
            | Move::R(v)
            | Move::B(v)
            | Move::D(v)
            | Move::X(v)
            | Move::Y(v)
            | Move::Z(v)
            | Move::Uw(_, v)
            | Move::Lw(_, v)
            | Move::Fw(_, v)
            | Move::Rw(_, v)
            | Move::Bw(_, v)
            | Move::Dw(_, v) => *v,
        }
    }

    /// Returns the Move with the given MoveVariant.
    pub fn with_variant(&self, variant: MoveVariant) -> Move {
        match self {
            Move::U(_) => Move::U(variant),
            Move::L(_) => Move::L(variant),
            Move::F(_) => Move::F(variant),
            Move::R(_) => Move::R(variant),
            Move::B(_) => Move::B(variant),
            Move::D(_) => Move::D(variant),
            Move::Uw(n, _) => Move::Uw(*n, variant),
            Move::Lw(n, _) => Move::Lw(*n, variant),
            Move::Fw(n, _) => Move::Fw(*n, variant),
            Move::Rw(n, _) => Move::Rw(*n, variant),
            Move::Bw(n, _) => Move::Bw(*n, variant),
            Move::Dw(n, _) => Move::Dw(*n, variant),
            Move::X(_) => Move::X(variant),
            Move::Y(_) => Move::Y(variant),
            Move::Z(_) => Move::Z(variant),
        }
    }
}

/// A move variation that must be applied to the ```Move``` struct.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveVariant {
    /// 90°顺时针旋转
    Standard = 1,
    /// 180°旋转
    Double,
    /// 90°逆时针旋转
    Inverse,
}

/// 获取给定大小的魔方的求解状态
pub fn solved_state(size: CubeSize) -> Vec<Face> {
    ORDERED_FACES
        .iter()
        .flat_map(|&face| vec![face; (size * size) as usize])
        .collect()
}

/// 获取给定的魔方大小所有可能的移动
pub fn all_moves(size: CubeSize) -> Vec<Move> {
    use Move::*;
    use MoveVariant::*;

    let mut moveset = Vec::new();

    for mv in [U, R, F, L, D, B] {
        for variant in [Standard, Double, Inverse] {
            moveset.push(mv(variant));
        }
    }

    for mv in [Uw, Lw, Fw, Rw, Bw, Dw] {
        for variant in [Standard, Double, Inverse] {
            for slice in 1..=(size / 2) {
                moveset.push(mv(slice, variant));
            }
        }
    }

    moveset
}
