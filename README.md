# suif

A static compile-time UI library

## How?

Each `Widget` has a generic parameter for its children

```rust
struct VerticalSplit<T: Widget, U: Widget> {
    percent: f64,
    left: T,
    right: U,
}
```

Creating a `Widget`:

```rust
let widget = suif::VerticalSplit::new(
    0.50,
    suif::Button::new("left", || {
        println!("left");
    }),
    suif::Button::new("right", || {
        println!("right");
    }),
);
```

A nested `Widget`:

```rust
let widget = suif::VerticalSplit::new(
    0.50,
    suif::Button::new("left", || {
        println!("left");
    }),
    suif::HorizontalSplit::new(
        0.50,
        suif::Button::new("right up", || {
            println!("right up");
        }),
        suif::GroupBox::new(
            "group of items",
            suif::list![
                suif::Text::new("first", (255, 0, 0, 255)),
                suif::CheckBox::new("second"),
            ],
        ),
    ),
);
```

Which results in a type like this:

```rust
let widget: VerticalSplit<
    Button<&str, impl Fn()>,
    HorizontalSplit<
        Button<&str, impl Fn()>,
        GroupBox<
            &str,
            (Text<&str>, (CheckBox<&str>, ()))
        >
    >
>
```

Which means the full structure of the widget is known at compile time, allowing
for insane optimizations, and zero runtime indirection, resulting very low
overhead over the renderer

## Renderer

A renderer isn't provided, thus traits `suif::RenderContext` and
`suif::InputContext` have to be implemented

## Examples

Under [examples/](https://github.com/ohogb/suif/tree/master/examples) there is
an implementation with SFML as the renderer. It also provides a more realistic
example of an UI, with tabs, etc...
