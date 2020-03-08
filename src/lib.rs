use std::collections::HashMap;

mod point;
pub use point::*;

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TouchType {
	Start,
	Move,
	End,
}

pub trait GestureEvents {
	fn touch_one_start(&mut self, _pos: &Point) {}
	fn touch_one_move(&mut self, _pos: &Point, _offset: &Point) {}
	fn touch_one_end(&mut self) {}

	fn touch_scale_start(&mut self, _pos: &Point) {}
	fn touch_scale_change(&mut self, _scale: f32, _pos: &Point, _offset: &Point) {}
	fn touch_scale_end(&mut self) {}

	fn touch_three_start(&mut self, _pos: &Point) {}
	fn touch_three_move(&mut self, _pos: &Point, _offset: &Point) {}
	fn touch_three_end(&mut self) {}
}

pub struct GestureRecognizer {
	current_touches: HashMap<u64, Point>,

	one_touch_regime: bool,
	one_touch_pos: Point,

	two_touch_regime: bool,
	two_touch_pos: Point,
	scale_start: f32,

	three_touch_regime: bool,
	three_touch_pos: Point,
}

impl Default for GestureRecognizer {
	fn default() -> Self {
		GestureRecognizer {
			current_touches: HashMap::new(),

			one_touch_regime: false,
			one_touch_pos: Point::default(),

			two_touch_regime: false,
			two_touch_pos: Point::default(),
			scale_start: 0.0,
			
			three_touch_regime: false,
			three_touch_pos: Point::default(),
		}
	}
}

impl GestureRecognizer {
	fn get_first_touch(&self) -> Option<&Point> {
		if let Some((_, pos)) = self.current_touches.iter().next() {
			Some(pos)
		} else {
			None
		}
	}

	fn get_first_two_touches(&self) -> Option<(&Point, &Point)> {
		let mut iter = self.current_touches.iter();
		if let Some((_, pos1)) = iter.next() {
			if let Some((_, pos2)) = iter.next() {
				Some((pos1, pos2))
			} else {
				None
			}
		} else {
			None
		}
	}

	fn get_first_three_touches(&self) -> Option<(&Point, &Point, &Point)> {
		let mut iter = self.current_touches.iter();
		if let Some((_, pos1)) = iter.next() {
			if let Some((_, pos2)) = iter.next() {
				if let Some((_, pos3)) = iter.next() {
					Some((pos1, pos2, pos3))
				} else {
					None
				}
			} else {
				None
			}
		} else {
			None
		}
	}
}

impl GestureRecognizer {
	pub fn process<GE: GestureEvents>(&mut self, ge: &mut GE, phase: TouchType, id: u64, x: f32, y: f32) {
		let pos: Point = (x, y).into();
		use TouchType::*;
		match phase {
			Start | Move   => { self.current_touches.insert(id, pos); },
			End => { self.current_touches.remove(&id); },
		}
		self.process_one_touch(ge);
		self.process_two_touches(ge);
		self.process_three_touches(ge);
	}

	fn process_one_touch<GE: GestureEvents>(&mut self, ge: &mut GE) {
		if self.current_touches.len() == 1 {
			let new_pos = self.get_first_touch().unwrap().clone();
			if self.one_touch_regime {
				ge.touch_one_move(&self.one_touch_pos, &(new_pos.clone() - &self.one_touch_pos));
				self.one_touch_pos = new_pos;
			} else {
				self.one_touch_pos = new_pos;
				self.one_touch_regime = true;
				ge.touch_one_start(&self.one_touch_pos);
			}
		} else if self.one_touch_regime {
			self.one_touch_regime = false;
			ge.touch_one_end();
		}
	}

	fn process_two_touches<GE: GestureEvents>(&mut self, ge: &mut GE) {
		if self.current_touches.len() == 2 {
			let (pos1, pos2) = self.get_first_two_touches().unwrap();
			let center = (pos1.clone() + pos2) / 2.0;
			let current_scale = (pos1.clone() - pos2).length();
			if self.two_touch_regime {
				ge.touch_scale_change(current_scale / self.scale_start, &center, &(center.clone() - &self.two_touch_pos));
				self.two_touch_pos = center;
			} else {
				self.two_touch_regime = true;
				self.scale_start = current_scale;
				self.two_touch_pos = center;
				ge.touch_scale_start(&self.two_touch_pos);
			}
		} else if self.two_touch_regime {
			self.two_touch_regime = false;
			ge.touch_scale_end();
		}
	}

	fn process_three_touches<GE: GestureEvents>(&mut self, ge: &mut GE) {
		if self.current_touches.len() == 3 {
			let (pos1, pos2, pos3) = self.get_first_three_touches().unwrap();
			let center = (pos1.clone() + pos2 + pos3) / 3.0;
			if self.three_touch_regime {
				ge.touch_three_move(&center, &(center.clone() - &self.three_touch_pos));
				self.three_touch_pos = center;
			} else {
				self.three_touch_regime = true;
				self.three_touch_pos = center;
				ge.touch_three_start(&self.three_touch_pos);
			}
		} else if self.three_touch_regime {
			self.three_touch_regime = false;
			ge.touch_three_end();
		}
	}
}

#[cfg(feature = "miniquad")]
impl From<miniquad::TouchPhase> for TouchType {
	fn from(val: miniquad::TouchPhase) -> Self {
		use miniquad::TouchPhase::*;
		use TouchType::*;
		match val {
			Started => Start,
			Moved => Move,
			Ended => End,
			Cancelled => End,
		}
	}
}
