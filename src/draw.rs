// TODO: Fix the D rhomb, and the angles
use crate::tiling::{TileCoordinate, Tiling};
use glam::f32::*;
use std::io::Write;

pub trait DrawableTileCachedData: Copy + Clone {
    fn new() -> Self;
}

pub trait DrawableTile {
    type Data: DrawableTileCachedData;
    const SCALING_FACTOR: f32;

    fn get_coordinate(self, data: Self::Data) -> (Vec3, f32);
    fn get_shape(self, data: Self::Data) -> Vec<Vec3>;
    fn get_color(self) -> &'static str;
}

const MIN_ITERATIONS: usize = 0;

#[allow(unused)]
pub fn draw_svg<T: Tiling>(coords: Vec<TileCoordinate<T>>)
where
    T::Tile: DrawableTile,
{
    let colors = ["red", "green", "blue", "cyan", "magenta", "#888800"];

    let shape_info = <T::Tile as DrawableTile>::Data::new();

    let mut file = std::fs::File::create("Tiles.svg").unwrap();

    file.write_all(b"<svg version=\"1.1\" height=\"1024\" width=\"1024\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-12 -12 24 24\">\n").unwrap();

    for (_index, coord) in coords.into_iter().enumerate() {
        for j in [0] {
            //}, 1, 2, 3, 4, 5] {
            writeln!(
                file,
                "<path fill=\"{}\" stroke-width=\"{}\" d=\"M {} Z\" alt=\"{coord:?}\"/>",
                coord.get_at(j).get_color(),
                if j == 0 { 0.025 } else { 0.0128 },
                &draw(&coord, j, shape_info)
                    .into_iter()
                    .map(|c| format!("L {} {} ", c.0, -c.1))
                    .collect::<String>()[1..]
            )
            .unwrap();
        }
    }

    writeln!(file, "<circle cx=\"{}\" cy=\"{}\" r=\"0.05\"/>", 0.0, 0.0).unwrap();

    file.write_all(b"</svg>").unwrap();
}

fn draw<T: Tiling>(
    coordinate: &TileCoordinate<T>,
    offset: usize,
    shape_info: <T::Tile as DrawableTile>::Data,
) -> Vec<(f32, f32)>
where
    T::Tile: DrawableTile,
{
    let mut matrix = glam::f32::Mat4::IDENTITY;
    let reference = TileCoordinate::<T>::new(vec![]).unwrap();
    for index in offset..coordinate.len().max(MIN_ITERATIONS) {
        matrix = get_matrix(coordinate, index, shape_info) * matrix;
    }

    for i in (0..coordinate.len().max(MIN_ITERATIONS).max(offset)).rev() {
        matrix = get_matrix(&reference, i, shape_info).inverse() * matrix;
    }

    let shape = coordinate.get_at(offset).get_shape(shape_info);

    return shape
        .into_iter()
        .map(|i| matrix.transform_point3(i * T::Tile::SCALING_FACTOR.powi(offset as i32)))
        .map(|i| (i.x, i.y))
        .collect();
}

fn get_matrix<T: Tiling>(
    coordinate: &TileCoordinate<T>,
    index: usize,
    shape_info: <T::Tile as DrawableTile>::Data,
) -> Affine3A
where
    T::Tile: DrawableTile,
{
    let (position, angle) = get_offset_and_rotation(coordinate.get_at(index), shape_info);

    Affine3A::from_translation(position * T::Tile::SCALING_FACTOR.powi(index as i32 + 1))
        * Affine3A::from_quat(Quat::from_rotation_z(angle))
}

fn get_offset_and_rotation<T: DrawableTile>(tile: T, shape_info: T::Data) -> (Vec3, f32) {
    tile.get_coordinate(shape_info)
}
