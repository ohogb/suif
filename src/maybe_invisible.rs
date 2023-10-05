use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct MaybeInvisible<T: Widget> {
	widget: T,
	is_visible: bool,
}

impl<T: Widget> MaybeInvisible<T> {
	pub fn new(widget: T) -> Self {
		Self {
			widget,
			is_visible: true,
		}
	}

	pub fn set_is_visible(&mut self, is_visible: bool) {
		self.is_visible = is_visible;
	}

	pub fn is_visible(&self) -> bool {
		self.is_visible
	}

	pub fn get_widget(&self) -> &T {
		&self.widget
	}

	pub fn get_widget_mut(&mut self) -> &mut T {
		&mut self.widget
	}
}

impl<T: Widget> Updateable for MaybeInvisible<T> {
	fn update(&mut self, ic: &impl InputContext) {
		if self.is_visible {
			self.widget.update(ic);
		}
	}
}

impl<T: Widget> Paintable for MaybeInvisible<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		if self.is_visible {
			self.widget.paint(rc);
		}
	}
}

impl<T: Widget> InvalidateFontMetrics for MaybeInvisible<T> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.widget.invalidate_font_metrics(font);
	}
}

impl<T: Widget> Widget for MaybeInvisible<T> {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.widget.invalidate(position, size);
	}
}
