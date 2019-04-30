use inkwell::memory_buffer::MemoryBuffer;
use inkwell::module::Module;
use inkwell::values::*;
use inkwell::AddressSpace;

use crate::compile::generator::*;
use crate::syntax::ast::SExp;

impl Generator {
    pub(crate) fn std_entry(&self) -> Result<(), String> {
        // 遅延評価にしとく

        // std output
        use std::path::Path;
        let path = Path::new("lib/stdio.bc").to_owned();
        let buf = MemoryBuffer::create_from_file(&path).map_err(|e| e.to_string())?;
        Module::parse_bitcode_from_buffer(&buf)
            .map(|m| self.module.link_in_module(m).map_err(|e| e.to_string()))
            .map_err(|e| e.to_string())?;

        // llvm func
        let i8_type_ptr = self.context.i8_type().ptr_type(AddressSpace::Local);
        let void_type = self.context.void_type();
        let fn_stack_type = i8_type_ptr.fn_type(&[], false);
        let fn_pop_type = void_type.fn_type(&[i8_type_ptr.into()], false);
        self.module
            .add_function("llvm.stacksave", fn_stack_type, None);
        self.module
            .add_function("llvm.stackrestore", fn_pop_type, None);
        self.module
            .get_function("llvm.stacksave")
            .map(|f| self.func_regit("stacksave".to_string(), f))
            .ok_or_else(|| "buildin:24".to_string())?;
        self.module
            .get_function("llvm.stackrestore")
            .map(|f| self.func_regit("stackrestore".to_string(), f))
            .ok_or_else(|| "buildin:24".to_string())
    }

    pub(crate) fn symbol_regit(&self, symbol: String, value: BasicValueEnum) {
        self.env_dic
            .borrow_mut()
            .insert(symbol.to_uppercase(), value);
    }

    pub(crate) fn func_regit(&self, symbol: String, value: FunctionValue)  {
        self.func_dic
            .borrow_mut()
            .insert(symbol.to_uppercase(), value);
    }

    pub(crate) fn symbol_entry(&self)  {
        // nil -> NIL
        let nil_val = self
            .context
            .void_type()
            .ptr_type(AddressSpace::Generic)
            .const_null();
        nil_val.set_name("nil");
        self.symbol_regit("NIL".to_string(), nil_val.into());

        // t -> True
        let true_val = self.context.bool_type().const_all_ones();
        true_val.set_name("true");
        self.symbol_regit("T".to_string(), true_val.into());

        // f -> False
        let false_val = self.context.bool_type().const_zero();
        false_val.set_name("false");
        self.symbol_regit("F".to_string(), false_val.into());
    }

    pub(crate) fn let_local(&self, arg: &[SExp]) -> Result<BasicValueEnum, &'static str> {
        info!("let");
        if let Some((SExp::List(s), tail)) = arg.split_first() {
            println!("head: {:?}", s);
            println!("tail: {:?}", tail);
            self.llvm_stacksave();
            self.def_var(s, ScopeType::Closure)?;
            let val = self.expr(&tail[0]);
            self.llvm_stackrestore();
            self.stack_pointer.borrow_mut().pop();
            println!("{:?}", self.stack_pointer.borrow());
            val
        } else {
            Err("inv call")
        }
    }

    pub fn llvm_stacksave(&self) {
        let func: FunctionValue = *self
            .func_dic
            .borrow()
            .get(&"stacksave".to_ascii_uppercase())
            .expect("nai");
        let adr = self.builder.build_call(func, &[], "");
        adr.try_as_basic_value()
            .left()
            .map(|a| self.stack_pointer.borrow_mut().push(a))
            .expect("");
    }

    pub fn llvm_stackrestore(&self) {
        let adr = self.stack_pointer.borrow_mut().pop().expect("");
        let func: FunctionValue = *self
            .func_dic
            .borrow()
            .get(&"stackrestore".to_ascii_uppercase())
            .expect("");
        self.builder.build_call(func, &[adr], "");
    }

    pub(crate) fn fold_op(
        &self,
        op: OP,
        // ref mut arg: impl Iterator<Item = SExp>,
        arg: &[SExp],
        // arg: Vec<SExp>,
    ) -> Result<BasicValueEnum, &'static str> {
        let append = |e: FloatValue, v: FloatValue| -> FloatValue {
            match op {
                OP::Add => e.const_add(v),
                OP::Sub => e.const_sub(v),
                OP::Mul => e.const_mul(v),
                OP::Div => e.const_div(v),
            }
        };

        let allow_type = |v: BasicValueEnum| -> Result<FloatValue, &'static str> {
            match v {
                BasicValueEnum::FloatValue(v) => Ok(v),
                _ => unreachable!(),
            }
        };

        // let mut iter = arg.flat_map(|r| self.expr(r)).by_ref();
        (arg.iter().by_ref().nth(0).ok_or(""))
            .map(|r| self.expr(r))?
            .map(allow_type)?
            .map(|l| {
                arg.iter()
                    .skip(1)
                    .flat_map(|r| self.expr(r))
                    .flat_map(allow_type)
                    .fold(l, append)
            })
            .map(|v| v.as_basic_value_enum())
    }

    pub(crate) fn def_var(
        &self,
        value: &[SExp],
        scope: ScopeType,
    ) -> Result<BasicValueEnum, &'static str> {
        let mut itr = value.iter();
        let symbol = itr.next().ok_or("").map(|s| s.is_call().ok_or(""))?;
        let rhs = itr.next().map(|v| self.expr(v)).ok_or("damene")?;
        self.alloca_and_store(&rhs?, symbol?, scope);
        itr.next().map_or(Ok(rhs?), |_| Err(""))
    }
}
