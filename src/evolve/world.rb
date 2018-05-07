require_relative 'cell'

class Evolve::World
  def self.build(lim_x, lim_y, window)
    current_grid = []

    cell_size = [window.width/lim_x, window.height/lim_y]

    lim_x.times do |cell|
      lim_y.times { |row| current_grid << Evolve::Cell.new(
        coordinates: [row, cell], dimensions: cell_size) }
    end

    #TODO 2D grid, init cells only when alive

    new current_grid, [lim_x, lim_y], [window.width/lim_x, window.height/lim_y].min
  end

  def seed
    (dimensions[0] * dimensions[1] * 0.12).to_i.times do |x|
      get_cell(rand(dimensions[0]), rand(dimensions[1])).vivify
    end
  end

  def proliferate
    @next_grid = @current_grid.map do |cell|
      conway(cell)
    end
  end

  def cull
    @current_grid = @next_grid
    @next_grid = []
  end

  attr_reader :current_grid, :next_grid, :dimensions, :scale

  private

  def initialize(current_grid, dimensions, scale)
    @current_grid = current_grid
    @next_grid = []
    @dimensions = dimensions
    @scale = scale
  end

  def get_cell(x, y, generation=:current)
    # this is awful, don't iterate here if you can
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
end
