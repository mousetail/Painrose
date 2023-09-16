use penrose::geometry::rhomb::*;
use penrose::geometry::tiling::{
    EdgeDefinitionType, OutgoingEdgeDefinition, TileCoordinate, TileCoordinateError, Tiling,
};

#[test]
pub fn test_if_all_relations_are_symetric() {
    for tile in [Tile::A, Tile::B, Tile::C, Tile::D, Tile::E] {
        for direction in [
            AbsoluteDirection::North,
            AbsoluteDirection::East,
            AbsoluteDirection::South,
            AbsoluteDirection::West,
        ] {
            let definition = RhombTiling::get_internal_edge_definition(tile, direction);
            match definition.edge_type {
                EdgeDefinitionType::Inside(tile_2, direction_2) => {
                    assert_eq!(definition.direction, vec![]);
                    assert_eq!(
                        RhombTiling::get_internal_edge_definition(tile_2, direction_2),
                        OutgoingEdgeDefinition {
                            edge_type: EdgeDefinitionType::Inside(tile, direction),
                            direction: vec![]
                        },
                        "tile={tile:?} direction={direction:?} (internal)"
                    )
                }
                EdgeDefinitionType::Outside(direction_2) => {
                    let new_tile_type = match tile {
                        Tile::A | Tile::B | Tile::C => Tile::A,
                        Tile::D | Tile::E => Tile::B,
                    };

                    assert_eq!(
                        RhombTiling::get_external_edge_definition(
                            new_tile_type,
                            direction_2,
                            definition.direction.clone()
                        ),
                        OutgoingEdgeDefinition {
                            edge_type: EdgeDefinitionType::Inside(tile, direction),
                            direction: vec![],
                        },
                        "tile={tile:?} direction={direction:?}, (external)"
                    )
                }
            }
        }
    }
}

#[test]
fn test_if_graph_is_symetric() -> Result<(), TileCoordinateError<Tile>> {
    for tile_graph in [
        TileCoordinate::<RhombTiling>::new(vec![Tile::A])?,
        TileCoordinate::<RhombTiling>::new(vec![Tile::B])?,
        TileCoordinate::<RhombTiling>::new(vec![])?,
        TileCoordinate::<RhombTiling>::new(vec![Tile::D, Tile::D])?,
        TileCoordinate::<RhombTiling>::new(vec![Tile::E, Tile::D])?,
    ]
    .iter()
    {
        for direction in [
            AbsoluteDirection::North,
            AbsoluteDirection::East,
            AbsoluteDirection::South,
            AbsoluteDirection::West,
        ] {
            let (new_tile, new_direction) = tile_graph.go(direction)?;
            let (original_tile, original_direction) = new_tile.go(new_direction)?;

            assert_eq!(
                (&original_tile, original_direction),
                (tile_graph, direction),
                "{tile_graph:?} {direction:?}"
            );
        }
    }

    Ok(())
}
