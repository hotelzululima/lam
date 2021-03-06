/// This module defines the core literal values that the LAM can operate on.
///
/// It also defines operations on them, such as formatting for pretty printing,
/// equalities, and (de)serializations.
///
use num_bigint::BigInt;
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type Label = u32;
pub type Arity = u32;
pub type Atom = String;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
/// An opaque reference to a runtime and platform specific value that can not
/// be inspected.
///
/// This value can't be sent over the wire meaningfully since it does not
/// actually contain the data it references.
pub struct Ref {
    pub tag: u128,
    pub id: u128,
}

impl Display for Ref {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "ref#{}.{}", self.tag, self.id)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct Lambda {
    /// The first label to execute when the Lambda runs.
    pub first_label: Label,

    /// The module in which the first label should be found
    pub module: Atom,

    /// The amount of values captured when the Lambda was created.
    pub environment: Vec<Literal>,

    /// The amount of arguments this lambda takes
    pub arity: Arity,
}

impl Display for Lambda {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "fun {}/{}", self.first_label, self.arity)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)]
#[repr(C)]
pub struct Pid {
    pub scheduler_id: u32,
    pub process_id: u64,
    is_main: bool,
}

impl Display for Pid {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "<{:?}.{:?}.0>", self.scheduler_id, self.process_id)
    }
}

impl Pid {
    pub fn new(scheduler_id: u32, process_id: u64) -> Pid {
        Pid {
            scheduler_id,
            process_id,
            is_main: scheduler_id == 0 && process_id == 0,
        }
    }

    pub fn is_main(&self) -> bool {
        self.is_main
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub struct Tuple {
    pub size: u32,
    pub elements: Vec<Literal>,
}

impl Display for Tuple {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{{")?;
        for e in self.clone().elements {
            write!(fmt, "{}, ", e)?;
        }
        write!(fmt, "}}")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
/// NOTE(@ostera): consider reusing an existing implementation of cons lists
pub enum List {
    Nil,
    Cons(Box<Literal>, Box<List>),
}

impl Into<Vec<Literal>> for List {
    fn into(self) -> Vec<Literal> {
        match self {
            List::Nil => vec![],
            List::Cons(head, tail) => {
                let tail: Vec<Literal> = (*tail).into();
                let res: Vec<Literal> = vec![*head].iter().chain(tail.iter()).cloned().collect();
                res
            }
        }
    }
}

impl Display for List {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "[")?;
        let elements: Vec<Literal> = self.clone().into();
        for e in elements {
            write!(fmt, "{}, ", e)?;
        }
        write!(fmt, "]")
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[repr(C)]
pub enum Literal {
    /// NOTE(@ostera): consider interning atoms so this becomes Atom(u32) instead
    Atom(Atom),

    /// NOTE(@ostera): consider interning binaries into a table so that this particular Literal is
    /// either Binary(InternedRef(u32) | ActualString(String))
    Binary(String),
    Bool(bool),
    Character(u8),
    Float(f64),
    Integer(BigInt),
    Lambda(Lambda),
    List(List),
    Pid(Pid),
    Tuple(Tuple),
    Ref(Ref),
    // Map(Map),
}

impl Into<String> for Literal {
    fn into(self) -> String {
        match self {
            Literal::Binary(str) => str,
            x => format!("{}", x),
        }
    }
}

impl Into<Ref> for Literal {
    fn into(self) -> Ref {
        match self {
            Literal::Ref(r) => r,
            _ => panic!("Could not turn {:?} into a Ref", self),
        }
    }
}

impl Into<BigInt> for Literal {
    fn into(self) -> BigInt {
        match self {
            Literal::Integer(bi) => bi,
            Literal::Float(f) => BigInt::from_f64(f).unwrap(),
            _ => panic!("Could not turn {:?} into a BigInt", self),
        }
    }
}

impl Into<Vec<Literal>> for Literal {
    fn into(self) -> Vec<Literal> {
        match self {
            Literal::List(l) => l.into(),
            _ => panic!("Could not turn {:?} into a Vec", self),
        }
    }
}
impl Display for Literal {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Literal::Atom(atom) => write!(fmt, "{}", atom),
            Literal::Binary(bin) => write!(fmt, "<<{:?}>>", bin),
            Literal::Bool(b) => write!(fmt, "{}", b),
            Literal::Character(char) => write!(fmt, "'{}'", char),
            Literal::Float(f) => write!(fmt, "{}", f),
            Literal::Integer(i) => write!(fmt, "{}", i.to_string()),
            Literal::Lambda(l) => write!(fmt, "{}", l),
            Literal::List(l) => write!(fmt, "{}", l),
            Literal::Pid(p) => write!(fmt, "{}", p),
            Literal::Ref(r) => write!(fmt, "{}", r),
            Literal::Tuple(t) => write!(fmt, "{}", t),
        }
    }
}
