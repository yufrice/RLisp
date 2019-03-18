use std::boxed::Box;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Symbol(String),
    Number(f64),
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Symbol(v) => write!(f, "{}", v),
            DataType::Number(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SExp {
    Atom(DataType),
    Dotted(Box<SExp>, Box<SExp>),
    List(Vec<SExp>),
}

impl SExp {
    pub fn new_nil() -> SExp {
        SExp::Atom(DataType::Symbol("NIL".to_string()))
    }

    pub fn is_call(&self) -> Option<String> {
        match self {
            SExp::Atom(DataType::Symbol(s)) => Some(s.to_string()),
            _ => None,
        }
    }
}

impl fmt::Display for SExp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SExp::Atom(v) => v.fmt(f),
            SExp::Dotted(l, r) => write!(f, "( {} . {} )", l, r),
            SExp::List(vec) => {
                write!(f, "(")?;
                for v in vec {
                    write!(f, " ")?;
                    fmt::Display::fmt(&v, f)?;
                }
                write!(f, " )")
            }
        }
    }
}
