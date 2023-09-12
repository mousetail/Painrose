// TODO: Fix the D rhomb, and the angles
use crate::penrose::{Tile, TileReference};
use glam::f32::*;
use std::f32::consts;

const SCALING_FACTOR: f32 = 1.0 / 1.618033988;

fn draw(coordinate: &TileReference) -> Vec<(f64, f64)> {
    let mut matrix = glam::f32::Mat4::IDENTITY;
    for index in (0..coordinate.0.len()) {}

    vec![]
}

fn get_offset_and_rotation(tile: Tile) -> (Vec3, f32) {
    let thin_big_angle: f32 = (114.0f32).to_radians();
    let thin_small_angle: f32 = (36.0f32).to_radians();

    let thick_big_angle: f32 = (108.0f32).to_radians();
    let thick_small_angle: f32 = (72.0f32).to_radians();

    let thick_side_length = SCALING_FACTOR;
    let thick_width = (0.5 * 0.5 - thick_side_length * thick_side_length).sqrt();

    let thin_width = SCALING_FACTOR - SCALING_FACTOR * SCALING_FACTOR;
    let thin_height = (SCALING_FACTOR.powf(2.0) - thin_width.powf(2.0)).sqrt();
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
        Tile::D => (Vec3::new(-thin_height * 0.5, -thin_width * 0.5, 0.0), 0.0),
        Tile::E => (Vec3::new(-thin_height * 0.5, -thin_width * 0.5, 0.0), 0.0),
        _ => panic!("Invalid tile"),
    }
}
