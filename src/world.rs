use window::EvoWindow;
use evo_cell::EvoCell;

pub struct World {
  current_grid: Vec<EvoCell>,
  next_grid: Option<Vec<EvoCell>>,
  dimensions: (i32, i32),
  pub scale: i32,
}

impl World {
  pub fn build(lim_x: i32, lim_y: i32, window: &EvoWindow) -> World {
    let current_grid: Vec<EvoCell> = vec![];
    let cell_size = (
      window.width/lim_x,
      window.height/lim_y
    );

    for cell in 0..lim_x as i32 {
      for row in 0..lim_y as i32 {
        let evo_cell = EvoCell::new((row, cell), cell_size);
        current_grid.push(evo_cell);
      }
    }

    World {
      current_grid: current_grid,
      next_grid: None,
      dimensions: (lim_x, lim_y),
      scale: vec![window.width/lim_x, window.height/lim_y].iter().min().unwrap().clone()
    }
  }

  pub fn seed(&self) {
    for x in 0..(dimensions[0] * dimensions[1] * 0.12 as i32) {
      self.get_cell(rand(dimensions[0]), rand(dimensions[1])).vivify
    }
  }

  pub fn proliferate(&self) {
    self.next_grid = self.current_grid.collect::<Vec<Cell>>(
      self.conway(cell)
    )
  }

  pub fn cull(&self) {
    self.current_grid = self.next_grid;
    self.next_grid = Vec::<EvoCell>;
  }

  // private

  // fn get_cell(x: i32, y: i32, generation: str) //generation: "current"
  //   // # this is awful, don't iterate here if you can

  //   the_grid = case generation
  //     when :current then current_grid
  //     when :next then next_grid
  //   end

  //   the_cell = the_grid.select do |cell|
  //     # cell.coordinates[:x] == x % dimensions[0] &&
  //     #   cell.coordinates[:y] == y % dimensions[1]
  //     cell.coordinates[:x] == x && cell.coordinates[:y] == y
  //   end

  //   the_cell[0]
  // end

  // def conway(cell)
  //   living_neighbors = cell.nearest_neighbors.select do |x_y|
  //     (c = get_cell(x_y[0], x_y[1])) && c.alive?
  //   end

  //   case
  //   when cell.alive? && living_neighbors.length < 2
  //     cell.clone.decrepify
  //   when cell.alive? && living_neighbors.length > 3
  //     cell.clone.decrepify
  //   when !cell.alive? && living_neighbors.length == 3
  //     cell.clone.vivify
  //   else
  //     cell.clone
  //   end
  // end
}
