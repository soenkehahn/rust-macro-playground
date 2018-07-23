#[derive(Debug)]
enum Ast {
    Lambda { parameter: String, body: Box<Ast> },
    Var { variable: String },
    AppChain { terms: Vec<Ast> },
}

fn pp(ast: Ast) -> String {
    match ast {
        Ast::Var { variable } => variable,
        Ast::Lambda { parameter, body } => format!("|{}| {}", parameter, pp(*body)),
        Ast::AppChain { terms } => {
            let mut result = terms
                .into_iter()
                .map(|term| format!("({})", pp(term)))
                .collect::<Vec<String>>();
            result.reverse();
            result.join(" ")
        }
    }
}

macro_rules! ast {
    (| $parameter:ident | $body:tt) => {
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
    (( $($t:tt)+ )) => {
        ast!($($t)+)
    };
    ($($t:tt)+) => {
        ast_h!([ ] $($t)+)
    };
}

macro_rules! ast_h {
    ([ $($stack:tt),* ] $token:tt $($rest:tt)*) => {
        ast_h!([ $token $(, $stack)* ] $($rest)*)
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
        assert_eq!(pp(ast!(|x| y)), "|x| y");
    }

    #[test]
    fn parses_applications() {
        assert_eq!(pp(ast!(x y)), "(x) (y)");
    }

    #[test]
    fn parses_parenthesized_lambdas() {
        assert_eq!(pp(ast!((|x| y))), "|x| y");
    }

    #[test]
    fn parses_complex_terms() {
        assert_eq!(pp(ast!((|x| x)(y))), "(|x| x) (y)");
    }

    #[test]
    fn parses_chains_of_applications() {
        assert_eq!(pp(ast!(a b c)), "(a) (b) (c)");
    }
}
