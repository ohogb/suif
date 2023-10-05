use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct Layer<T: Widget, U: Widget> {
	position: (usize, usize),
	size: (usize, usize),
	bottom: T,
	top: U,
}

impl<T: Widget, U: Widget> Layer<T, U> {
	pub fn new(bottom: T, top: U) -> Self {
		Self {
			position: Default::default(),
			size: Default::default(),
			bottom,
			top,
		}
	}

	pub fn get_first(&self) -> &T {
		&self.bottom
	}

	pub fn get_first_mut(&mut self) -> &mut T {
		&mut self.bottom
	}

	pub fn get_second(&self) -> &U {
		&self.top
	}

	pub fn get_second_mut(&mut self) -> &mut U {
		&mut self.top
	}
}

impl<T: Widget, U: Widget> Updateable for Layer<T, U> {
	fn update(&mut self, ic: &impl InputContext) {
		self.bottom.update(ic);
		self.top.update(ic);
	}
}

impl<T: Widget, U: Widget> Paintable for Layer<T, U> {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.bottom.paint(rc);
		self.top.paint(rc);
	}
}

impl<T: Widget, U: Widget> InvalidateFontMetrics for Layer<T, U> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.bottom.invalidate_font_metrics(font);
		self.top.invalidate_font_metrics(font);
	}
}

impl<T: Widget, U: Widget> Widget for Layer<T, U> {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.position = position;
		self.size = size;

		self.bottom.invalidate(position, size);
		self.top.invalidate(position, size);
	}
}
