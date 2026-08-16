use glam::Vec2;

/// world space
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub min: Vec2,
    pub max: Vec2,
}
impl Viewport {
    pub fn from_bounds(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
    pub fn from_centre(centre: Vec2, dims: Vec2) -> Self {
        let offset = dims / 2.0;
        Self {
            min: centre - offset,
            max: centre + offset,
        }
    }
    pub fn centre(&self) -> Vec2 {
        self.min.midpoint(self.max)
    }
    pub fn dims(&self) -> Vec2 {
        self.max - self.min
    }
}
