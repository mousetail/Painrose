use super::stack_item::StackItem;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    // Control Flow
    TurnLeft,
    TurnRight,
    TurnLeftIf,
    TurnRightIf,
    TurnAround,
    Less,
    Greater,
    Equal,
    // Stack
    Duplicate,
    DuplicateTwo,
    PopTop,
    Swap,
    RotateLeft,
    RotateRight,
    DuplicateN,
    CopyNth,
    UnwrapArray,
    WrapArray,
    // String
    StartCharacterString,
    StartArrayString,
    StartCharacter,
    // Constants
    Const(u8),
    // Math
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    // Input
    InputCharacter,
    InputLine,
    InputWord,
    InputNumber,
    // Output
    OutputCharacter,
    OutputN,
    OutputNumber,
    // Array
    GetArrayN,
    PutArrayN,
    // Exit
    Quit
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
    Stopped
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
        match item {
            '<' => Some(Self::TurnLeft),
            '>' => Some(Self::TurnRight),
            '^' => Some(Self::TurnLeftIf),
            'v' => Some(Self::TurnRightIf),
            '|' => Some(Self::TurnAround),
            '(' => Some(Self::Less),
            ')' => Some(Self::Greater),
            '=' => Some(Self::Equal),
            // Stack
            ':' => Some(Self::Duplicate),
            '#' => Some(Self::DuplicateTwo),
            '~' => Some(Self::PopTop),
            's' => Some(Self::Swap),
            '{' => Some(Self::RotateLeft),
            '}' => Some(Self::RotateRight),
            'd' => Some(Self::DuplicateN),
            'c' => Some(Self::CopyNth),
            'u' => Some(Self::UnwrapArray),
            'a' => Some(Self::WrapArray),
            // Strings
            '"' => Some(Self::StartCharacterString),
            '\'' => Some(Self::StartArrayString),
            '`' => Some(Self::StartCharacter),
            // Constants
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
            // Math
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            '_' => Some(Self::Negate),
            // Input
            'i' => Some(Self::InputCharacter),
            'w' => Some(Self::InputWord),
            'l' => Some(Self::InputLine),
            'n' => Some(Self::InputNumber),
            // Output
            'I' => Some(Self::OutputCharacter),
            'W' => Some(Self::OutputN),
            'N' => Some(Self::OutputNumber),
            // Arrays
            '[' => Some(Self::GetArrayN),
            ']' => Some(Self::PutArrayN),
            // Stop
            ';' => Some(Self::Quit),
            _ => None,
        }
    }

    #[allow(unused)]
    pub fn evaluate(
        self,
        mode: &mut Mode,
        stack: &mut Vec<StackItem>,
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
            },
            Instruction::InputCharacter => todo!(),
            Instruction::InputLine => todo!(),
            Instruction::InputWord => todo!(),
            Instruction::InputNumber => todo!(),
            Instruction::OutputCharacter => todo!(),
            Instruction::OutputN => todo!(),
            Instruction::OutputNumber => todo!(),
            Instruction::GetArrayN => todo!(),
            Instruction::PutArrayN => todo!(),

            Instruction::Quit => *mode = Mode::Stopped
        }
        return behavior;
    }
}
