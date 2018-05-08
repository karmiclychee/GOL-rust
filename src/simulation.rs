use world::World;
use evo_cell::EvoCell;

pub struct Simulation {
  generation: i32,
  pub world: World
}

impl Simulation {
  pub fn new(world: World) -> Simulation {
    let sim = Simulation {
      generation: 0,
      world: world
    };

    sim.init();
    sim
  }

  pub fn init(&self) {
    self.world.seed();
  }

  pub fn step(&self) {
    self.generation += 1;
    self.world.proliferate();
    self.world.cull();
  }

  pub fn get_coordinates(&self) {
    let m = (1..10).map( |i| i as i32 ).collect::<Vec<i32>>();

    //     @world.current_grid.map do |Evocell|
    for itt in m.iter() {
      let scale = self.world.scale;
      let color = vec![1.0, 0.0, 0.0, 1.0]; //red
      let padding = 1;

      let x = itt * scale;
      let y = itt * scale;

      vec![
        [x + padding,                y + padding],                //color: color.clone() },
        [x + scale - (padding*2),  y + padding],                //color: color.clone() },
        [x + padding,                y + scale - (padding*2)],  //color: color.clone() },
        [x + scale - (padding*2),  y + scale - (padding*2)]  //color: color.clone() }
      ];
    }
  }
}
