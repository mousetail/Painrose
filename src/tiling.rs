#[derive(Debug, PartialEq)]
pub enum EdgeDefinitionType<Tile, Edge> {
    Inside(Tile, AbsoluteDirection),
    Outside(AbsoluteDirection),
}

#[derive(Debug, PartialEq)]
pub struct OutgoingEdgeDefinition<Tile, Edge> {
    pub edge_type: EdgeDefinitionType,
    pub direction: Vec<RelativeDirection>,
}

trait Tiling {
    type Edge;
    type Tile;

    fn get_internal_edge_definition(
        tile: Tile,
        direction: Edge,
    ) -> OutgoingEdgeDefinition<Tile, Edge>;

    fn get_external_edge_definition(
        tile: Tile,
        direction: Edge,
        side: Vec<RelativeDirection>,
    ) -> OutgoingEdgeDefinition<Tile, Edge>;

    fn new(coords: Vec<Tile>) -> Self;

    fn len(&self) -> usize;

    fn get_at(&self, position: usize) -> Tile;

    fn set_at(&mut self, position: usize, tile: Tile);

    fn set_at_no_validate(&mut self, position: usize);

    fn go(&self, direction: Edge) -> (Self, Edge);
}
