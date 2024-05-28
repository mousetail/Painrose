use super::tile_coordinate::CoordinateTraversalError;

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

pub trait All: Sized {
    fn all() -> &'static [Self];
    fn index(self) -> usize;
}

pub trait Tiling {
    type Edge: 'static + Copy + Clone + PartialEq + std::fmt::Debug + std::hash::Hash + All;
    type Tile: 'static + Copy + Clone + PartialEq + std::fmt::Debug + std::hash::Hash + All;

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
