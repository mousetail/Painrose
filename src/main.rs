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

#[derive(Debug)]
enum EdgeDefinitionType {
    Inside(Tile, AbsoluteDirection),
    Outside(AbsoluteDirection),
}

#[derive(Debug)]
struct OutgoingEdgeDefinition {
    edge_type: EdgeDefinitionType,
    direction: Vec<RelativeDirection>,
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

#[allow(unused)]
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
                panic!("Bad input for direction {direction:?} and side {side:?} and tile {tile:?}")
            }
        },
        _ => panic!("Unexpected tile when computing external edge definition {tile:?}"),
    }
}

#[derive(Clone, Debug, PartialEq)]
struct TileReference(Vec<Tile>);

impl TileReference {
    fn get_at(&self, index: usize) -> Tile {
        if index < self.0.len() {
            return self.0[index];
        }
        return TILE_PATTERN[index % TILE_PATTERN.len()];
    }

    fn set_at(&mut self, index: usize, tile: Tile) {
        while index >= self.0.len() {
            self.0.push(TILE_PATTERN[self.0.len() % TILE_PATTERN.len()]);
        }

        self.0[index] = tile;

        while self.0[self.0.len() - 1] == TILE_PATTERN[(self.0.len() - 1) % TILE_PATTERN.len()] {
            self.0.pop();
        }
    }

    fn go(&self, direction: AbsoluteDirection) -> (Self, AbsoluteDirection) {
        let mut copy = self.clone();

        let mut definition = get_internal_edge_definition(self.get_at(0), direction);

        let mut sides: Vec<Vec<RelativeDirection>> = vec![];
        let mut index = 0;

        loop {
            println!("{index:?} {copy:?} {definition:?}");
            match definition.edge_type {
                EdgeDefinitionType::Inside(tile, direction) => {
                    copy.set_at(index, tile);
                    if index == 0 {
                        return (copy, direction);
                    } else {
                        index -= 1;
                        definition =
                            get_external_edge_definition(tile, direction, sides.pop().unwrap())
                    }
                }
                EdgeDefinitionType::Outside(direction) => {
                    sides.push(definition.direction.iter().map(|i| i.invert()).collect());
                    index += 1;
                    definition = get_internal_edge_definition(copy.get_at(index), direction)
                }
            }
        }
    }
}

const TILE_PATTERN: [Tile; 5] = [Tile::A, Tile::B, Tile::D, Tile::E, Tile::C];

fn main() {
    let reference = TileReference(vec![]);

    println!(
        "{:?}",
        reference
            .go(AbsoluteDirection::West)
            .0
            .go(AbsoluteDirection::South)
    );
}
