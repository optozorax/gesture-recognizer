use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone, Debug, Default)]
pub struct Point {
	pub x: f32,
	pub y: f32,
}

impl Add<&Point> for Point {
	type Output = Point;

	#[inline]
	fn add(self, _rhs: &Point) -> Point {
		Point { 
			x: self.x + _rhs.x, 
			y: self.y + _rhs.y
		}
	}
}

impl Sub<&Point> for Point {
	type Output = Point;

	#[inline]
	fn sub(self, _rhs: &Point) -> Point {
		Point { 
			x: self.x - _rhs.x, 
			y: self.y - _rhs.y
		}
	}
}

impl Mul<f32> for Point {
	type Output = Point;

	#[inline]
	fn mul(self, _rhs: f32) -> Point {
		Point { 
			x: self.x * _rhs, 
			y: self.y * _rhs
		}
	}
}

impl Div<f32> for Point {
	type Output = Point;

	#[inline]
	fn div(self, _rhs: f32) -> Point {
		Point { 
			x: self.x / _rhs, 
			y: self.y / _rhs
		}
	}
}

impl From<(f32, f32)> for Point {
	#[inline]
	fn from(val: (f32, f32)) -> Self {
		Point { x: val.0, y: val.1 }
	}
}

impl From<[f32; 2]> for Point {
	#[inline]
	fn from(val: [f32; 2]) -> Self {
		Point { x: val[0], y: val[1] }
	}
}

impl Into<(f32, f32)> for Point {
	#[inline]
	fn into(self) -> (f32, f32) {
		(self.x, self.y)
	}
}

impl Into<[f32; 2]> for Point {
	#[inline]
	fn into(self) -> [f32; 2] {
		[self.x, self.y]
	}
}

impl Point {
	pub fn length(&self) -> f32 {
		(self.x.powi(2) + self.y.powi(2)).sqrt()
	}
}