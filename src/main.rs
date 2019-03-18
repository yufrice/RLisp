use combine::Parser;
use std::fs;
use std::io::{self, BufReader, Read, Write};

extern crate rlisp;
use rlisp::parser::grammer;

fn main() {
    let mut filepath = std::env::args();
    match filepath.nth(1) {
        None => repl(),
        Some(filepath) => {
            let mut file = BufReader::new(fs::File::open(&filepath).expect("file not found"));
            let mut script = Vec::new();
            file.read_to_end(&mut script).expect("failed read");
            let script = String::from_utf8(script.to_vec()).expect("nande");

            let r = grammer().easy_parse(script.as_str());
            match r {
                Ok((val, _)) => println!("{:?}", val),
                Err(err) => println!("{}", err),
            }
        }
    }
}

fn repl() {
    let mut engine = rlisp::engine::Engine::new().expect("muri");
    loop {
        print!("> ");
        io::stdout().flush().ok();

        let r = io::stdin();
        let mut buf = String::new();
        r.read_line(&mut buf).ok();
        if buf.starts_with("quit") {
            break;
        }

        let eval = grammer().easy_parse(buf.as_str());
        match eval {
            Ok((val, _)) => {
                engine.eval(val).map_err(|v| println!("err: {}", v));
            }
            Err(err) => println!("{}", err),
        }
    }
}

fn printer(put: Vec<rlisp::syntax::ast::SExp>) {
    for p in put {
        println!("{}", p);
    }
}
