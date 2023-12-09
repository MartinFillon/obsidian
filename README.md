# Obsidian

Small and simple graphic library in rust.

## Installation

```shell
cargo install --git https:://github.com/MartinFillon/obsidian
```

## Usage

```rust
fn main() {
    init();
    let mut window = Window::new(800, 600, "Engine Tester", false).unwrap();

    let mut square = Square::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(400.0, 300.0, 0.0),
        Point::new(400.0, -300.0, 0.0),
        Point::new(-400.0, -300.0, 0.0),
        Point::new(-400.0, 300.0, 0.0),
        Some(Colors::red()),
        None,
    );

    while !window.should_close() {
        window.clear(Colors::black());
        square.display();
        let ev = window.update();
        process_events(&mut window, ev);
    }
}

fn process_events(window: &mut Window, events: Vec<WindowEvent>) {
    for event in events {
        match event {
            WindowEvent::FramebufferSize(width, height) => window.resize(width, height),
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
```

This example will display a red square in the middle of the screen.
