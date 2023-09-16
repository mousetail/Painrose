pub mod geometry;

use crate::geometry::draw;
use crate::geometry::rhomb::{RhombTiling, Tile};
use crate::geometry::tiling::{TileCoordinate, TileCoordinateError, Tiling};

fn create_subtiles(tile: Vec<Tile>, level: usize) -> Vec<Vec<Tile>> {
    use Tile::*;
    let tile_options = match tile
        .first()
        .unwrap_or(&RhombTiling::TILE_PATTERN[(level + 1) % RhombTiling::TILE_PATTERN.len()])
    {
        A | C | E => vec![A, B, C],
        D | B => vec![D, E],
    };

    return tile_options
        .into_iter()
        .flat_map(|t| {
            let m = [vec![t], tile.clone()].concat();
            if level == 0 {
                return vec![m].into_iter();
            } else {
                return create_subtiles(m, level - 1).into_iter();
            }
        })
        .collect();
}
fn main() -> Result<(), TileCoordinateError<Tile>> {
    let tiles: Result<Vec<_>, _> = create_subtiles(vec![], 3)
        .into_iter()
        .map(TileCoordinate::<RhombTiling>::new)
        .collect();

    draw::draw_svg(tiles?);

    Ok(())
}
