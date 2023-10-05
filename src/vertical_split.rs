use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct VerticalSplit<T: Widget, U: Widget> {
	percent: f64,
	left: T,
	right: U,
}

impl<T: Widget, U: Widget> VerticalSplit<T, U> {
	pub fn new(percent: f64, left: T, right: U) -> Self {
		Self {
			percent,
			left,
			right,
		}
	}

	pub fn get_left(&self) -> &T {
		&self.left
	}

	pub fn get_left_mut(&mut self) -> &mut T {
		&mut self.left
	}

	pub fn get_right(&self) -> &U {
		&self.right
	}

	pub fn get_right_mut(&mut self) -> &mut U {
		&mut self.right
	}
}

impl<T: Widget, U: Widget> Updateable for VerticalSplit<T, U> {
	fn update(&mut self, ic: &impl InputContext) {
		self.left.update(ic);
		self.right.update(ic);
	}
}

impl<T: Widget, U: Widget> Paintable for VerticalSplit<T, U> {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.left.paint(rc);
		self.right.paint(rc);
	}
}

impl<T: Widget, U: Widget> InvalidateFontMetrics for VerticalSplit<T, U> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.left.invalidate_font_metrics(font);
		self.right.invalidate_font_metrics(font);
	}
}

impl<T: Widget, U: Widget> Widget for VerticalSplit<T, U> {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		let (x, y) = position;
		let (w, h) = size;

		self.left
			.invalidate(position, ((w as f64 * self.percent) as usize, h));
		self.right.invalidate(
			(x + (w as f64 * self.percent) as usize, y),
			((w as f64 * (1.0 - self.percent)) as usize, h),
		);
	}
}
