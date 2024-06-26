use crate::geometry::{
    draw::{DrawableTile, ShapeInfo},
    tiling::Tiling,
};

use super::{FollowableDirection, LanguageState};
use crate::geometry::draw;

pub(super) fn get_shapes<T: Tiling>(state: &LanguageState<T>) -> Vec<ShapeInfo>
where
    T::Tile: DrawableTile,
    T::Edge: FollowableDirection,
{
    state
        .code
        .iter()
        .map(|(coord, code)| {
            draw::TileDrawOptions {
                coordinate: coord.clone(),
                fill: coord != &state.instruction_pointer,
                label: match code.0 {
                    '\n' => '¶',
                    '\t' => '↹',
                    a => a,
                }
                .to_string(),
                supertile_index: 0,
            }
            .get_shape()
        })
        .collect()
}

pub(super) fn draw<T: Tiling>(state: &LanguageState<T>) -> std::io::Result<()>
where
    T::Tile: DrawableTile,
    T::Edge: FollowableDirection,
{
    draw::draw_svg(get_shapes(state))
}
