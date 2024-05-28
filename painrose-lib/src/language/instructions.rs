use std::io::Write;

use strum::EnumString;

use super::stack_item::StackItem;

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumString)]
pub enum Instruction {
    // Control Flow
    #[strum(serialize = "<")]
    TurnLeft,
    #[strum(serialize = ">")]
    TurnRight,
    #[strum(serialize = "^")]
    TurnLeftIf,
    #[strum(serialize = "v")]
    TurnRightIf,
    #[strum(serialize = "|")]
    TurnAround,
    #[strum(serialize = "(")]
    Less,
    #[strum(serialize = ")")]
    Greater,
    #[strum(serialize = "=")]
    Equal,
    // Stack
    #[strum(serialize = ":")]
    Duplicate,
    #[strum(serialize = "#")]
    DuplicateTwo,
    #[strum(serialize = "~")]
    PopTop,
    #[strum(serialize = "s")]
    Swap,
    #[strum(serialize = "{")]
    RotateLeft,

    #[strum(serialize = "}")]
    RotateRight,

    #[strum(serialize = "d")]
    DuplicateN,
    #[strum(serialize = "c")]
    CopyNth,
    #[strum(serialize = "u")]
    UnwrapArray,
    #[strum(serialize = "a")]
    WrapArray,
    // String
    #[strum(serialize = "'")]
    StartCharacterString,
    #[strum(serialize = "\"")]
    StartArrayString,
    #[strum(serialize = "`")]
    StartCharacter,
    // Constants
    Const(u8),
    // Math
    #[strum(serialize = "+")]
    Add,
    #[strum(serialize = "-")]
    Subtract,
    #[strum(serialize = "*")]
    Multiply,
    #[strum(serialize = "/")]
    Divide,
    #[strum(serialize = "_")]
    Negate,
    // Input
    #[strum(serialize = "i")]
    InputCharacter,
    #[strum(serialize = "w")]
    InputLine,
    #[strum(serialize = "l")]
    InputWord,
    #[strum(serialize = "n")]
    InputNumber,
    // Output
    #[strum(serialize = "I")]
    OutputCharacter,
    #[strum(serialize = "W")]
    OutputN,
    #[strum(serialize = "N")]
    OutputNumber,
    // Array
    #[strum(serialize = "[")]
    GetArrayN,
    #[strum(serialize = "]")]
    PutArrayN,
    // Exit
    #[strum(serialize = ";")]
    Quit,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum InstructionPointerBehavior {
    Straight,
    Left,
    Right,
    Back,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Mode {
    NormalMode,
    CharStringMode(Vec<StackItem>),
    ArrayStringMode(Vec<StackItem>),
    CharMode,
    Stopped,
}

fn top_of_stack_or_default(stack: &mut Vec<StackItem>) -> StackItem {
    stack.pop().unwrap_or_default()
}

fn top_two_of_stack_or_default(stack: &mut Vec<StackItem>) -> (StackItem, StackItem) {
    (
        top_of_stack_or_default(stack),
        top_of_stack_or_default(stack),
    )
}

fn copy_nth(stack: &mut Vec<StackItem>, n: StackItem) -> StackItem {
    match n {
        StackItem::Array(arr) => {
            StackItem::Array(arr.into_iter().map(|b| copy_nth(stack, b)).collect())
        }
        StackItem::Number(num) => {
            let num = num as usize;
            if num < stack.len() {
                stack[stack.len() - num - 1].clone()
            } else {
                StackItem::default()
            }
        }
    }
}

fn array_wrap(stack: &mut Vec<StackItem>, n: StackItem) -> StackItem {
    match n {
        StackItem::Number(n) => {
            let n = n as usize;
            let mut arr = (0..n)
                .map(|_| top_of_stack_or_default(stack))
                .collect::<Vec<_>>();
            arr.reverse();

            StackItem::Array(arr)
        }
        StackItem::Array(arr) => StackItem::Array(
            arr.into_iter()
                .map(|item| array_wrap(stack, item))
                .collect(),
        ),
    }
}

impl Instruction {
    pub fn is_nonconditional_movement_instruction(self) -> bool {
        match self {
            Self::TurnLeft | Self::TurnRight | Self::TurnAround => true,
            _ => false,
        }
    }

    pub fn from_char(item: char) -> Option<Self> {
        let mut tmp = [0u8; 4];
        let string = item.encode_utf8(&mut tmp);
        match string.parse() {
            Ok(k) => Some(k),
            Err(_) => match item {
                '0' => Some(Self::Const(0)),
                '1' => Some(Self::Const(1)),
                '2' => Some(Self::Const(2)),
                '3' => Some(Self::Const(3)),
                '4' => Some(Self::Const(4)),
                '5' => Some(Self::Const(5)),
                '6' => Some(Self::Const(6)),
                '7' => Some(Self::Const(7)),
                '8' => Some(Self::Const(8)),
                '9' => Some(Self::Const(9)),
                _ => None,
            },
        }
    }

    #[allow(unused)]
    pub fn evaluate<Out: Write>(
        self,
        mode: &mut Mode,
        stack: &mut Vec<StackItem>,
        out: &mut Out,
    ) -> InstructionPointerBehavior {
        let mut behavior = InstructionPointerBehavior::Straight;
        match self {
            Instruction::TurnLeft => behavior = InstructionPointerBehavior::Left,
            Instruction::TurnRight => behavior = InstructionPointerBehavior::Right,
            Instruction::TurnLeftIf => {
                if stack.pop().is_some_and(|k| k.is_truthy()) {
                    behavior = InstructionPointerBehavior::Left
                }
            }
            Instruction::TurnRightIf => {
                if stack.pop().is_some_and(|k| k.is_truthy()) {
                    behavior = InstructionPointerBehavior::Right
                }
            }
            Instruction::TurnAround => behavior = InstructionPointerBehavior::Back,
            Instruction::Less => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push((a > b).into())
            }
            Instruction::Greater => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push((a < b).into())
            }
            Instruction::Equal => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push((a == b).into())
            }
            Instruction::Duplicate => {
                let a = top_of_stack_or_default(stack);
                stack.push(a.clone());
                stack.push(a);
            }
            Instruction::DuplicateTwo => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(b.clone());
                stack.push(a.clone());
                stack.push(b);
                stack.push(a);
            }
            Instruction::PopTop => {
                top_of_stack_or_default(stack);
            }
            Instruction::Swap => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(b);
                stack.push(a);
            }
            Instruction::RotateLeft => stack.rotate_left(1),
            Instruction::RotateRight => stack.rotate_right(1),
            Instruction::DuplicateN => {
                let number_of_stack_items_to_copy = top_of_stack_or_default(stack);
                match number_of_stack_items_to_copy {
                    StackItem::Array(_arr) => (), // todo: Think of something sensible to do in this case,
                    StackItem::Number(num) => {
                        let num = num as usize;

                        let mut top_n_of_stack: Vec<StackItem> =
                            (0..num).map(|_| top_of_stack_or_default(stack)).collect();
                        top_n_of_stack.reverse();
                        stack.extend(top_n_of_stack.clone());
                        stack.extend(top_n_of_stack);
                    }
                }
            }
            Instruction::CopyNth => {
                let n = top_of_stack_or_default(stack);
                let value = copy_nth(stack, n);
                stack.push(value)
            }
            Instruction::UnwrapArray => match top_of_stack_or_default(stack) {
                StackItem::Number(n) => stack.push(StackItem::Number(n)),
                StackItem::Array(arr) => arr
                    .into_iter()
                    .for_each(|array_item| stack.push(array_item)),
            },
            Instruction::WrapArray => {
                let n = top_of_stack_or_default(stack);
                let wrapped_stack_item = array_wrap(stack, n);
                stack.push(wrapped_stack_item);
            }
            Instruction::StartCharacterString => *mode = Mode::CharStringMode(vec![]),
            Instruction::StartArrayString => *mode = Mode::ArrayStringMode(vec![]),
            Instruction::StartCharacter => *mode = Mode::CharMode,
            Instruction::Const(i) => stack.push(i.into()),
            Instruction::Add => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(a + b);
            }
            Instruction::Subtract => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(a - b);
            }
            Instruction::Multiply => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(a * b);
            }
            Instruction::Divide => {
                let (a, b) = top_two_of_stack_or_default(stack);
                stack.push(a / b);
            }
            Instruction::Negate => {
                let m = top_of_stack_or_default(stack);
                stack.push(-m);
            }
            Instruction::InputCharacter => todo!(),
            Instruction::InputLine => todo!(),
            Instruction::InputWord => todo!(),
            Instruction::InputNumber => todo!(),
            Instruction::OutputCharacter => {
                let top = top_of_stack_or_default(stack);
                top.for_each_recusrive(&mut |k|write!(out, "{}", (k as u32).try_into().unwrap_or('?')).unwrap())
            }
            Instruction::OutputN => {
                let n = top_of_stack_or_default(stack);

                n.for_each_recusrive(&mut |k|{
                    let n=k as usize;
                    for _ in 0..n {
                        let top = top_of_stack_or_default(stack);
                        top.for_each_recusrive(&mut |k|writeln!(out, "{}", (k as u32).try_into().unwrap_or('?')).unwrap())
                    }
                })
            },
            Instruction::OutputNumber => {
                let top = top_of_stack_or_default(stack);
                top.for_each_recusrive(&mut |k|write!(out, "{} ", k).unwrap())
            },
            Instruction::GetArrayN => {
                let n = top_of_stack_or_default(stack);
                let array = top_of_stack_or_default(stack);

                match array {
                    StackItem::Number(n) => (),
                    StackItem::Array(arr) => {
                        stack.push(n.apply_unary_operator(&|k|arr[k as usize].clone()));
                    }
                }
            },
            Instruction::PutArrayN => {
                let value = top_of_stack_or_default(stack);
                let n = top_of_stack_or_default(stack);
                let array = top_of_stack_or_default(stack);

                match array {
                    StackItem::Number(n) => (),
                    StackItem::Array(mut arr) => {
                        n.for_each_recusrive(&mut |k|arr[k as usize] = value.clone());
                        stack.push(StackItem::Array(arr));
                    }
                }
            },

            Instruction::Quit => *mode = Mode::Stopped,
        }
        return behavior;
    }
}
