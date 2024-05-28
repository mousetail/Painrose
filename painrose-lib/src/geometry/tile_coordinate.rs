
use super::tiling::{EdgeDefinitionType, RelativeDirection, Tiling};
use std::str::FromStr;
use strum::VariantArray;

#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct CoordinateTraversalError<T> {
    pub inner_tile: T,
    pub outer_tile: T,
}

#[derive(Copy, Clone, PartialEq, Debug, Hash)]
pub struct CoordinateParsingError<T> {
    index: usize,
    kind: CoordinateParsingErrorKind<T>
}

#[derive(Copy, Clone, PartialEq, Debug, Hash)]
enum CoordinateParsingErrorKind<T> {
    TraversalError(CoordinateTraversalError<T>),
    ParseError(char)
}

pub struct TileCoordinate<T: Tiling>(Vec<T::Tile>);

impl<T: Tiling> std::fmt::Debug for TileCoordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TileCoordinate").field(&self.0).finish()
    }
}

impl<T: Tiling> Clone for TileCoordinate<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: Tiling> PartialEq for TileCoordinate<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Tiling> Eq for TileCoordinate<T> {}

impl<T: Tiling> std::hash::Hash for TileCoordinate<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: Tiling> FromStr for TileCoordinate<T>
    where T::Tile: TryFrom<char, Error = char> {
    type Err = CoordinateParsingError<T::Tile>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return TileCoordinate::new(s.chars().enumerate().map(|(index, c)|T::Tile::try_from(c).map_err(|c|CoordinateParsingError{
                index,
                kind: CoordinateParsingErrorKind::ParseError(c)
            })).collect::<Result<Vec<_>, CoordinateParsingError<T::Tile>>>()?).map_err(|e|CoordinateParsingError {
                index: 0,
                kind: CoordinateParsingErrorKind::TraversalError(e)
            })
    }
}

impl<T: Tiling> TileCoordinate<T> {
    pub fn new(tiles: Vec<T::Tile>) -> Result<Self, CoordinateTraversalError<T::Tile>> {
        let mut item = Self(tiles);
        item.normalize()?;

        return Ok(item);
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn get_at(&self, index: usize) -> T::Tile {
        if index < self.0.len() {
            return self.0[index];
        }
        return T::TILE_PATTERN[index % T::TILE_PATTERN.len()];
    }

    fn set_at_unchecked(&mut self, index: usize, tile: T::Tile) {
        while index >= self.0.len() {
            self.0
                .push(T::TILE_PATTERN[self.0.len() % T::TILE_PATTERN.len()]);
        }

        self.0[index] = tile;

        while self.0.len() > 0
            && self.0[self.0.len() - 1]
                == T::TILE_PATTERN[(self.0.len() - 1) % T::TILE_PATTERN.len()]
        {
            self.0.pop();
        }
    }

    pub fn set_at(
        &mut self,
        index: usize,
        tile: T::Tile,
    ) -> Result<(), CoordinateTraversalError<T::Tile>> {
        if index > 0 {
            T::can_tile_fit_in_tile(self.get_at(index - 1), tile)?;
        }
        T::can_tile_fit_in_tile(tile, self.get_at(index + 1))?;

        self.set_at_unchecked(index, tile);

        Ok(())
    }

    fn normalize(&mut self) -> Result<(), CoordinateTraversalError<T::Tile>> {
        while self.0.len() > 0
            && self.0[self.0.len() - 1]
                == T::TILE_PATTERN[(self.0.len() - 1) % T::TILE_PATTERN.len()]
        {
            self.0.pop();
        }

        for i in 0..self.0.len() {
            let next_tile = *self
                .0
                .get(i + 1)
                .unwrap_or(&T::TILE_PATTERN[(i + 1) % T::TILE_PATTERN.len()]);

            T::can_tile_fit_in_tile(self.0[i], next_tile)?
        }

        Ok(())
    }

    pub fn go(&self, edge: T::Edge) -> Result<(Self, T::Edge), CoordinateTraversalError<T::Tile>> {
        let mut copy = self.clone();

        let mut definition = T::get_internal_edge_definition(self.get_at(0), edge);

        let mut sides: Vec<Vec<RelativeDirection>> = vec![];
        let mut index = 0;

        loop {
            match definition.edge_type {
                EdgeDefinitionType::Inside(tile, direction) => {
                    copy.set_at_unchecked(index, tile);
                    if index == 0 {
                        return Ok((copy, direction));
                    } else {
                        index -= 1;
                        definition =
                            T::get_external_edge_definition(tile, direction, sides.pop().unwrap())
                    }
                }
                EdgeDefinitionType::Outside(direction) => {
                    sides.push(definition.direction.iter().map(|i| i.invert()).collect());
                    index += 1;
                    definition = T::get_internal_edge_definition(copy.get_at(index), direction)
                }
            }
        }
    }

    pub fn next(&self) -> Self {
        let mut copy = self.0.clone();
        let options = <T as Tiling>::Tile::VARIANTS;

        loop {
            let mut index = 0;
            loop {
                while copy.len() <= index {
                    copy.push(T::TILE_PATTERN[copy.len() % T::TILE_PATTERN.len()])
                }

                let item = *(copy.as_slice().get(index).unwrap());
                let option_index = <T as Tiling>::Tile::VARIANTS.into_iter().position(|k|*k==item).unwrap();
                if option_index < options.len() - 1 {
                    copy[index] = options[option_index + 1];
                    break;
                } else {
                    copy[index] = options[0];
                    index += 1;
                }
            }

            match Self::new(copy.clone()) {
                Ok(k) => break k,
                Err(_e) => (),
            }
        }
    }
}
