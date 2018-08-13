#![macro_use]

macro_rules! ast {
    (# $parameter:ident -> $($body:tt)+) => {
        Ast::Lambda {
            parameter: stringify!($parameter).to_string(),
            body: Box::new(ast!($($body)+)),
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
        _ast_app_chain!([ ] $a $($rest)+)
    };
    (( $($a:tt)+ ) $($rest:tt)+) => {
        _ast_app_chain!([ ] ( $($a)+ ) $($rest)+)
    }
}

macro_rules! _ast_app_chain {
    ([ $($stack:tt),* ] $token:tt $($rest:tt)*) => {
        _ast_app_chain!([ $($stack, )* $token ] $($rest)*)
    };
    ([ $($stack:tt),* ]) => {
        {
            Ast::AppChain{
                terms: vec![$(ast!($stack)),*]
            }
        }
    };
}

#[cfg(test)]
mod test {
    use std::fmt::Display;
    use Ast;

    fn pretty<A: Display>(a: A) -> String {
        format!("{}", a)
    }

    #[test]
    fn parses_variables() {
        assert_eq!(pretty(ast!(x)), "x");
    }

    #[test]
    fn parses_parenthesis() {
        assert_eq!(pretty(ast!((x))), "x");
    }

    #[test]
    fn parses_lambdas() {
        assert_eq!(pretty(ast!(#x -> y)), "#x -> y");
    }

    #[test]
    fn parses_applications() {
        assert_eq!(pretty(ast!(x y)), "x y");
    }

    #[test]
    fn parses_parenthesized_lambdas() {
        assert_eq!(pretty(ast!((#x -> y))), "#x -> y");
    }

    #[test]
    fn parses_complex_terms() {
        assert_eq!(pretty(ast!((#x -> x) y)), "(#x -> x) y");
    }

    #[test]
    fn parses_chains_of_applications() {
        assert_eq!(pretty(ast!(a b c)), "a b c");
    }

    #[test]
    fn parses_functions_with_multiple_arguments() {
        assert_eq!(pretty(ast!(#x -> #y -> x y)), "#x -> #y -> x y");
    }
}
