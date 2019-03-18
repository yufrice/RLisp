use combine::error::ParseError;
use combine::parser::char::spaces;
use combine::{attempt, choice, many1, satisfy, sep_by1, skip_many, token};
use combine::{Parser, Stream};
use std::boxed::Box;

use crate::syntax::ast::SExp;

mod value;

pub fn grammer<I>() -> impl Parser<Input = I, Output = Option<SExp>>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    ((exp().or(many1(exp()).map(SExp::List))).map(|e| Some(e))).or(whitespace().map(|_| None))
}

fn exp<I>() -> impl Parser<Input = I, Output = SExp>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((value::atom().map(|v| SExp::Atom(v)), paren()))
}

// ( -> exp | , まで読めば先読み1にできる
fn paren<I>() -> impl Parser<Input = I, Output = SExp>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    value::lexer('(').with(choice((
        value::lexer(')').map(|_| SExp::new_nil()),
        attempt(dotted()),
        list(),
    )))
}

#[inline(always)]
fn dotted<I>() -> impl Parser<Input = I, Output = SExp>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    dotted_()
}
parser! {
    #[inline(always)]
    fn dotted_[I]()(I) -> SExp
        where
            [I: Stream<Item = char>,
            I::Error: ParseError<I::Item, I::Range, I::Position>,]
            {
                (exp(), value::lexer('.'), exp(), value::lexer(')'))
                    .map(|v| SExp::Dotted(Box::new(v.0), Box::new(v.2)))
            }
}

#[inline(always)]
fn list<I>() -> impl Parser<Input = I, Output = SExp>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    list_()
}
parser! {
    #[inline(always)]
    fn list_[I]()(I) -> SExp
        where
            [I: Stream<Item = char>,
            I::Error: ParseError<I::Item, I::Range, I::Position>,]
            {
                sep_by1(exp(), spaces()).skip(value::lexer(')'))
                    .map(SExp::List)
            }
}

fn whitespace<I>() -> impl Parser<Input = I, Output = ()>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    // ToDo multiline
    (token(';'), skip_many(satisfy(|c| c != '\n'))).map(|_| ())
}
