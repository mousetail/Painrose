// TODO: Fix the D rhomb, and the angles
use crate::penrose::{Tile, TileReference};
use glam::f32::*;
use std::{f32::consts, io::Write};

const SCALING_FACTOR: f32 = 1.0 / 1.618033988;

pub fn draw_svg(coords: Vec<TileReference>) {
    let mut file = std::fs::File::create("Tiles.svg").unwrap();

    file.write_all(b"<svg version=\"1.1\" height=\"106\" width=\"106\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-5 -5 10 10\">\n").unwrap();

    for coord in coords {
        writeln!(
            file,
            "<path fill=\"none\" stroke=\"black\" stroke-width=\"0.1\" d=\"M {} Z\"/>",
            &draw(&coord)
                .into_iter()
                .map(|c| format!("L {} {} ", c.0, c.1))
                .collect::<String>()[1..]
        )
        .unwrap();
    }

    file.write_all(b"</svg>").unwrap();
}

fn draw(coordinate: &TileReference) -> Vec<(f32, f32)> {
    let mut matrix = glam::f32::Mat4::IDENTITY;
    let reference = TileReference(vec![]);
    for index in (0..coordinate.0.len()) {
        let (mut position, mut angle) = get_offset_and_rotation(coordinate.get_at(index));

        if index == coordinate.0.len() - 1 {
            let (default_position, default_angle) =
                get_offset_and_rotation(reference.get_at(index));
            position -= default_position;
            angle -= default_angle;
        }

        matrix = Mat4::from_translation(position)
            * Mat4::from_quat(Quat::from_rotation_z(angle))
            * matrix;
    }

    let coordinates = match coordinate.get_at(0) {
        Tile::A | Tile::C | Tile::E => vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(-1.0, 1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
        ],
        Tile::D | Tile::B => vec![
            Vec3::new(-1.0, -1.5, 0.0),
            Vec3::new(1.0, -1.5, 0.0),
            Vec3::new(1.0, 1.5, 0.0),
            Vec3::new(-1.0, 1.5, 0.0),
        ],
    };

    return coordinates
        .into_iter()
        .map(|i| matrix.transform_vector3(i))
        .map(|i| (i.x, i.y))
        .collect();
}

fn get_offset_and_rotation(tile: Tile) -> (Vec3, f32) {
    let thin_big_angle: f32 = (114.0f32).to_radians();
    let thin_small_angle: f32 = (36.0f32).to_radians();

    let thick_big_angle: f32 = (108.0f32).to_radians();
    let thick_small_angle: f32 = (72.0f32).to_radians();

    let thick_side_length = SCALING_FACTOR;
    let thick_width = (thick_side_length * thick_side_length - 0.5 * 0.5).sqrt();

    let thin_width = SCALING_FACTOR - SCALING_FACTOR * SCALING_FACTOR;
    let thin_height = (SCALING_FACTOR.powf(2.0) - thin_width.powf(2.0)).sqrt();
    println!("{thin_big_angle} {thin_small_angle} {thick_big_angle} {thick_small_angle} {thick_side_length} {thick_width} {thin_width} {thin_height}");

    match tile {
        Tile::A => (Vec3::new(0.0, 1.0 - SCALING_FACTOR, 0.0), 0.0),
        Tile::B => (
            Vec3::new(
                thick_width * thick_side_length * SCALING_FACTOR * SCALING_FACTOR / thick_width,
                0.5 * thick_side_length * SCALING_FACTOR * SCALING_FACTOR / thick_width,
                0.0,
            ),
            consts::FRAC_PI_2 - thin_small_angle / 2.0,
        ),
        Tile::C => (Vec3::new(thick_width / 2.0, -0.5 / 2.0, 0.0), 0.0),
        Tile::D => (
            Vec3::new(
                -thin_height * thin_height / SCALING_FACTOR / thick_side_length,
                -thin_width * thin_height / SCALING_FACTOR / thick_side_length,
                0.0,
            ),
            0.0,
        ),
        Tile::E => (Vec3::new(-thin_height * 0.5, -thin_width * 0.5, 0.0), 0.0),
        _ => panic!("Invalid tile"),
    }
}
