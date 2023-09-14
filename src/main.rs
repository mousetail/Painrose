mod draw;
mod penrose;

fn create_ribbon(
    start_point: penrose::TileReference,
    direction: penrose::AbsoluteDirection,
    length: usize,
) -> Vec<penrose::TileReference> {
    let mut tiles = vec![start_point];
    let mut direction = direction;
    for _ in 0..length {
        let (new_tile, new_direction) = tiles.last().unwrap().go(direction);
        println!("{new_tile:?} {:?}", new_direction.invert());
        tiles.push(new_tile);
        direction = new_direction.invert();
    }

    tiles
}
fn main() {
    let mut tiles = vec![];
    tiles.extend(create_ribbon(
        penrose::TileReference::new(vec![]),
        penrose::AbsoluteDirection::North,
        20,
    ));
    tiles.extend(create_ribbon(
        penrose::TileReference::new(vec![])
            .go(penrose::AbsoluteDirection::East)
            .0,
        penrose::AbsoluteDirection::East,
        20,
    ));
    tiles.extend(create_ribbon(
        penrose::TileReference::new(vec![])
            .go(penrose::AbsoluteDirection::West)
            .0,
        penrose::AbsoluteDirection::East,
        20,
    ));

    draw::draw_svg(tiles);
}
