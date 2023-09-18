use crate::geometry::{draw::DrawableTile, tiling::Tiling};

use super::{FollowableDirection, LanguageState};
use crate::geometry::draw;

pub(super) fn draw<T: Tiling>(state: &LanguageState<T>) -> std::io::Result<()>
where
    T::Tile: DrawableTile,
    T::Edge: FollowableDirection,
{
    let tiles: Vec<_> = state
        .code
        .iter()
        .map(|(coord, code)| draw::TileDrawOptions {
            coordinate: coord.clone(),
            fill: coord == &state.instruction_pointer,
            label: code.0.to_string(),
            supertile_index: 0,
        })
        .collect();

    draw::draw_svg(tiles)
}
