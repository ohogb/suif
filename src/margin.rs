use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct Margin<T: Widget> {
	left: usize,
	top: usize,
	right: usize,
	bottom: usize,
	widget: T,
}

impl<T: Widget> Margin<T> {
	pub fn new(left: usize, top: usize, right: usize, bottom: usize, widget: T) -> Self {
		Self {
			left,
			top,
			right,
			bottom,
			widget,
		}
	}

	pub fn get_widget(&self) -> &T {
		&self.widget
	}

	pub fn get_widget_mut(&mut self) -> &mut T {
		&mut self.widget
	}
}

impl<T: Widget> Updateable for Margin<T> {
	fn update(&mut self, ic: &impl InputContext) {
		self.widget.update(ic);
	}
}

impl<T: Widget> Paintable for Margin<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.widget.paint(rc);
	}
}

impl<T: Widget> InvalidateFontMetrics for Margin<T> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.widget.invalidate_font_metrics(font);
	}
}

impl<T: Widget> Widget for Margin<T> {
	fn invalidate(&mut self, (x, y): (usize, usize), (w, h): (usize, usize)) {
		let x = x + self.left;
		let y = y + self.top;

		let w = w - self.left;
		let h = h - self.top;

		let w = w - self.right;
		let h = h - self.bottom;

		self.widget.invalidate((x, y), (w, h));
	}
}
