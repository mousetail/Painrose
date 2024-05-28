use strum::{EnumString, VariantArray};

use crate::language::FollowableDirection;

use super::draw;
use super::tile_coordinate::CoordinateTraversalError;
use super::tiling::{EdgeDefinitionType, OutgoingEdgeDefinition, RelativeDirection, Tiling};
use std::f32::consts;

const SCALING_FACTOR: f32 = 1.618033988;
const SCALING_FACTOR_INVERSE: f32 = 1.0 / SCALING_FACTOR;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, VariantArray, EnumString)]
pub enum AbsoluteDirection {
    #[strum(serialize = "north", serialize = "n", ascii_case_insensitive)]
    North,
    #[strum(serialize = "east", serialize = "e", ascii_case_insensitive)]
    East,
    #[strum(serialize = "south", serialize = "s", ascii_case_insensitive)]
    South,
    #[strum(serialize = "west", serialize = "w", ascii_case_insensitive)]
    West,
}

impl FollowableDirection for AbsoluteDirection {
    fn turn_left(self) -> Self {
        match self {
            AbsoluteDirection::North => AbsoluteDirection::West,
            AbsoluteDirection::East => AbsoluteDirection::South,
            AbsoluteDirection::South => AbsoluteDirection::East,
            AbsoluteDirection::West => AbsoluteDirection::North,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            AbsoluteDirection::North => AbsoluteDirection::East,
            AbsoluteDirection::East => AbsoluteDirection::South,
            AbsoluteDirection::South => AbsoluteDirection::West,
            AbsoluteDirection::West => AbsoluteDirection::North,
        }
    }

    fn opposite(self) -> Self {
        match self {
            AbsoluteDirection::North => AbsoluteDirection::South,
            AbsoluteDirection::East => AbsoluteDirection::West,
            AbsoluteDirection::South => AbsoluteDirection::North,
            AbsoluteDirection::West => AbsoluteDirection::East,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, VariantArray)]
pub enum Tile {
    A,
    B,
    C,
    D,
    E,
}

#[derive(Copy, Clone, Debug)]
struct ShapeInfo {
    width: f32,
    height: f32,
    bottom_angle: f32,
    side_angle: f32,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, char> {
        match value.to_ascii_lowercase() {
            'a' => Ok(Tile::A),
            'b' => Ok(Tile::B),
            'c' => Ok(Tile::C),
            'd' => Ok(Tile::D),
            'e' => Ok(Tile::E),
            err => Err(err),
        }
    }
}

impl ShapeInfo {
    fn new(angle: f32) -> ShapeInfo {
        let angle_radians = angle.to_radians();

        let width = (angle_radians * 0.5).cos() * SCALING_FACTOR_INVERSE * 2.0;
        let height = (angle_radians * 0.5).sin() * SCALING_FACTOR_INVERSE * 2.0;

        ShapeInfo {
            width,
            height,
            side_angle: angle_radians,
            bottom_angle: consts::PI - angle_radians,
        }
    }
}

#[derive(Clone, Copy, Debug)]

pub struct AllShapeInfos {
    thin_rhomb: ShapeInfo,
    thick_rhomb: ShapeInfo,
}

impl draw::DrawableTileCachedData for AllShapeInfos {
    fn new() -> AllShapeInfos {
        AllShapeInfos {
            thin_rhomb: ShapeInfo::new(36.0),
            thick_rhomb: ShapeInfo::new(108.0),
        }
    }
}

impl draw::DrawableTile for Tile {
    const SCALING_FACTOR: f32 = SCALING_FACTOR;
    type Data = AllShapeInfos;

