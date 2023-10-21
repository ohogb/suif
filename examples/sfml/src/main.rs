#![allow(incomplete_features)]
#![feature(fn_traits, return_position_impl_trait_in_trait)]

use suif::{Font, RenderContext};
use sfml::graphics::RenderTarget;

mod contexts;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut window = sfml::graphics::RenderWindow::new(
		sfml::window::VideoMode::new(1280, 720, 32),
		"suif sfml example",
		sfml::window::Style::default(),
		&sfml::window::ContextSettings::default(),
	);

	let mut rectangle = sfml::graphics::RectangleShape::new();

	let font =
		sfml::graphics::Font::from_file("/usr/share/fonts/liberation/LiberationMono-Regular.ttf")
			.ok_or("sfml::graphics::Font::from_file() failed")?;

	let mut text = sfml::graphics::Text::new("", &font, 12);
	window.set_vertical_sync_enabled(true);

	let current_tab = std::cell::Cell::new(-1);

	let widget = suif::Layer::new(
		suif::Color::new((0, 0, 0, 255)),
		suif::Layer::new(
			suif::Margin::new(1, 1, 1, 1, suif::Color::new((55, 55, 55, 255))),
			suif::Margin::new(
				10,
				10,
				10,
				10,
				suif::HorizontalSplit::new(
					0.08,
					suif::HorizontalList::new(suif::list![
						suif::Button::new("1st", || {
							current_tab.set(0);
						}),
						suif::Button::new("2nd", || {
							current_tab.set(1);
						}),
					]),
					suif::Margin::new(
						0,
						10,
						0,
						0,
						suif::Layer::new(
							suif::MaybeInvisible::new(suif::GroupBox::new(
								"Asdf",
								suif::list![
									suif::Text::new("first", (255, 0, 0, 255)),
									suif::Text::new("second", (0, 255, 0, 255)),
									suif::Text::new("third", (0, 0, 255, 255)),
									suif::Text::new("fourth", (0, 0, 255, 255)),
									suif::Button::new("Button!", || {
										println!("from button inside group box!");
									}),
									suif::Text::new("After button!", (123, 123, 123, 255)),
									suif::CheckBox::new("Checkbox!")
								],
							)),
							suif::MaybeInvisible::new(suif::GroupBox::new(
								"Second",
								suif::list![
									suif::Text::new("idk", (255, 255, 255, 255)),
									suif::Text::new("idk", (255, 255, 255, 255)),
									suif::Button::new("asdf!", || {
										println!("asdf");
									}),
								],
							)),
						),
					),
				),
			),
		),
	);

	let mut uiw = suif::Window::new((100, 100), (250, 400), widget);
	uiw.invalidate();

	let mut render_context = contexts::Render {
		window: &mut window,
		rectangle: &mut rectangle,
		default_text: &mut text,
	};

	let mut input_context = contexts::Input {
		mouse_position: Default::default(),
	};

	uiw.invalidate_font_metrics(&mut render_context.default_font());

	let mut fps = 0;
	let mut fps_counter = 0;
	let mut fps_timer = std::time::Instant::now();

	while render_context.window.is_open() {
		while let Some(event) = render_context.window.poll_event() {
			match event {
				sfml::window::Event::Closed => render_context.window.close(),
				sfml::window::Event::MouseMoved { x, y } => {
					input_context.mouse_position = (x as _, y as _);
				}
				_ => (),
			}
		}

		render_context
			.window
			.clear(sfml::graphics::Color::rgba(111, 111, 111, 255));

		fps_counter += 1;
		let now = std::time::Instant::now();

		if now.duration_since(fps_timer).as_secs_f64() > 1.0 {
			fps = fps_counter;
			fps_counter = 0;
			fps_timer = now;
		}

		render_context.default_font().paint(
			format!("fps: {}", fps),
			(10, 10),
			(255, 255, 255, 255),
			[],
		);

		let tabs = uiw
			.get_widget_mut()
			.get_second_mut()
			.get_second_mut()
			.get_widget_mut()
			.get_bottom_mut()
			.get_widget_mut();

		match current_tab.get() {
			0 => {
				tabs.get_first_mut().set_is_visible(true);
				tabs.get_second_mut().set_is_visible(false);
			}
			1 => {
				tabs.get_first_mut().set_is_visible(false);
				tabs.get_second_mut().set_is_visible(true);
			}
			_ => {
				tabs.get_first_mut().set_is_visible(false);
				tabs.get_second_mut().set_is_visible(false);
			}
		}

		uiw.update(&input_context);
		uiw.paint(&mut render_context);

		render_context.window.display();
	}

	Ok(())
}
