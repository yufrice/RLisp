extern crate rlisp;

mod parse {
    use rlisp::syntax::ast::*;
    use std::boxed::Box;

    fn as_str<T: std::fmt::Display>(val: T) -> String {
        format!("{}", val)
    }
    #[test]
    fn parse() {}
}
