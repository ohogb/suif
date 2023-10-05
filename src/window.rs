use crate::{Font, InputContext, RenderContext, Widget};

pub struct Window<T: Widget> {
	position: (usize, usize),
	size: (usize, usize),
	widget: T,
}

impl<T: Widget> Window<T> {
	pub fn new(position: (usize, usize), size: (usize, usize), widget: T) -> Self {
		Self {
			position,
			size,
			widget,
		}
	}

	pub fn update(&mut self, ic: &impl InputContext) {
		self.widget.update(ic);
	}

	pub fn paint(&self, rc: &mut impl RenderContext) {
		self.widget.paint(rc);
	}

	pub fn invalidate(&mut self) {
		self.widget.invalidate(self.position, self.size);
	}

	pub fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.widget.invalidate_font_metrics(font);
	}

	pub fn set_position(&mut self, position: (usize, usize)) {
		self.position = position;
	}

	pub fn get_widget(&self) -> &T {
		&self.widget
	}

	pub fn get_widget_mut(&mut self) -> &mut T {
		&mut self.widget
	}
}
