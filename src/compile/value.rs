use inkwell::values::{BasicValueEnum, FloatValue};
use inkwell::AddressSpace;

use crate::compile::generator::*;
use crate::syntax::ast::DataType;

impl Generator {
    pub(crate) fn atom(&self, value: &DataType) -> Result<BasicValueEnum, &'static str> {
        match value {
            DataType::Number(value) => Ok(self.floating(*value)),
            DataType::Symbol(s) => self.symbol(s.to_string()),
        }
    }

    pub(crate) fn alloca_and_store(&self, val: &BasicValueEnum, symbol: String, scope: ScopeType) {
        match scope {
            ScopeType::Local => {
                let typ = val.get_type();
                let ptr = self.builder.build_alloca(typ, &symbol[..]);
                self.symbol_regit(symbol, *val);
                self.builder.build_store(ptr, *val);
            }
            ScopeType::Closure => unimplemented!(),
            ScopeType::Global => {
                self.add_global_variable(symbol, *val);
            }
        };
    }

    pub(crate) fn add_global_variable(&self, symbol: String, val: BasicValueEnum) {
        let ptr_type = val.get_type();
        let ptr = self.get_module().add_global(
            ptr_type,
            Some(AddressSpace::Global),
            &symbol.to_ascii_uppercase(),
        );
        ptr.set_initializer(val.as_float_value())
    }

    pub(crate) fn floating(&self, value: f64) -> BasicValueEnum {
        let i64_type = self.context.f64_type();
        i64_type.const_float(value).into()
    }

    pub(crate) fn symbol(&self, sym: String) -> Result<BasicValueEnum, &'static str> {
        let symbol = sym.to_ascii_uppercase();
        match self.get_module().get_global(&symbol) {
            Some(val) => Ok(self.builder.build_load(val.as_pointer_value(), "")),
            None => match self.env_dic.borrow().get(&symbol) {
                Some(val) => Ok(*val),
                None => Err("nai"),
            },
        }
    }
    pub(crate) fn str_value(&self, sym: String) -> FloatValue {
        let len = sym.len() as u32;
        let i8_type = self.context.i8_type();
        let vec_type = i8_type.vec_type(len);
        (unreachable!())
    }
}
