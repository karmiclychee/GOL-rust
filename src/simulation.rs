use world::World;

struct EvoCell {
  x: f32,
  y: f32,
  color: Vec<f32>
}

pub struct Simulation {
  generation: i32,
  pub world: World
}

impl Simulation {
  pub fn new(world: World) -> Simulation {
    let sim = Simulation {
      generation: 0,
      world: world
    }

    sim.init
    sim
  }

  pub fn init(&self) {
    self.world.seed
  }

  pub fn step(&self) {
    self.generation += 1;
    self.world.proliferate();
    self.world.cull();
  }

  pub fn get_coordinates(&self) {
    let m = (1..10).map( |i| i as f32 ).collect::<Vec<f32>>();

    //     @world.current_grid.map do |Evocell|
    for itt in m.iter() {
      let scale = self.world.scale;
      let color = vec![1.0, 0.0, 0.0, 1.0]; //red
      let padding = 0.5;

      let x = itt * scale;
      let y = itt * scale;

      vec![
        EvoCell { x: x + padding,                y: y + padding,               color: color.clone() },
        EvoCell { x: x + scale - (padding*2.0),  y: y + padding,               color: color.clone() },
        EvoCell { x: x + padding,                y: y + scale - (padding*2.0), color: color.clone() },
        EvoCell { x: x + scale - (padding*2.0),  y: y + scale - (padding*2.0), color: color.clone() }
      ];
    }
  }
}
