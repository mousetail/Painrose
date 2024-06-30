mod draw;
mod error;
mod instructions;
mod stack_item;

use std::io::Read;
use std::str::FromStr;
use std::{collections::HashMap, io::Write};
use strum::VariantArray;

use crate::geometry::draw::{DrawableTile, ShapeInfo};
use crate::geometry::tile_coordinate::TileCoordinate;
use crate::geometry::tiling::Tiling;

use self::instructions::{Instruction, InstructionPointerBehavior, Mode};

pub trait FollowableDirection: Copy {
    fn turn_left(self) -> Self;
    fn turn_right(self) -> Self;
    fn opposite(self) -> Self;
}

#[derive(Clone, Debug)]
pub struct LanguageState<T: Tiling>
where
    T::Edge: FollowableDirection,
{
    code: HashMap<TileCoordinate<T>, (char, Option<Instruction>)>,
    instruction_pointer: TileCoordinate<T>,
    direction: T::Edge,
    stack: Vec<stack_item::StackItem>,
    mode: Mode,
}

impl<T: Tiling> LanguageState<T>
where
    T::Edge: FollowableDirection,
{
    #[allow(unused)]
    pub fn step<Out: Write, In: Read>(&mut self, out: &mut Out, input: &mut In) {
        let instuction = self.code.get(&self.instruction_pointer);

        let get_instruction_char_or_default =
            || stack_item::StackItem::Number(instuction.map(|t| t.0 as u32 as f64).unwrap_or(0.0));

        let behavior = match &mut self.mode {
            Mode::NormalMode => {
                if let Some((ch, Some(instruction))) = instuction {
                    instruction.evaluate(&mut self.mode, &mut self.stack, out, input)
                } else {
                    InstructionPointerBehavior::Straight
                }
            }
            Mode::CharMode => {
                self.mode = Mode::NormalMode;
                self.stack.push(get_instruction_char_or_default());
                InstructionPointerBehavior::Straight
            }
            Mode::ArrayStringMode(e) => {
                if let Some(Instruction::StartArrayString) = instuction.and_then(|k| k.1) {
                    self.stack.push(stack_item::StackItem::Array(e.clone()));
                    self.mode = Mode::NormalMode;
                } else {
                    e.push(get_instruction_char_or_default());
                };
                InstructionPointerBehavior::Straight
            }
            Mode::CharStringMode(e) => {
                if let Some(Instruction::StartCharacterString) = instuction.and_then(|k| k.1) {
                    self.stack.extend(e.iter().cloned());
                    self.mode = Mode::NormalMode;
                } else {
                    e.push(get_instruction_char_or_default());
                };
                InstructionPointerBehavior::Straight
            }
            _ => InstructionPointerBehavior::Straight,
        };

        let next_direction = match behavior {
            InstructionPointerBehavior::Straight => self.direction,
            InstructionPointerBehavior::Left => self.direction.turn_left(),
            InstructionPointerBehavior::Right => self.direction.turn_right(),
            InstructionPointerBehavior::Back => self.direction.opposite(),
        };

        let next_position = self.instruction_pointer.go(next_direction).unwrap();
        self.instruction_pointer = next_position.0;
        self.direction = next_position.1.opposite();
    }

    #[allow(unused)]
    pub fn new_from_string(source_code: String) -> Result<Self, error::ParseError>
    where
        TileCoordinate<T>: FromStr,
        T::Edge: FromStr,
    {
        let mut program = HashMap::<TileCoordinate<T>, (char, Option<Instruction>)>::new();
        for (line_number, line) in source_code.lines().enumerate() {
            let Some((coordinate, code)) = line.split_once(':') else {
                return Err(error::ParseError {
                    line: line_number,
                    column: 0,
                    kind: error::ParseErrorKind::InvalidPrefixError,
                });
            };

            let (coordinate, mut direction) = match coordinate.split_once('-') {
                Some((coodinate, direction)) => (
                    coodinate,
                    direction.parse().map_err(|e| error::ParseError {
                        line: line_number,
                        column: coodinate.len(),
                        kind: error::ParseErrorKind::BadDirectionError,
                    })?,
                ),
                None => (coordinate, T::Edge::VARIANTS[0]),
            };

            let mut coordinate: TileCoordinate<T> = match coordinate.parse() {
                Ok(coord) => coord,
                Err(e) => {
                    return Err(error::ParseError {
                        line: line_number,
                        column: 0,
                        kind: error::ParseErrorKind::BadCoordinateError,
                    })
                }
            };

            for (index, char) in code.chars().enumerate() {
                while program.contains_key(&coordinate) {
                    (coordinate, direction) = match coordinate.go(direction) {
                        Ok((coordinate, direction)) => (coordinate, direction.opposite()),
                        Err(e) => {
                            return Err(error::ParseError {
                                line: line_number,
                                column: index,
                                kind: error::ParseErrorKind::BadCoordinateError,
                            })
                        }
                    };
                }

                program.insert(coordinate.clone(), (char, Instruction::from_char(char)));
            }
        }

        Ok(LanguageState {
            code: program,
            instruction_pointer: TileCoordinate::new(vec![]).unwrap(),
            direction: T::Edge::VARIANTS[0],
            stack: vec![],
            mode: Mode::NormalMode,
        })
    }

    pub fn is_running(&self) -> bool {
        match self.mode {
            Mode::Stopped => false,
            _ => true,
        }
    }
}

impl<T: Tiling> LanguageState<T>
where
    T::Tile: DrawableTile,
    T::Edge: FollowableDirection,
{
    #[allow(unused)]
    pub fn draw(&self) -> std::io::Result<()> {
        super::language::draw::draw(self)
    }

    pub fn get_shapes(&self) -> Vec<ShapeInfo> {
        super::language::draw::get_shapes(self)
    }
}
