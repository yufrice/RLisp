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

use crate::syntax::ast::SExp;

#[derive(Debug, PartialEq)]
pub(crate) enum OP {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ScopeType {
    Closure,
    Local,
    Global,
}

pub struct Generator {
    pub(crate) context: Context,
    pub(crate) module: Module,
    pub(crate) jit_module: Option<Module>,
    pub(crate) builder: Builder,
    pub(crate) stack_pointer: RefCell<Vec<BasicValueEnum>>,
    pub(crate) func_dic: RefCell<HashMap<String, FunctionValue>>,
    pub(crate) env_dic: RefCell<HashMap<String, BasicValueEnum>>,
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

impl Generator {
    pub fn new() -> Generator {
        let context = Context::create();
        Generator {
            module: context.create_module("RLISP"),
            jit_module: None,
            builder: context.create_builder(),
            context,
            stack_pointer: RefCell::new(Vec::new()),
            func_dic: RefCell::new(HashMap::new()),
            env_dic: RefCell::new(HashMap::new()),
        }
    }

    pub fn jit_env(&mut self, module: Module) {
        self.jit_module = Some(module);
    }

    pub fn init(&mut self) {
        // buildin
        self.std_entry().expect("init");
        self.symbol_entry();

        // main func
        let fn_type = self.context.void_type().fn_type(&[], false);
        let func = self.create_function(&"main".to_string(), fn_type);
        let entry = self.create_entry(func, "entry");
        self.builder.position_at_end(&entry);
        let ret = self.builder.build_return(None);
        self.builder.position_before(&ret);
    }

    pub fn get_module(&self) -> &Module {
        match self.jit_module {
            Some(ref module) => module,
            None => &self.module,
        }
    }

    fn create_function(&mut self, name: &str, ty: FunctionType) -> FunctionValue {
        let func = self.get_module().add_function(name, ty, None);
        self.func_dic.borrow_mut().insert(name.to_string(), func);
        func
    }

    fn create_entry(&self, fun: FunctionValue, name: &str) -> BasicBlock {
        self.context.append_basic_block(&fun, name)
    }

    pub fn create_engine(&self) -> Result<ExecutionEngine, LLVMString> {
        self.get_module()
            .create_jit_execution_engine(OptimizationLevel::None)
    }

    pub fn jit_eval(&self, ast: &SExp) -> Result<FunctionValue, &'static str> {
        let func_type = FloatType::f64_type().fn_type(&[], false);
        let func = self.get_module().add_function("lambda", func_type, None);
        let bb = self.context.append_basic_block(&func, "entry");

        let expr = self.expr(ast)?;
        self.builder.position_at_end(&bb);
        self.builder.build_return(Some(&expr));
        Ok(func)
    }

    pub fn expr(&self, ast: &SExp) -> Result<BasicValueEnum, &'static str> {
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
}
