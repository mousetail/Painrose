// TODO: Fix the D rhomb, and the angles
use crate::penrose::{Tile, TileReference};
use glam::f32::*;
use std::{f32::consts, io::Write};

const SCALING_FACTOR: f32 = 1.618033988;
const SCALING_FACTOR_INVERSE: f32 = 1.0 / SCALING_FACTOR;

#[derive(Copy, Clone, Debug)]
struct ShapeInfo {
    width: f32,
    height: f32,
    side_length: f32,
    bottom_angle: f32,
    side_angle: f32,
}

impl ShapeInfo {
    fn new(angle: f32) -> ShapeInfo {
        let angle_radians = angle.to_radians();

        let width = (angle_radians * 0.5).cos() * SCALING_FACTOR_INVERSE * 2.0;
        let height = (angle_radians * 0.5).sin() * SCALING_FACTOR_INVERSE * 2.0;

        ShapeInfo {
            width,
            height,
            side_length: SCALING_FACTOR_INVERSE,
            side_angle: angle_radians,
            bottom_angle: consts::PI - angle_radians,
        }
    }
}

#[derive(Clone, Copy, Debug)]

struct AllShapeInfos {
    thin_rhomb: ShapeInfo,
    thick_rhomb: ShapeInfo,
}

impl AllShapeInfos {
    fn new() -> AllShapeInfos {
        AllShapeInfos {
            thin_rhomb: ShapeInfo::new(36.0),
            thick_rhomb: ShapeInfo::new(108.0),
        }
    }
}

const MIN_ITERATIONS: usize = 0;

pub fn draw_svg(coords: Vec<TileReference>) {
    let colors = [
        "red", "green", "blue", "cyan", "magenta", "orange", "purple", "grey",
    ];
    let shape_info = AllShapeInfos::new();
    println!("{shape_info:?}");

    let mut file = std::fs::File::create("Tiles.svg").unwrap();

    file.write_all(b"<svg version=\"1.1\" height=\"512\" width=\"512\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-4 -4 8 8\">\n").unwrap();

    for (index, coord) in coords.into_iter().enumerate() {
        for j in [2] {
            writeln!(
                file,
                "<path fill=\"none\" stroke=\"{}\" stroke-width=\"0.025\" d=\"M {} Z\" alt=\"{coord:?}\"/>",
                if j==0{colors[index%colors.len()]} else { ["#00000044", "#22882244"][index % 2] },
                &draw(&coord, j, shape_info)
                    .into_iter()
                    .map(|c| format!("L {} {} ", c.0 , c.1 ))
                    .collect::<String>()[1..]
            )
            .unwrap();
        }
    }

    writeln!(
        file,
        "<circle cx=\"{}\" cy=\"{}\" r=\"0.05\"/>",
        shape_info.thick_rhomb.width / 2.0,
        shape_info.thick_rhomb.height / 2.0
    )
    .unwrap();

    file.write_all(b"</svg>").unwrap();
}

fn draw(coordinate: &TileReference, offset: usize, shape_info: AllShapeInfos) -> Vec<(f32, f32)> {
    let mut matrix = glam::f32::Mat4::IDENTITY;
    let reference = TileReference(vec![]);
    for index in offset..coordinate.0.len().max(MIN_ITERATIONS) {
        matrix = get_matrix(coordinate, index, shape_info) * matrix;
    }
    println!("{matrix:?}");

    for i in (0..coordinate.0.len().max(MIN_ITERATIONS)).rev() {
        matrix = get_matrix(&reference, i, shape_info).inverse() * matrix;
    }

    let rhomb = match coordinate.get_at(0) {
        Tile::A | Tile::C | Tile::E => shape_info.thick_rhomb,
        Tile::D | Tile::B => shape_info.thin_rhomb,
    };

    let coordinates = vec![
        Vec3::new(0.0, rhomb.height / 2.0, 0.0),
        Vec3::new(-rhomb.width / 2.0, 0.0, 0.0),
        Vec3::new(0.0, -rhomb.height / 2.0, 0.0),
        Vec3::new(rhomb.width / 2.0, 0.0, 0.0),
        Vec3::new(0.0, rhomb.height / 2.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
    ];

    return coordinates
        .into_iter()
        .map(|i| matrix.transform_point3(i * SCALING_FACTOR.powi(offset as i32)))
        .map(|i| (i.x, i.y))
        .collect();
}

fn get_matrix(coordinate: &TileReference, index: usize, shape_info: AllShapeInfos) -> Affine3A {
    let (position, angle) = get_offset_and_rotation(coordinate.get_at(index), shape_info);

    Affine3A::from_translation(position * SCALING_FACTOR.powi(index as i32 + 1))
        * Affine3A::from_quat(Quat::from_rotation_z(angle))
}

fn get_offset_and_rotation(tile: Tile, shape_info: AllShapeInfos) -> (Vec3, f32) {
    match tile {
        Tile::A => (
            Vec3::new(
                0.0,
                (shape_info.thick_rhomb.height - SCALING_FACTOR_INVERSE) * 0.5,
                0.0,
            ),
            consts::PI,
        ),
        Tile::B => (
            Vec3::new(
                (-shape_info.thick_rhomb.bottom_angle / 2.0 + consts::FRAC_PI_2).sin()
                    * shape_info.thin_rhomb.width
                    * SCALING_FACTOR_INVERSE
                    / 2.0,
                (-shape_info.thick_rhomb.bottom_angle / 2.0 + consts::FRAC_PI_2).cos()
                    * shape_info.thin_rhomb.width
                    * SCALING_FACTOR_INVERSE
                    / 2.0
                    + 0.5
                    - SCALING_FACTOR_INVERSE,
                0.0,
            ),
            -shape_info.thick_rhomb.side_angle / 2.0 + consts::FRAC_PI_2,
        ),
        Tile::C => (
            Vec3::new(
                shape_info.thick_rhomb.width / 4.0,
                -shape_info.thick_rhomb.height / 4.0,
                0.0,
            ),
            -shape_info.thick_rhomb.bottom_angle / 2.0 + consts::PI,
        ),
        Tile::D => {
            let scaling_factor = shape_info.thin_rhomb.height / 2.0;
            assert_eq!(
                shape_info.thin_rhomb.height * SCALING_FACTOR_INVERSE
                    + SCALING_FACTOR_INVERSE * SCALING_FACTOR_INVERSE,
                SCALING_FACTOR_INVERSE
            );
            (
                Vec3::new(
                    -shape_info.thin_rhomb.width * 0.5 * scaling_factor,
                    shape_info.thin_rhomb.height * 0.5 * (1.0 - scaling_factor),
                    0.0,
                ),
                shape_info.thin_rhomb.side_angle / 2.0 + consts::FRAC_PI_2,
            )
        }
        Tile::E => (
            Vec3::new(
                -shape_info.thin_rhomb.width * 0.25,
                -shape_info.thin_rhomb.height * 0.25,
                0.0,
            ),
            shape_info.thin_rhomb.bottom_angle / 2.0 + consts::PI,
        ),
        _ => panic!("Invalid tile"),
    }
}
