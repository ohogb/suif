pub trait InputContext {
	fn get_mouse_position(&self) -> (usize, usize);
	fn is_mouse_in_area(&self, position: (usize, usize), size: (usize, usize)) -> bool;
	fn is_mouse_left_down(&self) -> bool;
}
