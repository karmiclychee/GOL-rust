use simulation::Simulation;
use world::World;

pub struct EvoWindow {
  space_key: KeyStroke,
  esc_key: KeyStroke,
  click: KeyStroke,
  simulation: Option<Simulation>,
  is_paused: bool,
  pub height: i32,
  pub width: i32
}

struct KeyStroke(i32);

impl EvoWindow {
  pub fn new(square: i32) -> EvoWindow {

    let window = EvoWindow {
      height: square,
      width: square,
      is_paused: false,
      // font: ,
      simulation: None,
      space_key: KeyStroke(44),
      esc_key: KeyStroke(41),
      click: KeyStroke(256),
    };

    window.new_simulation(square);
    window
  }

  fn new_simulation(&self, square: i32) {
    let world = World::build(20, 20, self);
    let sim = Simulation::new(world);
    self.simulation = Some(sim);
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

  fn update(&self) {
    if self.is_paused() { self.simulation.unwrap().step(); }
  }

  // fn corner_text(&self) -> {
  //   "#{paused? ? "PAUSED" : @simulation.generation} | SPACE to restart, CLICK to pause, ESC to exit"
  // }

  fn draw(&self) {
    // // @font.draw @button_id, 0, 20, 1
    // // @font.draw corner_text, 0, 0, 1
    // self.simulation.get_coordinates.each ( |coordinates_set|
    //   // draw_quad *coordinates_set
    // )
  }
}
