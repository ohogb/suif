use crate::{
	Font, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable, Widget,
};

pub struct MaybeInvisible<T: Widget> {
	widget: T,
	is_visible: std::cell::Cell<bool>,
}

impl<T: Widget> MaybeInvisible<T> {
	pub fn new(widget: T) -> Self {
		Self {
			widget,
			is_visible: std::cell::Cell::new(true),
		}
	}

	pub fn set_is_visible(&self, is_visible: bool) {
		self.is_visible.set(is_visible);
	}

	pub fn is_visible(&self) -> bool {
		self.is_visible.get()
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
		if self.is_visible() {
			self.widget.update(ic);
		}
	}
}

impl<T: Widget> Paintable for MaybeInvisible<T> {
	fn paint(&self, rc: &mut impl RenderContext) {
		if self.is_visible() {
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

pub trait MaybeInvisibleList {
	fn set_is_visible(&self, index: usize, is_visible: bool);
	fn is_visible(&self, index: usize) -> bool;
	fn size(&self, count: usize) -> usize;
}

impl<T: Widget> MaybeInvisibleList for MaybeInvisible<T> {
	fn set_is_visible(&self, _index: usize, is_visible: bool) {
		self.set_is_visible(is_visible);
	}

	fn is_visible(&self, _index: usize) -> bool {
		self.is_visible()
	}

	fn size(&self, count: usize) -> usize {
		count + 1
	}
}

impl<T: MaybeInvisibleList, U: MaybeInvisibleList> MaybeInvisibleList for (&T, U) {
	fn set_is_visible(&self, index: usize, is_visible: bool) {
		if index == 0 {
			self.0.set_is_visible(index, is_visible);
		} else {
			self.1.set_is_visible(index - 1, is_visible);
		}
	}

	fn is_visible(&self, index: usize) -> bool {
		if index == 0 {
			self.0.is_visible(index)
		} else {
			self.1.is_visible(index - 1)
		}
	}

	fn size(&self, count: usize) -> usize {
		self.1.size(count + 1)
	}
}

impl<T: MaybeInvisibleList> MaybeInvisibleList for (&T, ()) {
	fn set_is_visible(&self, index: usize, is_visible: bool) {
		if index == 0 {
			self.0.set_is_visible(index, is_visible);
		} else {
			panic!();
		}
	}

	fn is_visible(&self, index: usize) -> bool {
		if index == 0 {
			self.0.is_visible(index)
		} else {
			panic!();
		}
	}

	fn size(&self, count: usize) -> usize {
		count + 1
	}
}
