#[derive(PartialEq)]
pub enum FontFlag {
	CenterHorizontally,
	CenterVertically,
}

pub trait Font {
	fn paint<const N: usize>(
		&mut self,
		text: impl AsRef<str>,
		position: (usize, usize),
		color: (u8, u8, u8, u8),
		flags: [FontFlag; N],
	);

	fn get_size(&mut self, text: impl AsRef<str>) -> (usize, usize);
}

pub trait RenderContext {
	fn rectangle(
		&mut self,
		position: (usize, usize),
		size: (usize, usize),
		color: (u8, u8, u8, u8),
	);

	fn rectangle_outline(
		&mut self,
		position: (usize, usize),
		size: (usize, usize),
		color: (u8, u8, u8, u8),
	);

	fn default_font(&mut self) -> impl Font + '_;
}
