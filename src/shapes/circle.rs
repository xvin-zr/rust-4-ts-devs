use super::{
    area::Area,
    collisions::{Contains, Points},
};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        return dx * dx + dy * dy <= self.r * self.r;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.r * self.r * std::f64::consts::PI;
    }
}
