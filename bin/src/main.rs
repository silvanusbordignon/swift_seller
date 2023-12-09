// Energy and Event
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;

use robotics_lib::interface::{ look_at_sky, where_am_i };

use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::runner::Runnable;

use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::Tile;

use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::world_generator::World;

/// Create a 3x3 world where the robot spawns right next to a Market content tile
pub fn main() {

    // Generate the world

    struct SmallWorld {
        size: usize
    }
    impl SmallWorld {
        fn init() -> Self { SmallWorld { size: 3 } }
    }
    impl Generator for SmallWorld {
        fn gen(&mut self) -> World {

            // Generate a grass square of side self.size
            let mut map: Vec<Vec<Tile>> = Vec::new();
            for i in 0..self.size {
                let mut row: Vec<Tile> = Vec::new();
                for j in 0..self.size {
                    if i == 1 && j == 2 {
                        row.push(Tile {
                            tile_type: TileType::Grass,
                            content: Content::Market(1),
                            elevation: 0
                        })
                    }
                    else {
                        row.push(Tile {
                            tile_type: TileType::Grass,
                            content: Content::None,
                            elevation: 0
                        })
                    }
                }
                map.push(row);
            }

            // On this grass world, the sun always shines
            let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap();

            (map, (1, 1), environmental_conditions, 100.0, None)
        }
    }

    struct MyRobot(Robot);

    impl Runnable for MyRobot {
        fn process_tick(&mut self, world: &mut robotics_lib::world::World) {

            // let ec:EnvironmentalConditions = look_at_sky(world);
            // println!("It's {}", ec.get_time_of_day_string());

            println!("---- START TICK ----");
            self.print_area_around_me(world);
            println!("---- END TICK ----");
        }

        fn handle_event(&mut self, _event: Event) {
            // react to this event in a GUI
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
        fn get_backpack_mut(&mut self) -> &mut BackPack {
            &mut self.0.backpack
        }
    }
    impl MyRobot {
        fn print_area_around_me(&mut self, world: &mut robotics_lib::world::World) {

            let (robot_view, robot_position) = where_am_i(self, &world);

            for row in robot_view.iter() {
                for col in row.iter() {
                    match col {
                        | None => print!("default_unknown_tile"),
                        | Some(tile) => {
                            if tile.content != Content::None {
                                print!(" {:?}({:?}) ", tile.tile_type, tile.content)
                            }
                            else {
                                print!(" {:?} ", tile.tile_type)
                            }
                        },
                    }
                }
                println!();
            }
        }
    }

    // Instances

    let mut pwg = SmallWorld::init();

    let robot = MyRobot(Robot::new());
    let run = Runner::new(Box::new(robot), &mut pwg);

    match run {
        | Ok(mut r) => {
            for _ in 0..3 {
                let _ = r.game_tick();
            }
        }
        | Err(e) => println!("{:?}", e),
    }
}