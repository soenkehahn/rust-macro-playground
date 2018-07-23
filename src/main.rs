#[derive(Debug)]
enum Ast {
    Lambda {
        parameter: String,
        body: Box<Ast>,
    },
    Var {
        variable: String,
    },
    App {
        function: Box<Ast>,
        argument: Box<Ast>,
    },
}

fn pp(ast: Ast) -> String {
    match ast {
        Ast::Var { variable } => variable,
        Ast::Lambda { parameter, body } => format!("|{}| {}", parameter, pp(*body)),
        Ast::App { function, argument } => format!("({})({})", pp(*function), pp(*argument)),
    }
}

macro_rules! ast {
    (($($x:tt)+)) => {
        ast!($($x)+)
    };
    (@ $f:tt $x:tt) => {
        Ast::App {
            function: Box::new(ast!($f)),
            argument: Box::new(ast!($x)),
        }
    };
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
}

fn main() {
    let term = ast!(y);
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
        assert_eq!(pp(ast!(@ x (y))), "(x)(y)");
    }

    #[test]
    fn parses_parenthesized_lambdas() {
        assert_eq!(pp(ast!((|x| y))), "|x| y");
    }

    #[test]
    fn parses_complex_terms() {
        assert_eq!(pp(ast!(@ (|x| x) (y))), "(|x| x)(y)");
    }

    #[test]
    fn parses_chains_of_applications() {
        assert_eq!(pp(ast!(@ (@ a b) c)), "((a)(b))(c)");
    }
}
