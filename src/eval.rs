use parser::*;
use std::collections::HashSet;

impl Ast {
    pub fn eval(mut self) -> Ast {
        let mut cont = true;
        while cont {
            self = self.make_variables_unique();
            match self.clone().step() {
                Some(new) => {
                    self = new;
                }
                None => cont = false,
            }
        }
        self
    }

    fn get_used_variables(&self) -> HashSet<String> {
        match self {
            Ast::Var { identifier } => {
                let mut result = HashSet::new();
                result.insert(identifier.identifier.clone());
                result
            }
            Ast::Lambda { parameter, body } => {
                let mut result = body.get_used_variables();
                result.insert(parameter.identifier.clone());
                result
            }
            Ast::App { function, argument } => {
                let function_identifiers = function.get_used_variables();
                let argument_identifiers = argument.get_used_variables();
                let union = function_identifiers.union(&argument_identifiers);
                union.map(|x| x.clone()).collect() //change to cloned?
            }
        }
    }

    fn make_variables_unique(self) -> Ast {
        struct AllVariables {
            counter: i32,
            used_variables: HashSet<String>,
        };
        impl Iterator for AllVariables {
            type Item = String;
            fn next(&mut self) -> Option<String> {
                let r = format!("v{}", self.counter);
                self.counter += 1;
                if self.used_variables.contains(&r) {
                    self.next()
                } else {
                    Some(r)
                }
            }
        }

        fn inner(variables: &mut AllVariables, ast: Ast) -> Ast {
            match ast {
                var @ Ast::Var { .. } => var,
                Ast::Lambda { parameter, body } => match variables.next() {
                    None => panic!("no variables left"),
                    Some(new_variable) => {
                        let new_identifier = Identifier {
                            identifier: new_variable.clone(),
                            original: parameter.original.clone(),
                        };
                        let new_body = inner(
                            variables,
                            body.replace(
                                parameter.identifier.clone(),
                                Ast::Var {
                                    identifier: new_identifier.clone(),
                                },
                            ),
                        );
                        Ast::Lambda {
                            parameter: new_identifier,
                            body: Box::new(new_body),
                        }
                    }
                },
                Ast::App { function, argument } => {
                    let new_function = inner(variables, *function);
                    let new_argument = inner(variables, *argument);
                    Ast::App {
                        function: Box::new(new_function),
                        argument: Box::new(new_argument),
                    }
                }
            }
        }
        inner(
            &mut AllVariables {
                counter: 0,
                used_variables: self.get_used_variables(),
            },
            self,
        )
    }

    fn step(self) -> Option<Ast> {
        match self {
            Ast::App { function, argument } => match (*function).clone() {
                Ast::Lambda { parameter, body } => {
                    Some(body.replace(parameter.identifier, *argument))
                }
                _ => match function.clone().step() {
                    Some(new_function) => Some(Ast::App {
                        function: Box::new(new_function),
                        argument,
                    }),
                    None => match argument.step() {
                        Some(new_argument) => Some(Ast::App {
                            function,
                            argument: Box::new(new_argument),
                        }),
                        None => None,
                    },
                },
            },
            Ast::Lambda { parameter, body } => match body.step() {
                Some(new_body) => Some(Ast::Lambda {
                    parameter,
                    body: Box::new(new_body),
                }),
                None => None,
            },
            Ast::Var { .. } => None,
        }
    }

    fn replace(self, var: String, replacement: Ast) -> Ast {
        match self {
            Ast::Var { identifier } => {
                if identifier.identifier == var {
                    replacement
                } else {
                    Ast::Var { identifier }
                }
            }
            Ast::Lambda { parameter, body } => {
                if parameter.identifier == var {
                    Ast::Lambda { parameter, body }
                } else {
                    Ast::Lambda {
                        parameter,
                        body: Box::new(body.replace(var, replacement)),
                    }
                }
            }
            Ast::App { function, argument } => Ast::App {
                function: Box::new(function.replace(var.clone(), replacement.clone())),
                argument: Box::new(argument.replace(var, replacement)),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn evaluates_identity_function() {
        assert_eq!(ast!((#x -> x) y).eval().pretty(), "y");
    }

    #[test]
    fn allows_to_implement_fst() {
        assert_eq!(
            ast!(
                (# tuple -> (tuple (# a -> (# b -> a))))
                (# t -> t x y)
            ).eval()
                .pretty(),
            "x"
        );
    }

    #[test]
    fn does_terminate_on_free_variable_applications() {
        assert_eq!(ast!(a b).eval().pretty(), "a b");
    }

    #[test]
    fn allows_to_implement_not_without_alpha_conversion() {
        let term = ast!((#b -> (# tt -> (# ff -> (b ff tt)))) (# t -> (# f -> t)));
        assert_eq!(term.eval().pretty(), "#v0<tt> -> #v3<ff> -> v3<ff>");
    }

    #[test]
    fn allows_to_implement_not_with_alpha_conversion() {
        let term = ast!((#b -> (# t -> (# f -> (b f t)))) (# t -> (# f -> t)));
        assert_eq!(term.eval().pretty(), "#v0<t> -> #v3<f> -> v3<f>");
    }

    mod make_variables_unique {
        use super::*;

        #[test]
        fn works() {
            let term = ast!((#x -> (#y -> x y)) (#x -> x));
            assert_eq!(
                term.make_variables_unique().pretty(),
                "(#v0<x> -> #v1<y> -> v0<x> v1<y>) (#v2<x> -> v2<x>)"
            );
        }

        #[test]
        fn does_not_use_existing_variables() {
            let term = ast!(#x -> #v0 -> x v1);
            assert_eq!(
                term.make_variables_unique().pretty(),
                "#v2<x> -> #v3<v0> -> v2<x> v1"
            );
        }
    }

    mod get_used_variables {
        use super::*;

        #[test]
        fn does_return_the_current_identifiers() {
            let var = Ast::Lambda {
                parameter: Identifier {
                    identifier: "foo".to_string(),
                    original: "foo_original".to_string(),
                },
                body: Box::new(Ast::Var {
                    identifier: Identifier {
                        identifier: "bar".to_string(),
                        original: "bar_original".to_string(),
                    },
                }),
            };
            let expected: HashSet<String> = ["foo", "bar"].iter().map(|x| x.to_string()).collect();
            assert_eq!(var.get_used_variables(), expected);
        }
    }

    #[test]
    fn allows_to_implement_apply() {
        let term = ast!((#fun -> (#x -> fun x)) (#x -> x) (# t -> # f -> t));
        assert_eq!(term.eval().pretty(), "#v0<t> -> #v3<f> -> v0<t>");
    }

    #[test]
    fn proper_alpha_conversion() {
        let term = ast!(#clashing -> ((#x -> #clashing -> clashing x) clashing));
        assert_eq!(
            term.eval().pretty(),
            "#v1<clashing> -> #v3<clashing> -> v3<clashing> v1<clashing>"
        );
    }
}
