use crate::{
	Font, FontFlag, InputContext, InvalidateFontMetrics, Item, Paintable, RenderContext, Updateable,
};

const BOX_SIZE: (usize, usize) = (10, 10);
const TEXT_OFFSET: usize = 20;

pub struct CheckBox<T: AsRef<str>> {
	position: (usize, usize),
	name: T,
	state: bool,
	is_pressed: bool,
	font_metrics: (usize, usize),
}

impl<T: AsRef<str>> CheckBox<T> {
	pub fn new(name: T) -> Self {
		Self {
			position: Default::default(),
			name,
			state: false,
			is_pressed: false,
			font_metrics: Default::default(),
		}
	}
}

impl<T: AsRef<str>> Updateable for CheckBox<T> {
	fn update(&mut self, ic: &impl InputContext) {
		let size = (TEXT_OFFSET + self.font_metrics.0, BOX_SIZE.1);

		if ic.is_mouse_in_area(self.position, size) {
			if ic.is_mouse_left_down() && !self.is_pressed {
				self.is_pressed = true;
			} else if !ic.is_mouse_left_down() && self.is_pressed {
				self.is_pressed = false;
				self.state = !self.state;
			}
		} else if self.is_pressed {
			self.is_pressed = false;
		}
	}
}

impl<T: AsRef<str>> Paintable for CheckBox<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		if self.state {
			rc.rectangle(self.position, BOX_SIZE, (0, 0, 255, 255));
		} else {
			rc.rectangle(self.position, BOX_SIZE, (33, 33, 33, 255));
		}

		if self.is_pressed {
			rc.rectangle(self.position, BOX_SIZE, (0, 0, 0, 50));
		}

		rc.rectangle_outline(self.position, BOX_SIZE, (0, 0, 0, 255));

		rc.default_font().paint(
			&self.name,
			(
				self.position.0 + TEXT_OFFSET,
				self.position.1 + BOX_SIZE.1 / 2,
			),
			(255, 255, 255, 255),
			[FontFlag::CenterVertically],
		);
	}
}

impl<T: AsRef<str>> InvalidateFontMetrics for CheckBox<T> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.font_metrics = font.get_size(self.name.as_ref());
	}
}

impl<T: AsRef<str>> Item for CheckBox<T> {
	fn get_height(&self) -> usize {
		BOX_SIZE.1
	}

	fn invalidate(&mut self, position: (usize, usize), _: (usize, usize)) {
		self.position = position;
	}
}
