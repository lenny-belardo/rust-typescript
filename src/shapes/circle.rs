use super::area::Area;
use std::{f64::consts::PI, fmt::Display};
use std::str::FromStr;
use super::{collisions::{Contains, Points}};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Points for Circle {
    fn points(&self) -> super:: collisions::PointIter {
        return vec![(self.x, self.y)].into();
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Circle({}, {}): {}", self.x, self.y, self.radius);
    }
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;

        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split("").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("bad circle from str"));
        }

        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}