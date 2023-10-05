use sfml::graphics::{RenderTarget, Shape, Transformable};

struct SfmlFont<'a, 'b, 'c> {
	ctx: *mut Render<'a, 'b, 'c>,
	_phantom_a: std::marker::PhantomData<&'a ()>,
	_phantom_b: std::marker::PhantomData<&'b ()>,
	_phantom_c: std::marker::PhantomData<&'c ()>,
}

impl<'a, 'b, 'c> suif::Font for SfmlFont<'a, 'b, 'c> {
	fn paint<const N: usize>(
		&mut self,
		text: impl AsRef<str>,
		(mut x, mut y): (usize, usize),
		(r, g, b, a): (u8, u8, u8, u8),
		flags: [suif::FontFlag; N],
	) {
		if !flags.is_empty() {
			let (w, h) = self.get_size(&text);

			if flags.contains(&suif::FontFlag::CenterHorizontally) {
				x -= w / 2;
			}

			if flags.contains(&suif::FontFlag::CenterVertically) {
				y -= h / 2;
			}
		}

		unsafe {
			(*self.ctx).default_text.set_string(text.as_ref());

			(*self.ctx)
				.default_text
				.set_fill_color(sfml::graphics::Color::rgba(r, g, b, a));

			(*self.ctx)
				.default_text
				.set_position(sfml::system::Vector2f::new(x as _, y as _));

			(*self.ctx).window.draw((*self.ctx).default_text);
		}
	}

	fn get_size(&mut self, text: impl AsRef<str>) -> (usize, usize) {
		let bounds = unsafe {
			(*self.ctx).default_text.set_string(text.as_ref());
			(*self.ctx).default_text.local_bounds()
		};

		(
			(bounds.left + bounds.width).round() as usize,
			(bounds.top + bounds.height).round() as usize + 4,
		)
	}
}

pub struct Render<'a, 'b, 'c> {
	pub window: &'a mut sfml::graphics::RenderWindow,
	pub rectangle: &'a mut sfml::graphics::RectangleShape<'b>,
	pub default_text: &'a mut sfml::graphics::Text<'c>,
}

impl<'a, 'b, 'c> suif::RenderContext for Render<'a, 'b, 'c> {
	fn rectangle(
		&mut self,
		(x, y): (usize, usize),
		(w, h): (usize, usize),
		(r, g, b, a): (u8, u8, u8, u8),
	) {
		self.rectangle
			.set_position(sfml::system::Vector2f::new(x as _, y as _));

		self.rectangle
			.set_size(sfml::system::Vector2f::new(w as _, h as _));

		self.rectangle
			.set_fill_color(sfml::graphics::Color::rgba(r, g, b, a));

		self.window.draw(self.rectangle);
	}

	fn rectangle_outline(
		&mut self,
		(x, y): (usize, usize),
		(w, h): (usize, usize),
		color: (u8, u8, u8, u8),
	) {
		self.rectangle((x, y), (w, 1), color);
		self.rectangle((x + w - 1, y), (1, h), color);
		self.rectangle((x, y), (1, h), color);
		self.rectangle((x, y + h - 1), (w, 1), color);
	}

	fn default_font(&mut self) -> impl suif::Font + '_ {
		SfmlFont {
			ctx: self as _,
			_phantom_a: std::marker::PhantomData,
			_phantom_b: std::marker::PhantomData,
			_phantom_c: std::marker::PhantomData,
		}
	}
}

pub struct Input {
	pub mouse_position: (usize, usize),
}

impl suif::InputContext for Input {
	fn get_mouse_position(&self) -> (usize, usize) {
		self.mouse_position
	}

	fn is_mouse_in_area(&self, (x, y): (usize, usize), (w, h): (usize, usize)) -> bool {
		let (mx, my) = self.get_mouse_position();
		mx >= x && my >= y && mx <= x + w && my <= y + h
	}

	fn is_mouse_left_down(&self) -> bool {
		sfml::window::mouse::Button::Left.is_pressed()
	}
}
