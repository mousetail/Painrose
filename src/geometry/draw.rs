// TODO: Fix the D rhomb, and the angles
use super::tiling::{TileCoordinate, Tiling};
use glam::f32::*;
use std::io::Write;

pub trait DrawableTileCachedData: Copy + Clone {
    fn new() -> Self;
}

pub trait DrawableTile {
    type Data: DrawableTileCachedData;
    const SCALING_FACTOR: f32;

    fn get_coordinate(self, data: Self::Data) -> (Vec2, f32);
    fn get_shape(self, data: Self::Data) -> Vec<Vec2>;
    fn get_color(self) -> &'static str;
}

const MIN_ITERATIONS: usize = 0;

pub struct TileDrawOptions<T: Tiling>
where
    T::Tile: DrawableTile,
{
    pub coordinate: TileCoordinate<T>,
    pub fill: bool,
    pub label: String,
    pub supertile_index: usize,
}

#[allow(unused)]
pub fn draw_svg<T: Tiling>(coords: Vec<TileDrawOptions<T>>) -> std::io::Result<()>
where
    T::Tile: DrawableTile,
{
    let shape_info = <T::Tile as DrawableTile>::Data::new();

    let mut file = std::fs::File::create("Tiles.svg")?;

    file.write_all(b"<svg version=\"1.1\" height=\"1024\" width=\"1024\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-12 -12 24 24\">\n")?;

    for (_index, draw_options) in coords.into_iter().enumerate() {
        let matrix = get_matrix(
            &draw_options.coordinate,
            draw_options.supertile_index,
            shape_info,
        );

        writeln!(
            file,
            "<path stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\" d=\"M {} Z\"/>",
            if draw_options.fill {
                "none"
            } else {
                draw_options
                    .coordinate
                    .get_at(draw_options.supertile_index)
                    .get_color()
            },
            if !draw_options.fill {
                "none"
            } else {
                draw_options
                    .coordinate
                    .get_at(draw_options.supertile_index)
                    .get_color()
            },
            if !draw_options.fill { 0.025 } else { 0.0 },
            &draw_options
                .coordinate
                .get_at(draw_options.supertile_index)
                .get_shape(shape_info)
                .into_iter()
                .enumerate()
                .map(|(index, point)| {
                    let transformed_point = matrix.transform_point2(point);
                    format!(
                        "{} {} {} ",
                        if index == 0 { "M" } else { "L" },
                        transformed_point.x,
                        transformed_point.y
                    )
                })
                .collect::<String>()[1..]
        )?;

        let base_position = matrix.transform_point2(Vec2 { x: 0.0, y: 0.0 });
        if draw_options.label.len() > 0 {
            writeln!(
                file,
                "<text font-family=\"sans-serif\" font-size=\"1\" x=\"{}\" y=\"{}\"><![CDATA[{}]]></text>",
                base_position.x, base_position.y, draw_options.label.replace("]]>", "]]>]]&gt;<!CDATA[")
            )?;
        }
    }

    file.write_all(b"</svg>")?;

    Ok(())
}

fn get_matrix<T: Tiling>(
    coordinate: &TileCoordinate<T>,
    offset: usize,
    shape_info: <T::Tile as DrawableTile>::Data,
) -> Affine2
where
    T::Tile: DrawableTile,
{
    let mut matrix = glam::f32::Affine2::IDENTITY;
    let reference = TileCoordinate::<T>::new(vec![]).unwrap();
    for index in offset..coordinate.len().max(MIN_ITERATIONS) {
        matrix = get_level_matrix(coordinate, index, shape_info) * matrix;
    }

    for i in (0..coordinate.len().max(MIN_ITERATIONS).max(offset)).rev() {
        matrix = get_level_matrix(&reference, i, shape_info).inverse() * matrix;
    }

    //let shape = coordinate.get_at(offset).get_shape(shape_info);

    return Affine2::from_scale(T::Tile::SCALING_FACTOR.powi(offset as i32) * Vec2::ONE) * matrix;
}

fn get_level_matrix<T: Tiling>(
    coordinate: &TileCoordinate<T>,
    index: usize,
    shape_info: <T::Tile as DrawableTile>::Data,
) -> Affine2
where
    T::Tile: DrawableTile,
{
    let (position, angle) = get_offset_and_rotation(coordinate.get_at(index), shape_info);

    Affine2::from_translation(position * T::Tile::SCALING_FACTOR.powi(index as i32 + 1))
        * Affine2::from_angle(angle)
}

fn get_offset_and_rotation<T: DrawableTile>(tile: T, shape_info: T::Data) -> (Vec2, f32) {
    tile.get_coordinate(shape_info)
}
