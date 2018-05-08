extern crate piston_window;
use piston_window::*;

mod window;
mod simulation;
mod world;
mod evo_cell;
use window::EvoWindow;

fn main() {
  let evo_window = EvoWindow::new(640);

  let mut window: PistonWindow = WindowSettings::new(
        "Evolve", [evo_window.height as u32, evo_window.width as u32]
      ).exit_on_esc(true).build().unwrap();

  let mut i = 0.0;
  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
      // evo_window.update();
      // evo_window.draw();
      // window.draw_2d(&event, |context, graphics| {
      //     clear([1.0; 4], graphics);
      //     rectangle([1.0, 0.0, i, 1.0], // red
      //               [0.0, 0.0, 100.0, 100.0],
      //               context.transform,
      //               graphics);
      // });
  }
}
