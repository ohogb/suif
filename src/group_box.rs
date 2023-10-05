use crate::{
	Font, FontFlag, InputContext, InvalidateFontMetrics, Paintable, RenderContext, Updateable,
	VerticalItemList, Widget,
};

pub struct GroupBox<T: AsRef<str>, U: VerticalItemList> {
	position: (usize, usize),
	size: (usize, usize),
	name: T,
	items: U,
	font_metrics: (usize, usize),
}

impl<T: AsRef<str>, U: VerticalItemList> GroupBox<T, U> {
	pub fn new(name: T, items: U) -> Self {
		Self {
			position: Default::default(),
			size: Default::default(),
			name,
			items,
			font_metrics: Default::default(),
		}
	}
}

impl<T: AsRef<str>, U: VerticalItemList> Updateable for GroupBox<T, U> {
	fn update(&mut self, ic: &impl InputContext) {
		self.items.update(ic);
	}
}

impl<T: AsRef<str>, U: VerticalItemList> Paintable for GroupBox<T, U> {
	fn paint(&self, rc: &mut impl RenderContext) {
		let (x, y) = self.position;

		rc.rectangle(self.position, self.size, (22, 22, 22, 255));
		rc.rectangle_outline(self.position, self.size, (0, 0, 0, 255));

		rc.rectangle((x + 10, y), (self.font_metrics.0, 2), (22, 22, 22, 255));

		// TODO: center
		rc.default_font().paint(
			&self.name,
			(x + 10, y),
			(255, 255, 255, 255),
			[FontFlag::CenterVertically],
		);

		self.items.paint(rc);
	}
}

impl<T: AsRef<str>, U: VerticalItemList> InvalidateFontMetrics for GroupBox<T, U> {
	fn invalidate_font_metrics(&mut self, font: &mut impl Font) {
		self.font_metrics = font.get_size(&self.name);
		self.items.invalidate_font_metrics(font);
	}
}

impl<T: AsRef<str>, U: VerticalItemList> Widget for GroupBox<T, U> {
	fn invalidate(&mut self, position: (usize, usize), size: (usize, usize)) {
		self.position = position;
		self.size = size;

		let (x, y) = self.position;
		let (x, y) = (x + 20, y + 20);

		let (w, h) = self.size;
		let (w, h) = (w - 20 * 2, h - 20 * 2);

		self.items.invalidate((x, y), (w, h), 10);
	}
}
