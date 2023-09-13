mod draw;
mod penrose;

fn main() {
    draw::draw_svg(vec![
        penrose::TileReference::new(vec![penrose::Tile::A]),
        penrose::TileReference::new(vec![penrose::Tile::B]),
        penrose::TileReference::new(vec![]),
        penrose::TileReference::new(vec![penrose::Tile::D, penrose::Tile::D]),
        penrose::TileReference::new(vec![penrose::Tile::E, penrose::Tile::D]),
        penrose::TileReference::new(vec![penrose::Tile::A, penrose::Tile::A, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::B, penrose::Tile::A, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::C, penrose::Tile::A, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::A, penrose::Tile::C, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::B, penrose::Tile::C, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::C, penrose::Tile::C, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::D, penrose::Tile::B, penrose::Tile::E]),
        penrose::TileReference::new(vec![penrose::Tile::E, penrose::Tile::B, penrose::Tile::E]),
    ])
}
