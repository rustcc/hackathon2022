use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use derive_more::Display;
use glam::Vec3;

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

    /// 对魔方进行多次转动
    ///
    /// # Examples
    ///
    /// 1. 转动顶层90°
    /// 2. 转动右层180度
    /// 3. 逆时针转动后层90°
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
/// [WCA表示法]: https://worldcubeassociation.org/regulations/#article-12-notation
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

/// 面的顺序
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
/// [WCA表示法]: https://worldcubeassociation.org/regulations/#article-12-notation
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    /// 转动顶层
    U(MoveVariant),
    /// 转动左层
    L(MoveVariant),
    /// 转动前层
    F(MoveVariant),
    /// 转动右层
    R(MoveVariant),
    /// 转动后层
    B(MoveVariant),
    /// 转动底层
    D(MoveVariant),
    /// 转动顶层起第n层
    Uw(CubeSize, MoveVariant),
    /// 转动左层起第n层
    Lw(CubeSize, MoveVariant),
    /// 转动前层起第n层
    Fw(CubeSize, MoveVariant),
    /// 转动右层起第n层
    Rw(CubeSize, MoveVariant),
    /// 转动后层起第n层
    Bw(CubeSize, MoveVariant),
    /// 转动底层起第n层
    Dw(CubeSize, MoveVariant),
    /// 沿x轴转动整个魔方
    X(MoveVariant),
    /// 沿y轴转动整个魔方
    Y(MoveVariant),
    /// 沿z轴转动整个魔方
    Z(MoveVariant),
    /// 转动中间的X轴
    M(MoveVariant),
    /// 转动中间的Y轴
    E(MoveVariant),
    /// 转动中间的Z轴
    S(MoveVariant),
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.get_move_name(), self.get_variant())
    }
}

impl Move {
    /// 获取转动的变量
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
            | Move::M(v)
            | Move::E(v)
            | Move::S(v)
            | Move::Uw(_, v)
            | Move::Lw(_, v)
            | Move::Fw(_, v)
            | Move::Rw(_, v)
            | Move::Bw(_, v)
            | Move::Dw(_, v) => *v,
        }
    }

    /// 获取转动的角度(弧度)
    pub fn angle(&self) -> f32 {
        match self.get_variant() {
            MoveVariant::Standard => FRAC_PI_2,
            MoveVariant::Double => PI,
            MoveVariant::Inverse => -FRAC_PI_2,
        }
    }

    pub fn clockwise(&self) -> bool {
        match self.get_variant() {
            MoveVariant::Standard => true,
            MoveVariant::Double => true,
            MoveVariant::Inverse => false,
        }
    }

    pub fn axis(&self) -> Vec3 {
        match self {
            Move::U(_) => -Vec3::Y,
            Move::L(_) => Vec3::X,
            Move::F(_) => -Vec3::Z,
            Move::R(_) => -Vec3::X,
            Move::B(_) => Vec3::Z,
            Move::D(_) => Vec3::Y,
            Move::X(_) => Vec3::X,
            Move::Y(_) => Vec3::Y,
            Move::Z(_) => Vec3::Z,
            Move::Uw(_, _) => Vec3::Y,
            Move::Lw(_, _) => Vec3::X,
            Move::Fw(_, _) => Vec3::Z,
            Move::Rw(_, _) => Vec3::X,
            Move::Bw(_, _) => Vec3::Z,
            Move::Dw(_, _) => Vec3::Y,
            Move::M(_) => Vec3::X,
            Move::E(_) => Vec3::Y,
            Move::S(_) => Vec3::Z,
        }
    }

    /// 用给定转动变量产生新的转动
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
            Move::M(_) => Move::M(variant),
            Move::E(_) => Move::E(variant),
            Move::S(_) => Move::S(variant),
        }
    }

    fn get_move_name(&self) -> String {
        match self {
            Move::U(_) => "U".to_string(),
            Move::L(_) => "L".to_string(),
            Move::F(_) => "F".to_string(),
            Move::R(_) => "R".to_string(),
            Move::B(_) => "B".to_string(),
            Move::D(_) => "D".to_string(),
            Move::Uw(n, _) => format!("{n}Uw"),
            Move::Lw(n, _) => format!("{n}Lw"),
            Move::Fw(n, _) => format!("{n}Fw"),
            Move::Rw(n, _) => format!("{n}Rw"),
            Move::Bw(n, _) => format!("{n}Bw"),
            Move::Dw(n, _) => format!("{n}Dw"),
            Move::X(_) => "X".to_string(),
            Move::Y(_) => "Y".to_string(),
            Move::Z(_) => "Z".to_string(),
            Move::M(_) => "M".to_string(),
            Move::E(_) => "E".to_string(),
            Move::S(_) => "S".to_string(),
        }
    }

    /// 反转
    pub fn prime(&self) -> Move {
        self.with_variant(self.get_variant().inverse())
    }
}

/// 控制转动的变量
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveVariant {
    /// 90°顺时针转动
    Standard = -1,
    /// 180°转动
    Double,
    /// 90°逆时针转动
    Inverse,
}

impl Display for MoveVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MoveVariant::Standard => "",
            MoveVariant::Double => "2",
            MoveVariant::Inverse => "'",
        };
        write!(f, "{}", s)
    }
}

impl MoveVariant {
    pub fn inverse(&self) -> MoveVariant {
        match self {
            MoveVariant::Standard => MoveVariant::Inverse,
            MoveVariant::Double => MoveVariant::Double,
            MoveVariant::Inverse => MoveVariant::Standard,
        }
    }
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

/// 产生随机的转动步骤
pub fn rand_moves(_size: CubeSize, max_step: usize) -> Vec<Move> {
    use rand::prelude::*;
    use Move::*;
    use MoveVariant::*;
    let mut rng = thread_rng();

    let mut moveset = Vec::new();

    let m_list = [U, R, F, L, D, B];
    let v_list = [Standard, Double, Inverse];
    for _ in 0..max_step {
        let m = rng.gen_range(0..6);
        let v = rng.gen_range(0..3);
        let mv = m_list[m];
        let variant = v_list[v];

        moveset.push(mv(variant));
    }

    moveset
}
