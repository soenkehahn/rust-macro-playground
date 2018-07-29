#![macro_use]

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Ast {
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

impl Display for Ast {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fn wrap_in_parens<T: Display>(a: T) -> String {
            let s = format!("{}", a);
            if s.contains(" ") {
                format!("({})", s)
            } else {
                s
            }
        }
        let result = match self {
            Ast::Var { variable } => variable.clone(),
            Ast::Lambda { parameter, body } => format!("#{} -> {}", parameter, body),
            Ast::App { function, argument } => {
                format!("{} {}", wrap_in_parens(function), wrap_in_parens(argument))
            }
        };
        write!(f, "{}", result)
    }
}

pub fn from_app_chain(mut terms: Vec<Ast>) -> Ast {
    terms.reverse();
    match terms.pop() {
        None => panic!(),
        Some(first) => {
            let mut result = first;
            terms.reverse();
            for i in terms {
                result = Ast::App {
                    function: Box::new(result),
                    argument: Box::new(i),
                };
            }
            result
        }
    }
}

macro_rules! ast {
    (# $parameter:ident -> $($body:tt)+) => {
        Ast::Lambda {
            parameter: stringify!($parameter).to_string(),
            body: Box::new(ast!($($body)+)),
            // body: Box::new(from_app_chain(_ast_app_chain!([ ] $($body)+))),
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
        from_app_chain(_ast_app_chain!([ ] $a $($rest)+))
    };
    (( $($a:tt)+ ) $($rest:tt)+) => {
        from_app_chain(_ast_app_chain!([ ] ( $($a)+ ) $($rest)+))
    }
}

macro_rules! _ast_app_chain {
    ([ $($stack:tt),* ] $token:tt $($rest:tt)*) => {
        _ast_app_chain!([ $($stack, )* $token ] $($rest)*)
    };
    ([ $($stack:tt),* ]) => {
        {
            vec![$(ast!($stack)),*]
        }
    };
}

#[cfg(test)]
mod test {
    use parser::*;

    impl Ast {
        pub fn pretty(self) -> String {
            format!("{}", self)
        }
    }

    #[test]
    fn parses_variables() {
        assert_eq!(ast!(x).pretty(), "x");
    }

    #[test]
    fn parses_parenthesis() {
        assert_eq!(ast!((x)).pretty(), "x");
    }

    #[test]
    fn parses_lambdas() {
        assert_eq!(ast!(#x -> y).pretty(), "#x -> y");
    }

    #[test]
    fn parses_applications() {
        assert_eq!(ast!(x y).pretty(), "x y");
    }

    #[test]
    fn parses_parenthesized_lambdas() {
        assert_eq!(ast!((#x -> y)).pretty(), "#x -> y");
    }

    #[test]
    fn parses_complex_terms() {
        assert_eq!(ast!((#x -> x) y).pretty(), "(#x -> x) y");
    }

    #[test]
    fn parses_chains_of_applications() {
        assert_eq!(ast!(a b c).pretty(), "(a b) c");
    }

    #[test]
    fn parses_functions_with_multiple_arguments() {
        assert_eq!(ast!(#x -> #y -> x y).pretty(), "#x -> #y -> x y");
    }

    #[test]
    fn parses_chains_of_applications_as_lambda_bodies() {
        assert_eq!(ast!(# x -> a b).pretty(), "#x -> a b");
    }
}