    fn get_coordinate(self, shape_info: AllShapeInfos) -> (glam::Vec2, f32) {
        use glam::Vec2;
        match self {
            Tile::A => (
                Vec2::new(
                    0.0,
                    (shape_info.thick_rhomb.height - SCALING_FACTOR_INVERSE) * 0.5,
                ),
                consts::PI,
            ),
            Tile::B => (
                Vec2::new(
                    (-shape_info.thick_rhomb.bottom_angle / 2.0 + consts::FRAC_PI_2).sin()
                        * shape_info.thin_rhomb.width
                        * SCALING_FACTOR_INVERSE
                        / 2.0,
                    (-shape_info.thick_rhomb.bottom_angle / 2.0 + consts::FRAC_PI_2).cos()
                        * shape_info.thin_rhomb.width
                        * SCALING_FACTOR_INVERSE
                        / 2.0
                        + 0.5
                        - SCALING_FACTOR_INVERSE,
                ),
                -shape_info.thick_rhomb.side_angle / 2.0 + consts::FRAC_PI_2,
            ),
            Tile::C => (
                Vec2::new(
                    -shape_info.thick_rhomb.width / 4.0,
                    -shape_info.thick_rhomb.height / 4.0,
                ),
                shape_info.thick_rhomb.bottom_angle / 2.0 - consts::PI,
            ),
            Tile::D => {
                let scaling_factor = shape_info.thin_rhomb.height / 2.0;
                assert_eq!(
                    shape_info.thin_rhomb.height * SCALING_FACTOR_INVERSE
                        + SCALING_FACTOR_INVERSE * SCALING_FACTOR_INVERSE,
                    SCALING_FACTOR_INVERSE
                );
                (
                    Vec2::new(
                        -shape_info.thin_rhomb.width * 0.5 * scaling_factor,
                        shape_info.thin_rhomb.height * 0.5 * (1.0 - scaling_factor),
                    ),
                    shape_info.thin_rhomb.side_angle / 2.0 + consts::FRAC_PI_2,
                )
            }
            Tile::E => (
                Vec2::new(
                    -shape_info.thin_rhomb.width * 0.25,
                    -shape_info.thin_rhomb.height * 0.25,
                ),
                shape_info.thin_rhomb.bottom_angle / 2.0 + consts::PI,
            ),
        }
    }

    fn get_shape(self, shape_info: AllShapeInfos) -> Vec<glam::Vec2> {
        use glam::Vec2;

        let rhomb = match self {
            Tile::A | Tile::C | Tile::E => shape_info.thick_rhomb,
            Tile::D | Tile::B => shape_info.thin_rhomb,
        };

        return vec![
            Vec2::new(-rhomb.width / 2.0, 0.0),
            Vec2::new(0.0, -rhomb.height / 2.0),
            Vec2::new(rhomb.width / 2.0, 0.0),
            //
            // Vec2::new(rhomb.width / 2.0 * 0.55, rhomb.height / 2.0 * 0.45),
            // Vec2::new(0.05, 0.05),
            // Vec2::new(0.05, 0.1),
            // Vec2::new(rhomb.width / 2.0 * 0.45, rhomb.height / 2.0 * 0.55),
            //
            Vec2::new(0.0, rhomb.height / 2.0),
            //
            // Vec2::new(-rhomb.width / 2.0 * 0.45, rhomb.height / 2.0 * 0.55),
            // Vec2::new(-0.05, 0.1),
            // Vec2::new(-0.05, 0.05),
            // Vec2::new(-rhomb.width / 2.0 * 0.55, rhomb.height / 2.0 * 0.45),
        ];
    }

    fn get_color(self) -> &'static str {
        match self {
            Tile::A => "red",
            Tile::B => "green",
            Tile::C => "blue",
            Tile::D => "magenta",
            Tile::E => "orange",
        }
    }
}
pub struct RhombTiling;

impl Tiling for RhombTiling {
    type Edge = AbsoluteDirection;

    type Tile = Tile;

    const TILE_PATTERN: &'static [Tile] = &[Tile::C, Tile::E, Tile::D, Tile::B, Tile::A, Tile::A];

