use crate::utils::vector2::Vector2;

pub struct SpaceObject {
    pub pos: Vector2,
    pub dir: Vector2,
    pub angle: f64,
    pub size: usize,
}
