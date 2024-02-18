use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::Tile;

use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::world_generator::World;

/// World for our demo
pub struct DemoWorld {
    size: usize,
}

impl DemoWorld {
    pub fn init() -> Self {
        DemoWorld {
            size: 4
        }
    }
}

/**************************************************************************
*  MAP:
*    ___________________________________________________
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    |   None     |   None     |   None     |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |  Shallow   |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    | Market(1)  |  Rock(1)   |  Fish(1)   |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    |   None     |   None     |  Rock(1)   |
*   |____________|____________|____________|____________|
*   |            |            |            |            |
*   |    Grass   |   Grass    |   Grass    |   Grass    |
*   |    0 el    |   0 el     |   0 el     |   0 el     |
*   |    None    | Market(2)  |  Tree(1)   |  Tree(2)   |
*   |____________|____________|____________|____________|
*
*   Copyright: comment format courtesy of the common crate
*/

impl Generator for DemoWorld {
    fn gen(&mut self) -> World {

        // Default world

        let mut map: Vec<Vec<Tile>> = Vec::new();
        for _ in 0..4 {
            let mut row: Vec<Tile> = Vec::new();
            for _ in 0..4 {
                row.push(Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                })
            }
            map.push(row);
        }
        map[1][3] = Tile {
            tile_type: TileType::ShallowWater,
            content: Content::None,
            elevation: 0,
        };

        // Adding the contents

        map[1][1].content = Content::Market(1);
        map[1][2].content = Content::Rock(1);
        map[1][3].content = Content::Fish(1);
        map[2][3].content = Content::Rock(1);
        map[3][1].content = Content::Market(2);
        map[3][2].content = Content::Tree(1);
        map[3][3].content = Content::Tree(2);

        // On this demo world, the sun always shines
        let environmental_conditions = EnvironmentalConditions::new(&[Sunny], 15, 12).unwrap();

        // Return the world

        (map, (0, 0), environmental_conditions, 100.0, None)
    }
}