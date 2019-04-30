use inkwell::values::*;

use crate::compile::generator::*;
use crate::syntax::ast::{DataType, SExp};

impl Generator {
    pub(crate) fn atom(&self, value: &DataType) -> Result<BasicValueEnum, &'static str> {
        match value {
            DataType::Number(value) => Ok(self.floating(*value)),
            DataType::Symbol(s) => self.symbol(s.to_string()),
        }
    }

    pub(crate) fn alloca_and_store(
        &self,
        val: &BasicValueEnum,
        symbol: String,
        scope: ScopeType,
    ) -> PointerValue {
        let typ = val.get_type();
        let ptr = self.builder.build_alloca(typ, &symbol[..]);
        self.builder.build_store(ptr, *val);
        match scope {
            ScopeType::Closure => (),
            ScopeType::Local => self.symbol_regit(symbol, *val),
            _ => unimplemented!(),
        };
        ptr
    }

    pub(crate) fn floating(&self, value: f64) -> BasicValueEnum {
        let i64_type = self.context.f64_type();
        i64_type.const_float(value).into()
    }

    pub(crate) fn symbol(&self, sym: String) -> Result<BasicValueEnum, &'static str> {
        match self.module.get_global(&sym) {
            Some(val) => Ok(val.as_basic_value_enum()),
            None =>
            match self.env_dic.borrow().get(&sym.to_uppercase()) {
                Some(val) => Ok(*val),
                None => Err("nai"),
            }
        }
    }
    pub(crate) fn str_value(&self, sym: String) -> FloatValue {
        let len = sym.len() as u32;
        let i8_type = self.context.i8_type();
        let vec_type = i8_type.vec_type(len);
        (
            unreachable!()
        )
    }
}
