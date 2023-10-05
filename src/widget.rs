use crate::{Font, InputContext, RenderContext};

pub trait Updateable {
	fn update(&mut self, ic: &impl InputContext);
}

pub trait Paintable {
	fn paint(&self, rc: &mut impl RenderContext);
}

pub trait InvalidateFontMetrics {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font);
}

pub trait Widget: Updateable + Paintable + InvalidateFontMetrics {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize));
}

pub trait HorizontalWidgetList: Updateable + Paintable + InvalidateFontMetrics {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize));
	fn size(count: usize) -> usize;
}

impl<T: Widget, U: HorizontalWidgetList> HorizontalWidgetList for (T, U) {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.0.invalidate(position, size);
		self.1.invalidate((position.0 + size.0, position.1), size);
	}

	fn size(count: usize) -> usize {
		U::size(count) + 1
	}
}

impl<T: Widget> HorizontalWidgetList for (T, ()) {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.0.invalidate(position, size);
	}

	fn size(count: usize) -> usize {
		count + 1
	}
}
