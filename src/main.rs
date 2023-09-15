mod draw;
mod rhomb;
mod tiling;

use crate::tiling::Tiling;

fn create_subtiles(tile: Vec<rhomb::Tile>, level: usize) -> Vec<Vec<rhomb::Tile>> {
    use crate::rhomb::Tile::*;

    let tile_options = match tile.first().unwrap_or(
        &rhomb::RhombTiling::TILE_PATTERN[(level + 1) % rhomb::RhombTiling::TILE_PATTERN.len()],
    ) {
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
fn main() -> Result<(), tiling::TileCoordinateError<rhomb::Tile>> {
    let tiles: Result<Vec<_>, _> = create_subtiles(vec![], 3)
        .into_iter()
        .map(tiling::TileCoordinate::<rhomb::RhombTiling>::new)
        .collect();

    draw::draw_svg(tiles?);

    Ok(())
}
