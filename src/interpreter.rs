use crate::{Compile, parser::ast::{Node, Operator}};

pub struct Interpreter;

impl Compile for Interpreter {
    type Output = anyhow::Result<i32>;
    
    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0i32;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }
}

struct Eval;


impl Eval {
    pub fn new() -> Self {
        Self
    }
    
    pub fn eval(&self, node: &Node) -> i32 {
         match node {
            Node::Int(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);
                match op {
                    Operator::Plus => child,
                    Operator::Minus => -child,
                    _ => panic!("Syntax Error"),
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Plus => lhs_ret + rhs_ret,
                    Operator::Minus => lhs_ret - rhs_ret,
                    Operator::Multiply => lhs_ret * rhs_ret,
                    Operator::Divide => lhs_ret / rhs_ret,
                    Operator::Pow => lhs_ret.checked_pow(rhs_ret as u32).expect("Overflow while multiplying to the power"),
                }
            }
        }
    }
}