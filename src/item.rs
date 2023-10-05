use crate::{InvalidateFontMetrics, Paintable, Updateable};

pub trait Item: Updateable + Paintable + InvalidateFontMetrics {
	fn get_height(&self) -> usize;
	fn invalidate(&mut self, position: (usize, usize), area: (usize, usize));
}

pub trait VerticalItemList: Updateable + Paintable + InvalidateFontMetrics {
	fn invalidate(
		&mut self,
		position: (usize, usize),
		area: (usize, usize),
		space_in_between: usize,
	);
}

impl<T: Item, U: VerticalItemList> VerticalItemList for (T, U) {
	fn invalidate(
		&mut self,
		position: (usize, usize),
		area: (usize, usize),
		space_in_between: usize,
	) {
		self.0.invalidate(position, area);

		let (x, y) = position;
		let h = self.0.get_height();

		let y = y + h + space_in_between;
		self.1.invalidate((x, y), area, space_in_between);
	}
}

impl<T: Item> VerticalItemList for (T, ()) {
	fn invalidate(&mut self, position: (usize, usize), area: (usize, usize), _: usize) {
		self.0.invalidate(position, area);
	}
}
