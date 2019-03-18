use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::support::LLVMString;
use inkwell::types::*;
use inkwell::values::*;
use inkwell::OptimizationLevel;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::syntax::ast::{DataType, SExp};

#[derive(Debug, PartialEq)]
pub(crate) enum OP {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Generator {
    pub(crate) context: Context,
    pub(crate) module: Module,
    pub(crate) builder: Builder,
    pub(crate) stack_pointer: RefCell<Vec<PointerValue>>,
    pub(crate) func_dic: RefCell<HashMap<String, FunctionValue>>,
    pub(crate) env_dic: RefCell<HashMap<String, BasicValueEnum>>,
}

impl Generator {
    pub fn new() -> Generator {
        let context = Context::create();
        Generator {
            module: context.create_module("RLISP"),
            builder: context.create_builder(),
            context: context,
            stack_pointer: RefCell::new(Vec::new()),
            func_dic: RefCell::new(HashMap::new()),
            env_dic: RefCell::new(HashMap::new()),
        }
    }

    pub fn init(&mut self) {
        // buildin
        self.std_entry().expect("init");
        self.symbol_entry();

        // main func
        let fn_type = self.context.void_type().fn_type(&[], false);
        let func = self.create_function(&"main".to_string(), fn_type);
        let entry = self.create_entry(&func, "entry");
        self.builder.position_at_end(&entry);
        let ret = self.builder.build_return(None);
        self.builder.position_before(&ret);
    }

    pub fn get_module(&self) -> &Module {
        &self.module
    }

    fn create_function(&mut self, name: &str, ty: FunctionType) -> FunctionValue {
        let func = self.module.add_function(name, ty, None);
        self.func_dic.borrow_mut().insert(name.to_string(), func);
        func
    }

    fn create_entry(&self, fun: &FunctionValue, name: &str) -> BasicBlock {
        self.context.append_basic_block(fun, name)
    }

    pub fn create_engine(&self) -> Result<ExecutionEngine, LLVMString> {
        self.module
            .create_jit_execution_engine(OptimizationLevel::None)
    }

    pub(crate) fn expr(&self, ast: &SExp) -> Result<BasicValueEnum, &'static str> {
        self.module.print_to_stderr();
        match ast {
            SExp::Atom(v) => self.atom(v),
            SExp::List(v) => match v.as_slice() {
                [_] => unimplemented!(),
                [SExp::Atom(_), ..] => self.call(v),
                _ => unimplemented!(),
            },
            _ => Err("shiran"),
        }
    }

    // pub fn get_bitcode(&self) -> () {
    //     use std::path::Path;
    //     let path = Path::new("./bc");
    //     self.module.write_bitcode_to_path(path);
    // }

    // pub fn test(&self) -> () {
    //     println!("{}", self.module.print_to_string());
    // }
}
