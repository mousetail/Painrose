#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum RelativeDirection {
    Left,
    Right,
}

impl RelativeDirection {
    fn invert(self) -> Self {
        match self {
            RelativeDirection::Left => RelativeDirection::Right,
            RelativeDirection::Right => RelativeDirection::Left,
        }
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum AbsoluteDirection {
    North,
    East,
    South,
    West,
}
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    A,
    B,
    C,
    D,
    E,
}

enum EdgeDefinitionType {
    Inside(Tile, AbsoluteDirection),
    Outside(AbsoluteDirection),
}

struct OutgoingEdgeDefinition {
    edge_type: EdgeDefinitionType,
    direction: Vec<RelativeDirection>,
}

struct ExternalIncomingEdgeDefinition {
    direction: AbsoluteDirection,
    half: Vec<RelativeDirection>,
}

struct InternalIncomingEdgeDefinition {
    tile: Tile,
    direction: AbsoluteDirection,
}

#[allow(unused)]
fn get_internal_edge_definition(
    tile: Tile,
    direction: AbsoluteDirection,
) -> OutgoingEdgeDefinition {
    match tile {
        Tile::A => match direction {
            AbsoluteDirection::South => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                direction: vec![RelativeDirection::Right],
            },
            AbsoluteDirection::West => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                direction: vec![RelativeDirection::Left],
            },
            AbsoluteDirection::North => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::North),
                direction: vec![],
            },
            AbsoluteDirection::East => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::North),
                direction: vec![],
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
                edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::South),
                direction: vec![],
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
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::South),
                direction: vec![RelativeDirection::Left],
            },
            AbsoluteDirection::West => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Inside(Tile::B, AbsoluteDirection::West),
                direction: vec![],
            },
        },
        Tile::D => match direction {
            AbsoluteDirection::North => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Inside(Tile::E, AbsoluteDirection::North),
                direction: vec![],
            },
            AbsoluteDirection::East => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
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
                direction: vec![RelativeDirection::Right],
            },
            AbsoluteDirection::South => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                direction: vec![RelativeDirection::Right, RelativeDirection::Left],
            },
            AbsoluteDirection::West => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                direction: vec![RelativeDirection::Left],
            },
        },
        _ => panic!("Unexpected tile when computing internal edge definition {tile:?}"),
    }
}

fn get_external_edge_definition(
    tile: Tile,
    direction: AbsoluteDirection,
    side: Vec<RelativeDirection>,
) -> OutgoingEdgeDefinition {
    match tile {
        Tile::A | Tile::C | Tile::E => match (direction, side.as_slice()) {
            // North
            (AbsoluteDirection::North, [RelativeDirection::Left, RelativeDirection::Left]) => {
                OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::West),
                    direction: vec![RelativeDirection::Right],
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
                edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::South),
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
                edge_type: EdgeDefinitionType::Inside(Tile::C, AbsoluteDirection::West),
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
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::North),
                direction: vec![RelativeDirection::Left, RelativeDirection::Left],
            },
            _ => {
                panic!("Bad Input for direction {direction:?} and side {side:?} and tile {tile:?}")
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
            (AbsoluteDirection::North, [RelativeDirection::Right, RelativeDirection::Right]) => {
                OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::South),
                    direction: vec![],
                }
            }
            // East
            (AbsoluteDirection::East, [RelativeDirection::Left, RelativeDirection::Left]) => {
                OutgoingEdgeDefinition {
                    edge_type: EdgeDefinitionType::Inside(Tile::D, AbsoluteDirection::South),
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

            // East
            (AbsoluteDirection::South, [RelativeDirection::Left]) => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                direction: vec![RelativeDirection::Right],
            },
            (AbsoluteDirection::South, [RelativeDirection::Right]) => OutgoingEdgeDefinition {
                edge_type: EdgeDefinitionType::Outside(AbsoluteDirection::East),
                direction: vec![RelativeDirection::Left, RelativeDirection::Right],
            },
            _ => {
                panic!("Bad input for direction {direction:?} and side {side:?} and tile {tile:?}")
            }
        },
        _ => panic!("Unexpected tile when computing external edge definition {tile:?}"),
    }
}

fn main() {
    let external_edge_definitions = vec![(
        ExternalIncomingEdgeDefinition {
            direction: AbsoluteDirection::West,
            half: vec![],
        },
        OutgoingEdgeDefinition {
            edge_type: EdgeDefinitionType::Inside(Tile::A, AbsoluteDirection::North),
            direction: vec![],
        },
    )];

    println!("Hello, world!");
}
