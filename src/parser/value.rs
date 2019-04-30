extern crate regex;

use combine::error::ParseError;
use combine::parser::char::{digit, letter, spaces};
use combine::{choice, from_str, many1, optional, satisfy, skip_many, token};
use combine::{Parser, Stream};
use regex::Regex;

use crate::syntax::ast::DataType;

pub fn atom<I>() -> impl Parser<Input = I, Output = DataType>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    optional(spaces()).with(choice((number(), sign(), symbol(None)))).skip(spaces())
}

fn number<I>() -> impl Parser<Input = I, Output = DataType>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    floating().map(|v| DataType::Number(v))
}

fn integer<I>() -> impl Parser<Input = I, Output = i64>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    from_str(many1::<String, _>(digit()))
}

fn floating<I>() -> impl Parser<Input = I, Output = f64>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    fn length(f: f64) -> f64 {
        (f.log10() + 1.0).floor()
    };
    (integer(), optional(token('.').with(integer())))
        .map(|(n, e)| (n as f64, e.map(|v| v as f64)))
        .map(|(n, e)| match e {
            Some(v) => n + v * (10.0 as f64).powf(-length(v)),
            None => n,
        })
}

fn sign<I>() -> impl Parser<Input = I, Output = DataType>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    token('-').with(choice((
        floating().map(|v| DataType::Number(-v)),
        symbol(Some('-')),
        spaces().map(|_| DataType::Symbol("-".to_string())),
    )))
}

fn symbol<I>(head: Option<char>) -> impl Parser<Input = I, Output = DataType>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (many1(letter().or(buildin_symbol())).skip(spaces()))
        .map(move |t: String| head.map_or(format!("{}", t), |h| format!("{}{}", h, t)))
        .map(|t: String| DataType::Symbol(t))
}

fn buildin_symbol<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let reg = Regex::new(r#"[\s\[\]{}()'"`,;]"#).unwrap();
    satisfy(move |t: char| !reg.is_match(&t.to_string()))
}

pub fn lexer<I>(c: char) -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    optional(spaces()).with(token(c))
}
