use strum::VariantArray;

use crate::language::FollowableDirection;

use super::{draw::DrawableTile, tile_coordinate::CoordinateTraversalError};

#[derive(Debug, PartialEq)]
pub enum EdgeDefinitionType<Tile, Edge> {
    Inside(Tile, Edge),
    Outside(Edge),
}

#[derive(Debug, PartialEq)]
pub struct OutgoingEdgeDefinition<Tile, Edge> {
    pub edge_type: EdgeDefinitionType<Tile, Edge>,
    pub direction: Vec<RelativeDirection>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum RelativeDirection {
    Left,
    Right,
}

impl RelativeDirection {
    #[allow(unused)]
    pub(crate) fn invert(self) -> Self {
        match self {
            RelativeDirection::Left => RelativeDirection::Right,
            RelativeDirection::Right => RelativeDirection::Left,
        }
    }
}

pub trait Tiling {
    type Edge: 'static + Copy + Clone + PartialEq + std::fmt::Debug + std::hash::Hash + VariantArray;
    type Tile: 'static + Copy + Clone + PartialEq + std::fmt::Debug + std::hash::Hash + VariantArray;

    const TILE_PATTERN: &'static [Self::Tile];

    fn get_internal_edge_definition(
        tile: Self::Tile,
        direction: Self::Edge,
    ) -> OutgoingEdgeDefinition<Self::Tile, Self::Edge>;

    fn get_external_edge_definition(
        tile: Self::Tile,
        direction: Self::Edge,
        side: Vec<RelativeDirection>,
    ) -> OutgoingEdgeDefinition<Self::Tile, Self::Edge>;

    fn can_tile_fit_in_tile(
        inside_tile: Self::Tile,
        outside_tile: Self::Tile,
    ) -> Result<(), CoordinateTraversalError<Self::Tile>>;
}

// TODO: Replace this with a trait alias when https://github.com/rust-lang/rust/issues/41517 is stabilized
pub trait DrawableTiling: Tiling<Tile: DrawableTile> {}

impl<T: Tiling> DrawableTiling for T where <T as Tiling>::Tile: DrawableTile {}

pub trait ExecutableTiling: Tiling<Edge: FollowableDirection>
where
    <Self as Tiling>::Edge: FollowableDirection,
{
}

impl<T: Tiling> ExecutableTiling for T where <T as Tiling>::Edge: FollowableDirection {}
