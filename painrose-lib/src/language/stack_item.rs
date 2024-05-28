use itertools::{EitherOrBoth, Itertools};
use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Clone, PartialEq, Debug)]
pub enum StackItem {
    Number(f64),
    Array(Vec<StackItem>),
}

impl StackItem {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Number(k) => *k != 0.0,
            Self::Array(k) => k.len() != 0,
        }
    }

    fn apply_unary_operator<T: Fn(f64) -> f64>(self, operator: &T) -> StackItem {
        match self {
            StackItem::Number(a) => StackItem::Number(-a),
            StackItem::Array(arr) => StackItem::Array(
                arr.into_iter()
                    .map(|k| k.apply_unary_operator(operator))
                    .collect(),
            ),
        }
    }

    fn apply_binary_operator<T: Fn(f64, f64) -> f64>(
        self,
        other: StackItem,
        operator: &T,
    ) -> StackItem {
        match (self, other) {
            (StackItem::Number(a), StackItem::Number(b)) => StackItem::Number(operator(a, b)),
            (StackItem::Array(arr), StackItem::Number(b)) => StackItem::Array(
                arr.into_iter()
                    .map(|a| a.apply_binary_operator(StackItem::Number(b), operator))
                    .collect(),
            ),

            (StackItem::Number(a), StackItem::Array(arr)) => StackItem::Array(
                arr.into_iter()
                    .map(|b| StackItem::Number(a).apply_binary_operator(b, operator))
                    .collect(),
            ),
            (StackItem::Array(arr_a), StackItem::Array(arr_b)) => StackItem::Array(
                arr_a
                    .into_iter()
                    .zip_longest(arr_b)
                    .map(|pair| {
                        let (left, right) = match pair {
                            EitherOrBoth::Both(a, b) => (a, b),
                            EitherOrBoth::Left(a) => (a, StackItem::default()),
                            EitherOrBoth::Right(b) => (StackItem::default(), b),
                        };
                        left.apply_binary_operator(right, operator)
                    })
                    .collect(),
            ),
        }
    }
}

impl Default for StackItem {
    fn default() -> Self {
        StackItem::Number(0.0)
    }
}

impl PartialOrd for StackItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (StackItem::Number(a), StackItem::Number(b)) => a.partial_cmp(b),
            (StackItem::Array(a), StackItem::Array(b)) => b.partial_cmp(a),
            _ => None,
        }
    }
}

impl From<bool> for StackItem {
    fn from(item: bool) -> StackItem {
        StackItem::Number(item as u8 as f64)
    }
}

impl From<u8> for StackItem {
    fn from(item: u8) -> StackItem {
        StackItem::Number(item as f64)
    }
}

impl From<f64> for StackItem {
    fn from(item: f64) -> StackItem {
        StackItem::Number(item)
    }
}

impl Add for StackItem {
    type Output = StackItem;

    fn add(self, rhs: Self) -> Self::Output {
        self.apply_binary_operator(rhs, &|a, b| a + b)
    }
}

impl Sub for StackItem {
    type Output = StackItem;

    fn sub(self, rhs: Self) -> Self::Output {
        self.apply_binary_operator(rhs, &|a, b| a - b)
    }
}

impl Mul for StackItem {
    type Output = StackItem;

    fn mul(self, rhs: Self) -> Self::Output {
        self.apply_binary_operator(rhs, &|a, b| a * b)
    }
}

impl Div for StackItem {
    type Output = StackItem;

    fn div(self, rhs: Self) -> Self::Output {
        self.apply_binary_operator(rhs, &|a, b| a / b)
    }
}

impl Neg for StackItem {
    type Output = StackItem;

    fn neg(self) -> Self::Output {
        self.apply_unary_operator(&|a| -a)
    }
}
