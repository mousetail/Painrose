mod draw;
mod penrose;

fn main() {
    draw::draw_svg(vec![
        penrose::TileReference(vec![]),
        penrose::TileReference(vec![penrose::Tile::A]),
        penrose::TileReference(vec![penrose::Tile::C]),
        penrose::TileReference(vec![penrose::Tile::C, penrose::Tile::C]),
    ])
}