    fn get_internal_edge_definition(
        tile: Self::Tile,
        direction: Self::Edge,
    ) -> OutgoingEdgeDefinition<Self::Tile, Self::Edge> {
        match tile {
            Tile::A => match direction {
                AbsoluteDirection::North => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::North),
                    direction: vec![],
                },
                AbsoluteDirection::East => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Left, RelativeDirection::Right],
                },
                AbsoluteDirection::South => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Right],
                },
                AbsoluteDirection::West => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Left],
                },
            },
            Tile::B => match direction {
                AbsoluteDirection::North => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::North),
                    direction: vec![],
                },
                AbsoluteDirection::East => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Right, RelativeDirection::Left],
                },
                AbsoluteDirection::South => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Right, RelativeDirection::Right],
                },
                AbsoluteDirection::West => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::South),
                    direction: vec![RelativeDirection::Left],
                },
            },
            Tile::C => match direction {
                AbsoluteDirection::North => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::South),
                    direction: vec![RelativeDirection::Right],
                },
                AbsoluteDirection::East => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                    direction: vec![RelativeDirection::Left],
                },
                AbsoluteDirection::South => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                    direction: vec![RelativeDirection::Right],
                },
                AbsoluteDirection::West => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Left, RelativeDirection::Left],
                },
            },
            Tile::D => match direction {
                AbsoluteDirection::North => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::E, AbsoluteDirection::North),
                    direction: vec![],
                },
                AbsoluteDirection::East => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Right, RelativeDirection::Left],
                },
                AbsoluteDirection::South => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Right, RelativeDirection::Right],
                },
                AbsoluteDirection::West => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Left, RelativeDirection::Left],
                },
            },
            Tile::E => match direction {
                AbsoluteDirection::North => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::North),
                    direction: vec![],
                },
                AbsoluteDirection::East => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                    direction: vec![RelativeDirection::Left],
                },
                AbsoluteDirection::South => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                    direction: vec![RelativeDirection::Right],
                },
                AbsoluteDirection::West => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                    direction: vec![RelativeDirection::Left],
                },
            },
        }
    }

    fn get_external_edge_definition(
        tile: Self::Tile,
        direction: Self::Edge,
        side: Vec<RelativeDirection>,
    ) -> OutgoingEdgeDefinition<Self::Tile, Self::Edge> {
        match tile {
            Tile::A | Tile::C | Tile::E => match (direction, side.as_slice()) {
                // North
                (AbsoluteDirection::North, [RelativeDirection::Left, RelativeDirection::Left]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::West),
                        direction: vec![],
                    }
                }
                (AbsoluteDirection::North, [RelativeDirection::Left, RelativeDirection::Right]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::East),
                        direction: vec![],
                    }
                }
                (AbsoluteDirection::North, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::South),
                    direction: vec![],
                },
                // East
                (AbsoluteDirection::East, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::West),
                    direction: vec![],
                },
                (AbsoluteDirection::East, [RelativeDirection::Right, RelativeDirection::Left]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::East),
                        direction: vec![],
                    }
                }
                (AbsoluteDirection::East, [RelativeDirection::Right, RelativeDirection::Right]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::South),
                        direction: vec![],
                    }
                }
                // South
                (AbsoluteDirection::South, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::West),
                    direction: vec![],
                },
                (AbsoluteDirection::South, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::North),
                    direction: vec![],
                },
                // West
                (AbsoluteDirection::West, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::East),
                    direction: vec![],
                },
                (AbsoluteDirection::West, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::South),
                    direction: vec![],
                },
                _ => {
                    panic!(
                        "Bad Input for direction {direction:?} and side {side:?} and tile {tile:?}"
                    )
                }
            },
            Tile::B | Tile::D => match (direction, side.as_slice()) {
                // North
                (AbsoluteDirection::North, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::E, AbsoluteDirection::West),
                    direction: vec![],
                },
                (AbsoluteDirection::North, [RelativeDirection::Right, RelativeDirection::Left]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::East),
                        direction: vec![],
                    }
                }
                (
                    AbsoluteDirection::North,
                    [RelativeDirection::Right, RelativeDirection::Right],
                ) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::South),
                    direction: vec![],
                },
                // East
                (AbsoluteDirection::East, [RelativeDirection::Left, RelativeDirection::Left]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::West),
                        direction: vec![],
                    }
                }
                (AbsoluteDirection::East, [RelativeDirection::Left, RelativeDirection::Right]) => {
                    OutgoingEdgeDefinition {
                        edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::South),
                        direction: vec![RelativeDirection::Right],
                    }
                }
                (AbsoluteDirection::East, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::South),
                    direction: vec![RelativeDirection::Left],
                },

                // South
                (AbsoluteDirection::South, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Right],
                },
                (AbsoluteDirection::South, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                    direction: vec![RelativeDirection::Left, RelativeDirection::Right],
                },

                // West
                (AbsoluteDirection::West, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::E, AbsoluteDirection::East),
                    direction: vec![],
                },
                (AbsoluteDirection::West, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::E, AbsoluteDirection::South),
                    direction: vec![],
                },
                //
                _ => {
                    panic!(
                        "Bad input for direction {direction:?} and side {side:?} and tile {tile:?}"
                    )
                }
            },
        }
    }

    fn can_tile_fit_in_tile(
        inner_tile: Self::Tile,
        outer_tile: Self::Tile,
    ) -> Result<(), CoordinateTraversalError<Self::Tile>> {
        match (inner_tile, outer_tile) {
            (Tile::A | Tile::B | Tile::C, Tile::A | Tile::C | Tile::E)
            | (Tile::D | Tile::E, Tile::B | Tile::D) => Ok(()),
            _ => Err(CoordinateTraversalError {
                inner_tile,
                outer_tile,
            }),
        }
    }
}
