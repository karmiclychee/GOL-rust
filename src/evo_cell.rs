pub struct EvoCell {
  coordinates: Cartesian,
  dimensions: Cartesian
}

struct Cartesian(i32, i32);

impl EvoCell {
  pub fn new(coords: (i32, i32), dimensions: (i32, i32)) -> EvoCell {

    EvoCell {
      coordinates: Cartesian(coords.0, coords.1),
      dimensions: Cartesian(dimensions.0, dimensions.1),
      //     @skin = DEAD
      //     @alive = false
    }
  }
}

// class Evolve::Cell
//   attr_reader :coordinates, :dimensions, :skin
//   DEAD = Gosu::Color::BLACK
//   ALIVE = Gosu::Color::GREEN

//   def initialize(coordinates:, dimensions:)
//     @coordinates = {
//       x: coordinates[0],
//       y: coordinates[1]
//     }

//     @dimensions = {
//       x: dimensions[0],
//       y: dimensions[1]
//     }

//     @skin = DEAD
//     @alive = false
//   end

//   def vivify
//     @skin = ALIVE
//     @alive = true
//     self
//   end

//   def decrepify
//     @skin = DEAD
//     @alive = false
//     self
//   end

//   def nearest_neighbors
//     [
//       [ coordinates[:x] - 1,  coordinates[:y] - 1 ],
//       [ coordinates[:x],      coordinates[:y] - 1 ],
//       [ coordinates[:x] + 1,  coordinates[:y] - 1 ],
//       [ coordinates[:x] - 1,  coordinates[:y]     ],
//       [ coordinates[:x] + 1,  coordinates[:y]     ],
//       [ coordinates[:x] - 1,  coordinates[:y] + 1 ],
//       [ coordinates[:x],      coordinates[:y] + 1 ],
//       [ coordinates[:x] + 1,  coordinates[:y] + 1 ]
//     ]
//   end

//   def alive?
//     @alive
//   end
// end
