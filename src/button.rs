use crate::{
	Font, FontFlag, InputContext, InvalidateFontMetrics, Item, Paintable, RenderContext,
	Updateable, Widget,
};

pub struct Button<T: AsRef<str>, U: Fn()> {
	position: (usize, usize),
	size: (usize, usize),
	is_pressed: bool,
	name: T,
	callback: U,
}

impl<T: AsRef<str>, U: Fn()> Button<T, U> {
	pub fn new(name: T, callback: U) -> Self {
		Self {
			position: Default::default(),
			size: Default::default(),
			is_pressed: Default::default(),
			name,
			callback,
		}
	}
}

impl<T: AsRef<str>, U: Fn()> Updateable for Button<T, U> {
	fn update(&mut self, ic: &impl InputContext) {
		let iia = ic.is_mouse_in_area(self.position, self.size);
		let ild = ic.is_mouse_left_down();

		if iia {
			if ild && !self.is_pressed {
				self.is_pressed = true;
			} else if !ild && self.is_pressed {
				self.is_pressed = false;
				self.callback.call(());
			}
		} else if self.is_pressed {
			self.is_pressed = false;
		}
	}
}

impl<T: AsRef<str>, U: Fn()> Paintable for Button<T, U> {
	fn paint(&self, rc: &mut impl RenderContext) {
		rc.rectangle(self.position, self.size, (33, 33, 33, 255));
		rc.rectangle_outline(self.position, self.size, (0, 0, 0, 255));

		let (mut x, mut y) = self.position;
		let (w, h) = self.size;

		x += w / 2;
		y += h / 2;

		rc.default_font().paint(
			&self.name,
			(x, y),
			(255, 255, 255, 255),
			[FontFlag::CenterVertically, FontFlag::CenterHorizontally],
		);
	}
}

impl<T: AsRef<str>, U: Fn()> InvalidateFontMetrics for Button<T, U> {
	fn invalidate_font_metrics(&mut self, _: &mut impl Font) {}
}

impl<T: AsRef<str>, U: Fn()> Widget for Button<T, U> {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.position = position;
		self.size = size;
	}
}

impl<T: AsRef<str>, U: Fn()> Item for Button<T, U> {
	fn get_height(&self) -> usize {
		self.size.1
	}

	fn invalidate(&mut self, position: (usize, usize), (w, _): (usize, usize)) {
		self.position = position;
		self.size = (w, 40);
	}
}
