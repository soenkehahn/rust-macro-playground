use parser::*;

impl Ast {
    pub fn eval(self) -> Ast {
        let mut result = self.make_variables_unique();
        let mut cont = true;
        while cont {
            println!("eval: {}", result);
            match result.clone().step() {
                Some(new) => {
                    result = new;
                }
                None => cont = false,
            }
        }
        result
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
                        let new_identifier = Identifier {
                            identifier: new_variable.clone(),
                            original: parameter.identifier.clone(),
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
        inner(&mut AllVariables(0), self)
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
        assert_eq!(term.eval().pretty(), "#v1<tt> -> #v2<ff> -> v2<ff>");
    }

    #[test]
    fn allows_to_implement_not_with_alpha_conversion() {
        let term = ast!((#b -> (# t -> (# f -> (b f t)))) (# t -> (# f -> t)));
        assert_eq!(term.eval().pretty(), "#v1<t> -> #v2<f> -> v2<f>");
    }

    #[test]
    fn make_all_variables_unique() {
        let term = ast!((#x -> (#y -> x y)) (#x -> x));
        assert_eq!(
            term.make_variables_unique().pretty(),
            "(#v0<x> -> #v1<y> -> v0<x> v1<y>) (#v2<x> -> v2<x>)"
        );
    }

    #[test]
    fn allows_to_implement_apply() {
        let term = ast!((#fun -> (#x -> fun x)) (#x -> x) (# t -> # f -> t));
        assert_eq!(term.eval().pretty(), "#v3<t> -> #v4<f> -> v3<t>");
    }

    // #[test]
    // fn allows_to_apply_not_three_times() {
    //     let term =
    //         ast!((#fun -> (#x -> (fun (fun x)))) (#b -> # t -> # f -> b f t) (# t -> # f -> t));
    //     assert_eq!(term.eval().pretty(), "#t -> #f -> f");
    // }
}
