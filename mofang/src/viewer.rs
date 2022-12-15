use bevy::prelude::*;

/// 可视化插件
pub struct ViewerPlugin;

impl Plugin for ViewerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CubeSettings>();
    }
}

/// 魔方设置
#[derive(Debug, Resource)]
pub struct CubeSettings {
    /// 是几阶的魔方
    pub cube_order: u8,
    /// 块的大小
    pub piece_size: f32,
    /// 前面的颜色
    pub front_color: Color,
    /// 后面的颜色
    pub back_color: Color,
    /// 左面的颜色
    pub left_color: Color,
    /// 右面的颜色
    pub right_color: Color,
    /// 上面的颜色
    pub top_color: Color,
    /// 下面的颜色
    pub bottom_color: Color,
}

impl Default for CubeSettings {
    fn default() -> Self {
        Self {
            cube_order: 3,
            piece_size: 1.0,
            front_color: Color::GREEN,
            back_color: Color::BLUE,
            left_color: Color::ORANGE,
            right_color: Color::RED,
            top_color: Color::WHITE,
            bottom_color: Color::YELLOW,
        }
    }
}
