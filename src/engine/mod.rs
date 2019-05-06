use inkwell::OptimizationLevel;

use crate::compile::generator::Generator;
use crate::syntax::ast::*;

#[derive(Default)]
pub struct Engine {
    generator: Generator,
}

impl Engine {
    pub fn new() -> Engine {
        let mut generator = Generator::new();
        generator.init();
        Engine { generator }
    }

    pub fn eval(&mut self, ast: Option<SExp>) -> Result<(), &str> {
        match ast {
            None => Ok(println!()),
            Some(ast) => {
                let module = self.generator.context.create_module("tmp");
                let engine = module
                    .create_jit_execution_engine(OptimizationLevel::None)
                    .or(Err("Failed create engine"))?;
                engine
                    .add_module(&self.generator.module)
                    .or(Err("Failed add module"));

                self.generator.jit_env(Some(module));
                self.generator.jit_eval(&ast)?;

                let ret = unsafe {
                    let func = engine
                        .get_function::<unsafe extern "C" fn() -> f64>("lambda")
                        .or(Err(""))?;
                    func.call()
                };
                println!("{:?}", ret);

                self.generator.jit_env(None);
                engine
                    .remove_module(&self.generator.module)
                    .or(Err("Failed remove module"));
                Ok(())
            }
        }
    }
}
