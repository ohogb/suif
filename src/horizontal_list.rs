use crate::{
	Font, HorizontalWidgetList, InputContext, InvalidateFontMetrics, Paintable, RenderContext,
	Updateable, Widget,
};

pub struct HorizontalList<T: HorizontalWidgetList> {
	widgets: T,
}

impl<T: HorizontalWidgetList> HorizontalList<T> {
	pub fn new(widgets: T) -> Self {
		Self { widgets }
	}
}

impl<T: HorizontalWidgetList> Updateable for HorizontalList<T> {
	fn update(&mut self, ic: &impl InputContext) {
		self.widgets.update(ic);
	}
}

impl<T: HorizontalWidgetList> Paintable for HorizontalList<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.widgets.paint(rc);
	}
}

impl<T: HorizontalWidgetList> InvalidateFontMetrics for HorizontalList<T> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.widgets.invalidate_font_metrics(font);
	}
}

impl<T: HorizontalWidgetList> Widget for HorizontalList<T> {
	fn invalidate(&mut self, position: (usize, usize), (w, h): (usize, usize)) {
		let each = w / T::size(0);
		self.widgets.invalidate(position, (each, h));
	}
}
