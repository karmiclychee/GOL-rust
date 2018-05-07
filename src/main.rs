extern crate piston_window;
use piston_window::*;

mod window;
mod simulation;
mod world;

fn main() {
  let evo_window  = new_evo_window();

  let mut window: PistonWindow = WindowSettings::new(
        "Evolve", [evo_window.height, evo_window.width]
      ).exit_on_esc(true).build().unwrap();

  while let Some(event) = window.next() {
      window.draw_2d(&event, |context, graphics| {
          clear([1.0; 4], graphics);
          rectangle([1.0, 0.0, 0.0, 1.0], // red
                    [0.0, 0.0, 100.0, 100.0],
                    context.transform,
                    graphics);
      });
  }
}

fn new_evo_window() -> window::Window {
  let window = window::Window::new(640);
  let world = world::World::new(40, 40, &window);
  let simulation = simulation::Simulation::new(world);
  window
}




