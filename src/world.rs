use window::Window;
use cell::Cell;

pub struct World {
  current_grid: Vec,
  dimensions: Vec,
  scale: i32,
  next_grid: Vec,
}

impl World {
  pub fn build(f32: lim_x, f32: lim_y, window: Window) -> World {
    let current_grid = vec!;
    let cell_size = [window.width/lim_x, window.height/lim_y]

    for cell in (0..lim_x) {
      for row in (0..lim_y) {
        current_grid.push(Cell.new(coordinates: [row, cell], dimensions: cell_size))
      }
    }

    World {
      current_grid: current_grid,
      dimensions: [lim_x, lim_y],
      scale: [window.width/lim_x, window.height/lim_y].min
    }
  }

  pub fn seed(&self) -> {
    (dimensions[0] * dimensions[1] * 0.12).to_i.times do |x|
      get_cell(rand(dimensions[0]), rand(dimensions[1])).vivify
    end
  }

  pub fn proliferate(&self) -> {
    self.next_grid = self.current_grid.map do |cell|
      conway(cell)
    end
  }

  pub fn cull(&self) -> {
    self.current_grid = self.next_grid
    self.next_grid = []
  }

  private

  fn get_cell(x: i32, y: i32, generation: str) //generation: "current"
    // # this is awful, don't iterate here if you can

    the_grid = case generation
      when :current then current_grid
      when :next then next_grid
    end

    the_cell = the_grid.select do |cell|
      # cell.coordinates[:x] == x % dimensions[0] &&
      #   cell.coordinates[:y] == y % dimensions[1]
      cell.coordinates[:x] == x && cell.coordinates[:y] == y
    end

    the_cell[0]
  end

  def conway(cell)
    living_neighbors = cell.nearest_neighbors.select do |x_y|
      (c = get_cell(x_y[0], x_y[1])) && c.alive?
    end

    case
    when cell.alive? && living_neighbors.length < 2
      cell.clone.decrepify
    when cell.alive? && living_neighbors.length > 3
      cell.clone.decrepify
    when !cell.alive? && living_neighbors.length == 3
      cell.clone.vivify
    else
      cell.clone
    end
  end
}
