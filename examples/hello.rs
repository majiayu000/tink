//! Hello World example
//!
//! Run with: cargo run --example hello
//! Press Ctrl+C to exit

use tink::prelude::*;
use tink::renderer::App;

fn main() -> std::io::Result<()> {
    // Use inline mode (not alternate screen) so output stays visible
    let options = tink::renderer::AppOptions {
        alternate_screen: false,
        ..Default::default()
    };

    App::with_options(app, options).run()
}

fn app() -> Element {
    Box::new()
        .padding(1.0)
        .border_style(BorderStyle::Round)
        .border_color(Color::Cyan)
        .child(
            Text::new("Hello, Tink!")
                .color(Color::Green)
                .bold()
                .into_element()
        )
        .into_element()
}
