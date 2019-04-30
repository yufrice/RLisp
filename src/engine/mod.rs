use auto_enums::auto_enum;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::passes::PassManager;
use inkwell::values::*;
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

    pub fn eval(&self, ast: Option<SExp>) -> Result<(), &str> {
        match ast {
            None => Ok(println!("")),
            Some(ast) => {
                let res = self.generator.expr(&ast)?;
                let ref val = self.printer(&res);
                println!("{}", &val);
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
