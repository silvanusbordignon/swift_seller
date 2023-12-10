use robotics_lib::interface::{ Tools, robot_view };
use robotics_lib::world::World;
use robotics_lib::runner::Runnable;
use robotics_lib::world::tile::Content;

struct ProfitCraftor;
impl Tools for ProfitCraftor {}

impl ProfitCraftor {
    /// Returns whether the robot is near a market (no diagonals)
    /// DocString not complete as this is not the final version of the method
    pub fn profit_craftor(robot: &impl Runnable, world: &World) -> bool {

        let robot_view= robot_view(robot, &world);
        for (i, row) in robot_view.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match (i, j) {
                    (0, 1) | (1, 0) | (1, 2) | (2, 1) => {
                        match col {
                            | None => (),
                            | Some(tile) => {
                                match tile.content {
                                    Content::Market(_) => return true,
                                    _ => ()
                                }
                            }
                        }
                    },
                    _ => ()
                }
            }
        }

        false
    }

}


#[cfg(test)]
mod tests {
    use std::process::exit;

    use robotics_lib::energy::Energy;
    use robotics_lib::event::events::Event;

    use robotics_lib::interface::{Direction, go};

    use robotics_lib::runner::backpack::BackPack;
    use robotics_lib::runner::{Robot, Runner};

    use robotics_lib::world::coordinates::Coordinate;
    use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
    use robotics_lib::world::tile::{Tile, TileType};
    use robotics_lib::world::world_generator::Generator;

    use super::*;

    /**************************************************************************
    *  MAP:
    *    ______________________________________
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |    0 el    |
    *   |    None    |  Tree(1)   |    None    |
    *   |____________|____________|____________|
    *   |            |            |            |
    *   |    Grass   |   Grass    |   Grass    |
    *   |    0 el    |   0 el     |    0 el    |
    *   |    None    | Market(1)  |    None    |
    *   |____________|____________|____________|
    *   |            |            |            |
    *   |    Grass   |    Grass   |   Grass    |
    *   |    0 el    |    0 el    |    0 el    |
    *   |    None    |    None    |  Tree(2)   |
    *   |____________|____________|____________|
    *
    *   TEST:
    *
    *   Starting from (0,0), the robot will go around the (1,1) tile clockwise and call the
    *   profit_craftor() function. The Market should be detected four times only, and the call
    *   should fail every time the robot is in a corner.
    *
    *   Copyright: comment format courtesy of the common crate
    */
    #[test]
    fn detect_market() {

        // World generator

        struct MarketWorld {}
        impl MarketWorld {
            fn new() -> Self {
                MarketWorld {}
            }
        }
        impl Generator for MarketWorld {
            fn gen(&mut self) -> robotics_lib::world::world_generator::World {
                let mut map: Vec<Vec<Tile>> = Vec::new();

                map.push(Vec::new());
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(1),
                    elevation: 0,
                });
                map[0].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });

                map.push(Vec::new());
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Market(1),
                    elevation: 0,
                });
                map[1].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });

                map.push(Vec::new());
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                });
                map[2].push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::Tree(2),
                    elevation: 0,
                });

                let environmental_conditions = EnvironmentalConditions::new(&[WeatherType::Sunny], 15, 12);
                (map, (0, 0), environmental_conditions.unwrap(), 100.0, None)
            }
        }

        let mut generator: MarketWorld = MarketWorld::new();

        // Robot

        struct MyRobot(Robot);

        impl Runnable for MyRobot {
            fn process_tick(&mut self, world: &mut World) {

                // List all the movements I intend to make and the outcomes of the function call
                // after the corresponding movement
                let movements:&[(Direction, bool)] = &[
                    (Direction::Right, true), (Direction::Right, false),
                    (Direction::Down, true), (Direction::Down, false),
                    (Direction::Left, true), (Direction::Left, false),
                    (Direction::Up, true), (Direction::Up, false)
                ];

                // For each movement, perform the following actions
                for (movement, outcome) in movements {
                    // Since I created a world ad hoc, those movements should be possible
                    go(self, world, movement.clone()).expect("CANNOT MOVE");

                    // Call the function
                    assert_eq!(ProfitCraftor::profit_craftor(self, &world), *outcome);
                }
            }

            fn handle_event(&mut self, event: Event) {
                match event {
                    | Event::Terminated => {}
                    | _ => {}
                }
            }

            fn get_energy(&self) -> &Energy {
                &self.0.energy
            }
            fn get_energy_mut(&mut self) -> &mut Energy {
                &mut self.0.energy
            }
            fn get_coordinate(&self) -> &Coordinate {
                &self.0.coordinate
            }
            fn get_coordinate_mut(&mut self) -> &mut Coordinate {
                &mut self.0.coordinate
            }
            fn get_backpack(&self) -> &BackPack {
                &self.0.backpack
            }
            fn get_backpack_mut(&mut self) -> &mut BackPack { &mut self.0.backpack }
        }

        // Instance the robot and the world

        let my_robot = MyRobot(Robot::new());
        let run = Runner::new(Box::new(my_robot), &mut generator);

        // Since the weather is sunny day, the robot is walking on grass, and it starts with full
        // energy, I can walk around the Market content all in one tick
        match run {
            | Ok(mut r) => {
                let _ = r.game_tick();
            }
            | Err(e) => {
                println!("{:?}", e);
                exit(1)
            },
        }
    }
}
