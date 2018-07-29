use parser::*;

impl Ast {
    pub fn eval(self) -> Ast {
        let new = self.make_variables_unique();
        match new.clone().step() {
            Some(new) => new.eval(),
            None => new,
        }
    }

    fn make_variables_unique(self) -> Ast {
        struct AllVariables(i32);
        impl Iterator for AllVariables {
            type Item = String;
            fn next(&mut self) -> Option<String> {
                match self {
                    AllVariables(ref mut counter) => {
                        let r = Some(format!("v{}", counter));
                        *counter += 1;
                        r
                    }
                }
            }
        }

        fn inner(variables: &mut AllVariables, ast: Ast) -> Ast {
            match ast {
                var @ Ast::Var { .. } => var,
                Ast::Lambda { parameter, body } => match variables.next() {
                    None => panic!("no variables left"),
                    Some(new_variable) => {
                        let new_body = inner(
                            variables,
                            body.replace(
                                parameter,
                                Ast::Var {
                                    variable: new_variable.clone(),
                                },
                            ),
                        );
                        Ast::Lambda {
                            parameter: new_variable,
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
        inner(&mut AllVariables(0), self)
    }

    fn step(self) -> Option<Ast> {
        match self {
            Ast::App { function, argument } => match (*function).clone() {
                Ast::Lambda { parameter, body } => Some(body.replace(parameter, *argument)),
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
            Ast::Var { variable } => {
                if variable == var {
                    replacement
                } else {
                    Ast::Var { variable }
                }
            }
            Ast::Lambda { parameter, body } => Ast::Lambda {
                parameter,
                body: Box::new(body.replace(var, replacement)),
            },
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
        assert_eq!(term.eval().pretty(), "#v0 -> #v1 -> v1");
    }

    #[test]
    fn allows_to_implement_not_with_alpha_conversion() {
        let term = ast!((#b -> (# t -> (# f -> (b f t)))) (# t -> (# f -> t)));
        assert_eq!(term.eval().pretty(), "#v0 -> #v1 -> v1");
    }

    #[test]
    fn make_all_variables_unique() {
        let term = ast!((#x -> (#y -> x y)) (#x -> x));
        assert_eq!(
            term.make_variables_unique().pretty(),
            "(#v0 -> #v1 -> v0 v1) (#v2 -> v2)"
        );
    }

    // #[test]
    // fn foo() {
    //     let term = ast!((#fun -> (#x -> fun (fun x))) (#b -> (# tt -> (# ff -> (b ff tt)))) (# t -> (# f -> t)));
    //     assert_eq!(term.eval().pretty(), "# tt -> (# ff -> (ff))");
    // }
}
