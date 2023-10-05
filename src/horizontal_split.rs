use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct HorizontalSplit<T: Widget, U: Widget> {
	percent: f64,
	top: T,
	bottom: U,
}

impl<T: Widget, U: Widget> HorizontalSplit<T, U> {
	pub fn new(percent: f64, top: T, bottom: U) -> Self {
		Self {
			percent,
			top,
			bottom,
		}
	}

	pub fn get_top(&self) -> &T {
		&self.top
	}

	pub fn get_top_mut(&mut self) -> &mut T {
		&mut self.top
	}

	pub fn get_bottom(&self) -> &U {
		&self.bottom
	}

	pub fn get_bottom_mut(&mut self) -> &mut U {
		&mut self.bottom
	}
}

impl<T: Widget, U: Widget> Updateable for HorizontalSplit<T, U> {
	fn update(&mut self, ic: &impl InputContext) {
		self.top.update(ic);
		self.bottom.update(ic);
	}
}

impl<T: Widget, U: Widget> Paintable for HorizontalSplit<T, U> {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.top.paint(rc);
		self.bottom.paint(rc);
	}
}

impl<T: Widget, U: Widget> InvalidateFontMetrics for HorizontalSplit<T, U> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.top.invalidate_font_metrics(font);
		self.bottom.invalidate_font_metrics(font);
	}
}

impl<T: Widget, U: Widget> Widget for HorizontalSplit<T, U> {
	fn invalidate(&mut self, (x, y): (usize, usize), (w, h): (usize, usize)) {
		self.top
			.invalidate((x, y), (w, (h as f64 * self.percent) as _));

		self.bottom.invalidate(
			(x, y + (h as f64 * self.percent) as usize),
			(w, (h as f64 * (1.0 - self.percent)) as usize),
		);
	}
}
