use simulation::Simulation;
use world::World;

pub struct Window {
  space_key: KeyStroke,
  esc_key: KeyStroke,
  click: KeyStroke,
  simulation: Simulation,
  pub height: u32,
  pub width: u32
}

struct KeyStroke(i32);

impl Window {
  pub fn new(square: u32) -> Window {

    let window = Window {
      height: square,
      width: square
      // is_paused: bool,
      // font: ,
      space_key: KeyStroke(44),
      esc_key: KeyStroke(41),
      click: KeyStroke(256),
    };

    window.new_simulation
  }

  fn new_simulation(&self, square: u32) -> Simulation {
    self.simulation = Simulation.new(
      World.new(20, 20, self)
    )
  }

  fn is_paused(&self) -> bool {
    self.is_paused
  }

  // def button_down(id)
  //   case id
  //   when CLICK then @pause = !@pause
  //   when ESC_KEY then close
  //   when SPACE_KEY then new_simulation
  //   end
  //   @button_id = id
  // end

  fn update(&self) -> {
    self.simulation.step unless paused?
  }

  // fn corner_text(&self) -> {
  //   "#{paused? ? "PAUSED" : @simulation.generation} | SPACE to restart, CLICK to pause, ESC to exit"
  // }

  fn draw(&self) -> {
    // @font.draw @button_id, 0, 20, 1
    // @font.draw corner_text, 0, 0, 1
    self.simulation.get_coordinates.each ( |coordinates_set|
      // draw_quad *coordinates_set
    )
  }
}
