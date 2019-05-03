use auto_enums::auto_enum;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::passes::PassManager;
use inkwell::types::FloatType;
use inkwell::OptimizationLevel;
use inkwell::values::*;
use inkwell::AddressSpace;
use std::fmt::Display;

use crate::compile::generator::Generator;
use crate::syntax::ast::*;

pub struct Engine {
    engine: ExecutionEngine,
    generator: Generator,
}

impl Engine {
    pub fn new() -> Option<Engine> {
        let mut generator = Generator::new();
        generator.init();
        let fpm = PassManager::create_for_function(&generator.get_module());
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.initialize();
        match generator.create_engine() {
            Ok(eng) => Some(Engine {
                engine: eng,
                generator: generator,
            }),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }


    pub fn eval(&mut self, ast: Option<SExp>) -> Result<(), &str> {
        match ast {
            None => Ok(println!()),
            Some(ast) => {
                //self.generator.module.print_to_stderr();
                let module = self.generator.context.create_module("tmp");
                let engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
                self.generator.jit_env(module);

                self.generator.jit_eval(&ast)?;

                let ret = unsafe {
                    let func = engine.get_function::<unsafe extern "C" fn() -> f64>("lambda").unwrap();
                    func.call()
                };
/*                 let func = self.generator.jit_eval(&ast)?;
                let mut ret = unsafe {
                    func.print_to_stderr();
                    self.engine.run_function(&func, &[])
                };
                println!("{}", ret.as_float(&FloatType::f64_type())); */
                //let res = self.generator.expr(&ast)?;
                //let ref val = self.printer(&res);
                //println!("{}", val);
                println!("{:?}", ret);
                Ok(())
            }
        }
    }

    #[auto_enum(Display)]
    fn printer<T: BasicValue>(&self, val: &T) -> impl Display {
        match val.as_basic_value_enum() {
            BasicValueEnum::IntValue(v) => {
                if v.is_null() {
                    "F"
                } else {
                    "T"
                }
            }
            BasicValueEnum::FloatValue(v) => format!("{:?}", v),
            BasicValueEnum::FloatValue(v) => v.get_constant().map(|(v, _)| v).unwrap(),
            BasicValueEnum::VectorValue(ref v) => {
                v.get_string_constant().to_str().unwrap().to_owned()
            }
            BasicValueEnum::PointerValue(val) => match val.is_null() {
                true => "NIL",
                false => unimplemented!(),
            },
            _ => unreachable!(),
        }
    }
}
