use crate::{
	Font, InputContext, InvalidateFontMetrics, Item, Paintable, RenderContext, Updateable, Widget,
};

pub struct Text<T: AsRef<str>> {
	position: (usize, usize),
	text: T,
	color: (u8, u8, u8, u8),
}

impl<T: AsRef<str>> Text<T> {
	pub fn new(text: T, color: (u8, u8, u8, u8)) -> Self {
		Self {
			position: Default::default(),
			text,
			color,
		}
	}
}

impl<T: AsRef<str>> Updateable for Text<T> {
	fn update(&mut self, _: &impl InputContext) {}
}

impl<T: AsRef<str>> Paintable for Text<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		rc.default_font()
			.paint(&self.text, self.position, self.color, []);
	}
}

impl<T: AsRef<str>> InvalidateFontMetrics for Text<T> {
	fn invalidate_font_metrics(&mut self, _: &mut impl Font) {}
}

impl<T: AsRef<str>> Widget for Text<T> {
	fn invalidate(&mut self, position: (usize, usize), _: (usize, usize)) {
		self.position = position;
	}
}

impl<T: AsRef<str>> Item for Text<T> {
	fn get_height(&self) -> usize {
		16
	}

	fn invalidate(&mut self, position: (usize, usize), _: (usize, usize)) {
		self.position = position;
	}
}
