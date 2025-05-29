mod tmachine;

use raylib::prelude::*;

fn main() {
  let (mut rl, thread) = raylib::init().title("Turing Cogs").vsync().build();

  while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);

    d.clear_background(Color::WHITE);
    d.draw_text(
      "Hello world",
      (d.get_render_width() - d.measure_text("Hello world", 24)) / 2,
      d.get_render_height() / 2,
      24,
      Color::BLACK,
    );
  }
}
