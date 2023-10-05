use crate::{Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable};

impl<T: Updateable, U: Updateable> Updateable for (T, U) {
	fn update(&mut self, ic: &impl InputContext) {
		self.0.update(ic);
		self.1.update(ic);
	}
}

impl<T: Updateable> Updateable for (T, ()) {
	fn update(&mut self, ic: &impl InputContext) {
		self.0.update(ic);
	}
}

impl<T: Paintable, U: Paintable> Paintable for (T, U) {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.0.paint(rc);
		self.1.paint(rc);
	}
}

impl<T: Paintable> Paintable for (T, ()) {
	fn paint(&self, rc: &mut impl RenderContext) {
		self.0.paint(rc);
	}
}

impl<T: InvalidateFontMetrics, U: InvalidateFontMetrics> InvalidateFontMetrics for (T, U) {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.0.invalidate_font_metrics(font);
		self.1.invalidate_font_metrics(font);
	}
}

impl<T: InvalidateFontMetrics> InvalidateFontMetrics for (T, ()) {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.0.invalidate_font_metrics(font);
	}
}

#[macro_export]
macro_rules! list {
	($x:expr) => {
		($x, ())
	};
	($x:expr, $($y:expr),* $(,)?) => {
		($x, $crate::list![$($y),*])
	};
}
