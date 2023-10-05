use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

#[derive(Debug)]
pub struct Color {
	position: (usize, usize),
	size: (usize, usize),
	color: (u8, u8, u8, u8),
}

impl Color {
	pub fn new(color: (u8, u8, u8, u8)) -> Self {
		Self {
			position: Default::default(),
			size: Default::default(),
			color,
		}
	}
}

impl Updateable for Color {
	fn update(&mut self, _: &impl InputContext) {}
}

impl Paintable for Color {
	fn paint(&self, rc: &mut impl RenderContext) {
		rc.rectangle(self.position, self.size, self.color);
	}
}

impl InvalidateFontMetrics for Color {
	fn invalidate_font_metrics(&mut self, _: &mut impl Font) {}
}

impl Widget for Color {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.position = position;
		self.size = size;
	}
}
