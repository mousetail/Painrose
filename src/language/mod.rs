mod instructions;
mod stack_item;
use std::collections::HashMap;

use crate::geometry::tiling::All;
use crate::geometry::tiling::{TileCoordinate, Tiling};

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
    pub fn step(&mut self) {
        let instuction = self.code.get(&self.instruction_pointer);

        let behavior = if let Some((ch, Some(instruction))) = instuction {
            instruction.evaluate(&mut self.mode, &mut self.stack)
        } else {
            InstructionPointerBehavior::Straight
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
    pub fn new_from_string(source_code: String) -> Self {
        let mut code = HashMap::<TileCoordinate<T>, (char, Option<Instruction>)>::new();

        let mut next_position = TileCoordinate::<T>::new(vec![T::Tile::all()[0]]).unwrap();
        for char in source_code.chars() {
            code.insert(next_position.clone(), (char, Instruction::from_char(char)));
            next_position = next_position.next();
        }

        LanguageState {
            code,
            instruction_pointer: TileCoordinate::new(vec![]).unwrap(),
            direction: T::Edge::all()[0],
            stack: vec![],
            mode: Mode::NormalMode,
        }
    }
}
