mod parser;

use std::fmt::Display;

#[derive(Debug, Clone)]
enum Ast {
    Lambda { parameter: String, body: Box<Ast> },
    Var { variable: String },
    AppChain { terms: Vec<Ast> },
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn wrap_in_parens(s: String) -> String {
            if s.contains(" ") {
                format!("({})", s)
            } else {
                s
            }
        }
        let result = match self {
            Ast::Var { variable } => variable.clone(),
            Ast::Lambda { parameter, body } => format!("#{} -> {}", parameter, body),
            Ast::AppChain { terms } => {
                let mut result = terms
                    .into_iter()
                    .map(|term| wrap_in_parens(format!("{}", term)))
                    .collect::<Vec<String>>();
                result.join(" ")
            }
        };
        write!(f, "{}", result)
    }
}

fn main() {
    let term = ast!((#x -> x) y);
    println!("ast!({}) == {:?}", term, term);
}
