use std::collections::HashMap;
use robotics_lib::interface::{Tools, robot_view, put, Direction};
use robotics_lib::world::World;
use robotics_lib::runner::Runnable;
use robotics_lib::utils::LibError;
use robotics_lib::world::tile::Content;

pub struct SwiftSeller;
impl Tools for SwiftSeller {}

impl SwiftSeller {
    /// Sell at a Market every item in the robot's backpack
    ///
    /// # Usage
    /// ```rust
    /// use swift_seller::SwiftSeller;
    /// ```
    ///
    /// # Arguments
    /// - `robot`: The robot
    /// - `world`: The world in which the robot is
    ///
    /// # Returns
    /// - `HashMap<Content, usize>`: The items sold at the Market and their quantity
    /// - `LibError`: The error that occurred
    ///
    /// # Errors
    /// - `OperationNotAllowed`: The robot is not near a tile with a Market on it
    /// - `NotEnoughSpace`: The robot doesn't have enough space for earned coins
    ///
    /// # Notes
    /// - does not support multi-threading
    pub fn swift_seller(
        robot: &mut impl Runnable,
        world: &mut World
    ) -> Result<HashMap<Content, usize>, LibError> {

        // First of all, let's check if the robot happens to be near a tile with a Market on it

        let mut market_near:bool = false;
        let mut market_dir = Direction::Left; // initialized

        let robot_view= robot_view(robot, &world);
        for (i, row) in robot_view.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match (i, j) {
                    (0, 1) | (1, 0) | (1, 2) | (2, 1) => {
                        match col {
                            | None => (),
                            | Some(tile) => {
                                match tile.content {
                                    Content::Market(_) => {
                                        market_near = true;
                                        match (i, j) {
                                            (0, 1) => market_dir = Direction::Up,
                                            (1, 0) => market_dir = Direction::Left,
                                            (1, 2) => market_dir = Direction::Right,
                                            (2, 1) => market_dir = Direction::Down,
                                            _ => return Err(LibError::OperationNotAllowed)
                                        }
                                    },
                                    _ => ()
                                }
                            }
                        }
                    },
                    _ => ()
                }
            }
        }

        // If the robot is NOT near a tile with a Market on it this tool cannot be used
        if !market_near {
            return Err(LibError::OperationNotAllowed);
        }

        // If the robot is near a Market, sell the items held in its backpack which can be sold

        let mut _coins_earned:usize = 0;

        let mut items_sold:HashMap<Content, usize> = HashMap::new();
        items_sold.insert(Content::Rock(0), 0usize);
        items_sold.insert(Content::Tree(0), 0usize);
        items_sold.insert(Content::Fish(0), 0usize);

        let cloned_contents = robot.get_backpack().get_contents().clone();

        for (item, qty) in cloned_contents {
            if vec![Content::Rock(0), Content::Tree(0), Content::Fish(0)].contains(&item)
                && qty > 0 {
                match put(
                    robot,
                    world,
                    item.clone(),
                    qty,
                    market_dir.clone()
                ) {
                    Ok(earned) => {
                        _coins_earned += earned;
                        match item {
                            Content::Rock(0) => items_sold.insert(Content::Rock(0), qty),
                            Content::Tree(0) => items_sold.insert(Content::Tree(0), qty),
                            Content::Fish(0) => items_sold.insert(Content::Fish(0), qty),
                            _ => None
                        };
                    },
                    Err(LibError::NotEnoughSpace(tried)) => {
                        return Err(LibError::NotEnoughSpace(tried));
                    },
                    _ => {
                        eprintln!("PUT arguments: {:?} {:?} {:?}", item.clone(), qty, market_dir.clone());
                        panic!("UNEXPECTED ERROR - CONTACT THE GROUP")
                    }
                }
            }
        }

        Ok(items_sold)
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
    *   swift_seller() function. The Market should be detected four times only, and the call
    *   should fail every time the robot is in a corner. When it is in fact near a Market, it tries
    *   to sell the content of its backpack, and with it being empty, the function should return a
    *   map with three entries, each with an associated value of 0.
    *
    *   Copyright: comment format courtesy of the common crate
    */
    #[test]
    fn sell_to_market() {

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

                let environmental_conditions =
                    EnvironmentalConditions::new(&[WeatherType::Sunny],
                                                 15,
                                                 12);
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
                let movements:&[Direction] = &[
                    Direction::Right, Direction::Right,
                    Direction::Down, Direction::Down,
                    Direction::Left, Direction::Left,
                    Direction::Up, Direction::Up
                ];

                // For each movement, perform the following actions
                for movement in movements {
                    // Since I created a world ad hoc, those movements should be possible
                    go(self, world, movement.clone()).expect("CANNOT MOVE");

                    // Call the function
                    match SwiftSeller::swift_seller(self, world) {
                        Err(LibError::OperationNotAllowed) => println!("No Market nearby!"),
                        Err(any) => println!("{:?}", any),
                        Ok(map) => {
                            println!("Sold to market:");
                            for (key, value) in map {
                                println!("\t- item: {}, qty: {}", key, value)
                            }
                        }
                    }
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
