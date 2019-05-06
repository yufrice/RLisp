use inkwell::module::Linkage;
use inkwell::values::{BasicValueEnum, FloatValue, PointerValue};
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

    pub(crate) fn alloca_and_store(
        &self,
        val: &BasicValueEnum,
        symbol: String,
        scope: ScopeType,
    ) -> PointerValue {
        match scope {
            ScopeType::Local => {
                let typ = val.get_type();
                let ptr = self.builder.build_alloca(typ, &symbol[..]);
                self.builder.build_store(ptr, *val);
                ptr
            }
            ScopeType::Closure => unimplemented!(),
            ScopeType::Global => self.add_global_variable(symbol, *val),
        }
    }

    pub(crate) fn add_global_variable(&self, symbol: String, val: BasicValueEnum) -> PointerValue {
        let ptr_type = val.get_type();
        let ptr = self.module.add_global(
            ptr_type,
            Some(AddressSpace::Shared),
            &symbol.to_ascii_uppercase(),
        );
        ptr.set_linkage(Linkage::External);
        ptr.set_initializer(val.as_float_value());
        ptr.as_pointer_value()
    }

    pub(crate) fn floating(&self, value: f64) -> BasicValueEnum {
        let i64_type = self.context.f64_type();
        i64_type.const_float(value).into()
    }

    pub(crate) fn symbol(&self, sym: String) -> Result<BasicValueEnum, &'static str> {
        let symbol = sym.to_ascii_uppercase();
        match self.module.get_global(&symbol) {
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
