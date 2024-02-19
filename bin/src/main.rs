use std::collections::{HashMap, VecDeque};
use std::io::SeekFrom;

// Energy and Event
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;

use robotics_lib::interface::{destroy, go, look_at_sky, where_am_i, Direction};

use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::Runnable;
use robotics_lib::runner::{Robot, Runner};

use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType;

use ragnarok::GuiRunner;
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::world_generator::World;

pub mod demo_worldgen;
use demo_worldgen::DemoWorld;
use swift_seller::SwiftSeller;

/// Create a 3x3 world where the robot spawns right next to a Market content tile
pub fn main() {
    struct MyRobot {
        robot: Robot,
        go_directions: VecDeque<Direction>,
        destroy_directions: VecDeque<Direction>,
        tool_params: VecDeque<Vec<Content>>,
        azioni: VecDeque<i32>,
    }

    impl MyRobot {
        pub fn new() -> Self {
            Self {
                robot: Robot::new(),
                go_directions: VecDeque::from([
                    Direction::Down,
                    Direction::Down,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Right,
                    Direction::Right,
                    Direction::Left,
                    Direction::Left,
                ]),
                destroy_directions: VecDeque::from([
                    Direction::Up,
                    Direction::Down,
                    Direction::Right,
                    Direction::Up,
                    Direction::Down,
                    ]),
                tool_params: VecDeque::from([vec![Content::Fish(0), Content::Rock(0), Content::Tree(0)],vec![Content::Fish(0), Content::Rock(0), Content::Tree(0)], vec![Content::Tree(0)]]),
                azioni: VecDeque::from([0, 0, 0, 2, 0, 1, 1, 0, 2, 0, 1, 0, 1, 1, 0, 0, 2]),            
            }
        }
    }

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
            match self.azioni.pop_front() {
                None => {},
                Some(id) => {
                    match id {
                        0 => { 
                            let dir = self.go_directions.pop_front().unwrap();
                            let _ = go(self, world, dir); 
                        }
                        1 => { 
                            let dir = self.destroy_directions.pop_front().unwrap();
                            let _ = destroy(self, world, dir);
                        }
                        2 => { 
                            let vec = self.tool_params.pop_front().unwrap();
                            let _ = SwiftSeller::swift_seller(self, world, vec); 
                        }
                        _ => {},
                    }
                }                
            }
        }

        fn handle_event(&mut self, _event: Event) {
            // react to this event in a GUI
        }

        fn get_energy(&self) -> &Energy {
            &self.robot.energy
        }
        fn get_energy_mut(&mut self) -> &mut Energy {
            &mut self.robot.energy
        }

        fn get_coordinate(&self) -> &Coordinate {
            &self.robot.coordinate
        }
        fn get_coordinate_mut(&mut self) -> &mut Coordinate {
            &mut self.robot.coordinate
        }

        fn get_backpack(&self) -> &BackPack {
            &self.robot.backpack
        }
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.robot.backpack
        }
    }

    // Instances
    let robot = MyRobot::new();
    let mut pwg = DemoWorld::init();

    let gui_runner = GuiRunner::new(Box::new(robot), &mut pwg).unwrap();

    gui_runner.run().unwrap();
}
