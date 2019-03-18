use inkwell::values::*;

use crate::compile::generator::*;
use crate::syntax::ast::{DataType, SExp};

impl Generator {
    pub(crate) fn call(&self, fun: &Vec<SExp>) -> Result<BasicValueEnum, &'static str> {
        if let Some((SExp::Atom(DataType::Symbol(s)), tail)) = fun.split_first() {
            Ok(self.find_function(s.to_string(), tail))
        } else {
            Err("inv call") // quote list
        }?
    }

    fn find_function(
        &self,
        symbol: String,
        arg: &[SExp],
        // ref mut arg: impl Iterator<Item = SExp>,
    ) -> Result<BasicValueEnum, &'static str> {
        match &symbol[..] {
            "+" => self.fold_op(OP::Add, arg),
            "-" => self.fold_op(OP::Sub, arg),
            "*" => self.fold_op(OP::Mul, arg),
            "/" => self.fold_op(OP::Div, arg),
            "def!" => self.def_var(arg),
            _ => Err("func"),
        }
    }
}
