#![feature(test)]
extern crate rlisp;

#[cfg(test)]
mod tests {
    use rlisp::syntax::ast::*;

    fn as_str<T: std::fmt::Display>(val: T) -> String {
        format!("{}", val)
    }

    #[test]
    fn atom_display() {
        // Value
        let test_symbol = String::from("SYMBOL");
        let symbol = DataType::Symbol(test_symbol.clone());

        let test_number = -31319.;
        let number = DataType::Number(test_number);

        assert_eq!(test_symbol, as_str(symbol.clone()));
        assert_eq!(as_str(test_number), as_str(number));
    }

    #[test]
    fn exrp_display() {
        // Value
        let test_symbol = String::from("SYMBOL");
        let symbol = DataType::Symbol(test_symbol.clone());

        let test_number = -31319.;
        let number = DataType::Number(test_number);

        // Expr
        let atom = SExp::Atom(symbol.clone());
        let dotted = SExp::Dotted(Box::new(atom.clone()), Box::new(atom.clone()));
        let list = SExp::List(vec![atom.clone(), atom.clone(), atom.clone()]);

        assert_eq!(test_symbol, as_str(atom.clone()));
        assert_eq!(
            format!("( {} . {} )", atom.clone(), atom.clone()),
            as_str(dotted)
        );
        assert_eq!(
            format!("( {} {} {} )", atom.clone(), atom.clone(), atom.clone()),
            as_str(list)
        );
    }

    #[test]
    fn expr_method() {
        // make nil
        let test_nil = String::from("NIL");
        assert_eq!(test_nil, as_str(SExp::new_nil()));

        // symbol check
        let call_symbol = SExp::Atom(DataType::Symbol("add".to_string()));
        let not_call_symbol = SExp::Atom(DataType::Number(0.));
        assert!(call_symbol.is_call().is_some());
        assert!(not_call_symbol.is_call().is_none());
    }
}
