use crate::interpreter::Interpreter;

pub mod parser;
pub mod interpreter;

pub trait Compile {
    type Output;

    fn from_ast(ast: Vec<parser::ast::Node>) -> Self::Output;

    fn from_source(source: &str) -> Self::Output {
        //println!("Compiling the source: {}", source);
        let ast: Vec<parser::ast::Node> = parser::parse(source).unwrap();
        println!("{:?}", ast);
        Self::from_ast(ast)
    }
}

fn main() {
    println!("{:#?}", Interpreter::from_source("40").unwrap());
}