#![feature(trace_macros)]

use std::fmt::Display;

#[derive(Debug, Clone)]
enum Ast {
    Lambda { parameter: String, body: Box<Ast> },
    Var { variable: String },
    AppChain { terms: Vec<Ast> },
}

fn pp(ast: Ast) -> String {
    match ast {
        Ast::Var { variable } => variable.clone(),
        Ast::Lambda { parameter, body } => format!("# {} -> {}", parameter, pp(*body)),
        Ast::AppChain { terms } => {
            let mut result = terms
                .into_iter()
                .map(|term| format!("({})", pp(term)))
                .collect::<Vec<String>>();
            result.join(" ")
        }
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", pp((*self).clone()))
    }
}

macro_rules! ast {
    (# $parameter:ident -> $body:tt) => {
        Ast::Lambda {
            parameter: stringify!($parameter).to_string(),
            body: Box::new(ast!($body)),
        }
    };
    ($x:ident) => {
        Ast::Var {
            variable: stringify!($x).to_string(),
        }
    };
    ( ( $($t:tt)+ ) ) => {
        ast!($($t)+)
    };
    ($a:ident $($rest:tt)+) => {
        ast_app_chain!([ ] $a $($rest)+)
    };
    (( $($a:tt)+ ) $($rest:tt)+) => {
        ast_app_chain!([ ] ( $($a)+ ) $($rest)+)
    }
}

macro_rules! ast_app_chain {
    ([ $($stack:tt),* ] $token:tt $($rest:tt)*) => {
        ast_app_chain!([ $($stack, )* $token ] $($rest)*)
    };
    ([ $($stack:tt),* ]) => {
        {
            Ast::AppChain{
                terms: vec![$(ast!($stack)),*]
            }
        }
    };
}

fn main() {
    let term = ast!(a b c);
    println!("{:?}", term);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_variables() {
        assert_eq!(pp(ast!(x)), "x");
    }

    #[test]
    fn parses_parenthesis() {
        assert_eq!(pp(ast!((x))), "x");
    }

    #[test]
    fn parses_lambdas() {
        assert_eq!(pp(ast!(# x -> y)), "# x -> y");
    }

    #[test]
    fn parses_applications() {
        assert_eq!(pp(ast!(x y)), "(x) (y)");
    }

    #[test]
    fn parses_parenthesized_lambdas() {
        assert_eq!(pp(ast!((# x -> y))), "# x -> y");
    }

    #[test]
    fn parses_complex_terms() {
        assert_eq!(pp(ast!((# x -> x)(y))), "(# x -> x) (y)");
    }

    #[test]
    fn parses_chains_of_applications() {
        assert_eq!(pp(ast!(a b c)), "(a) (b) (c)");
    }
}
