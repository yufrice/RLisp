use inkwell::values::{BasicValue, BasicValueEnum};

use crate::compile::generator::*;
use crate::syntax::ast::{DataType, SExp};

// impl CodeGen {
//     pub(crate) fn init_value(&self, value: BasicValue) -> () {
//         let alloca = self.builder.build_alloca();
//         self.builder.build_store()
//     }
// }
