use std::env;
use std::io::{self, Write};
use text_to_png::TextRenderer;

fn main() {
    let msg = env::args().nth(1).unwrap_or("Hello, world!".to_string());
    let renderer = TextRenderer::default();

    if let Err(e) = renderer
        .render_text_to_png_data(msg, 64, "Blue")
        .map_err(|err| err.to_string())
        .and_then(|png| {
            let b = png.data;
            io::stdout()
                .write_all(&b[..])
                .map_err(|err| err.to_string())
        })
    {
        println!("failed to write png: {:?}", e);
    }
}
