// TODO: Fix the D rhomb, and the angles
use super::{tile_coordinate::TileCoordinate, tiling::Tiling};
use glam::f32::*;
use std::io::Write;

pub trait DrawableTile {
    const SCALING_FACTOR: f32;

    fn get_coordinate(self) -> (Vec2, f32);
    fn get_shape(self) -> Vec<Vec2>;
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

impl<T: Tiling> TileDrawOptions<T>
where
    T::Tile: DrawableTile,
{
    pub fn get_shape(self) -> ShapeInfo {
        let matrix = get_matrix(&self.coordinate, self.supertile_index);

        let outline = self
            .coordinate
            .get_at(self.supertile_index)
            .get_shape()
            .into_iter()
            .map(|point| matrix.transform_point2(point))
            .collect();

        let center = matrix.transform_point2(Vec2 { x: 0.0, y: 0.0 });

        let color = self.coordinate.get_at(self.supertile_index).get_color();

        return ShapeInfo {
            outline,
            center,
            label: self.label,
            fill: self.fill.then(|| color),
            stroke: (!self.fill).then(|| color),
            stroke_width: if !self.fill { 0.025 } else { 0.0 },
            text_color: if self.fill { "white" } else { "black" },
        };
    }
}

pub struct ShapeInfo {
    pub outline: Vec<glam::Vec2>,
    pub center: glam::Vec2,
    pub label: String,
    pub fill: Option<&'static str>,
    pub stroke: Option<&'static str>,
    pub stroke_width: f32,
    pub text_color: &'static str,
}

impl ShapeInfo {
    pub fn get_outline_d(&self) -> String {
        let mut out = self
            .outline
            .iter()
            .enumerate()
            .map(|(index, transformed_point)| {
                format!(
                    "{} {} {} ",
                    if index == 0 { "M" } else { "L" },
                    transformed_point.x,
                    transformed_point.y
                )
            })
            .collect::<String>();
        out.push('Z');
        return out;
    }
}

#[allow(unused)]
pub fn draw_svg(coords: Vec<ShapeInfo>) -> std::io::Result<()> {
    let mut file = std::fs::File::create("Tiles.svg")?;

    file.write_all(b"<svg version=\"1.1\" height=\"1024\" width=\"1024\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"-12 -12 24 24\">\n")?;

    for shape_info in coords.into_iter() {
        writeln!(
            file,
            "<path stroke=\"{}\" fill=\"{}\" stroke-width=\"{}\" d=\"{} Z\"/>",
            shape_info.fill.unwrap_or("none"),
            shape_info.stroke.unwrap_or("none"),
            shape_info.stroke_width,
            shape_info.get_outline_d()
        )?;

        let base_position = shape_info.center;
        if shape_info.label.len() > 0 {
            writeln!(
                file,
                "<text font-family=\"sans-serif\" font-size=\"0.5\" x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"middle\" fill=\"{}\"><![CDATA[{}]]></text>",
                base_position.x, base_position.y, shape_info.text_color, shape_info.label.replace("]]>", "]]>]]&gt;<!CDATA[")
            )?;
        }
    }

    file.write_all(b"</svg>")?;

    Ok(())
}

fn get_matrix<T: Tiling>(coordinate: &TileCoordinate<T>, offset: usize) -> Affine2
where
    T::Tile: DrawableTile,
{
    let mut matrix = glam::f32::Affine2::IDENTITY;
    let reference = TileCoordinate::<T>::new(vec![]).unwrap();
    for index in offset..coordinate.len().max(MIN_ITERATIONS) {
        matrix = get_level_matrix(coordinate, index) * matrix;
    }

    for i in (0..coordinate.len().max(MIN_ITERATIONS).max(offset)).rev() {
        matrix = get_level_matrix(&reference, i).inverse() * matrix;
    }

    //let shape = coordinate.get_at(offset).get_shape(shape_info);

    return Affine2::from_scale(T::Tile::SCALING_FACTOR.powi(offset as i32) * Vec2::ONE) * matrix;
}

fn get_level_matrix<T: Tiling>(coordinate: &TileCoordinate<T>, index: usize) -> Affine2
where
    T::Tile: DrawableTile,
{
    let (position, angle) = get_offset_and_rotation(coordinate.get_at(index));

    Affine2::from_translation(position * T::Tile::SCALING_FACTOR.powi(index as i32 + 1))
        * Affine2::from_angle(angle)
}

fn get_offset_and_rotation<T: DrawableTile>(tile: T) -> (Vec2, f32) {
    tile.get_coordinate()
}
