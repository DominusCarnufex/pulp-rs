use std::collections::HashMap;
use super::segments::{Const, Opcode, Segment};

pub type Environment = HashMap<String, Const>;
pub type ProgState = (Vec<Const>, Vec<Environment>);

impl Segment    {
    pub fn execute(&self, state : ProgState) -> Result<ProgState, String>   {
        let Segment::Code {
            name         : _,
            symbol_table : ref symbols,
            const_table  : ref constants,
            code         : ref opcodes
        } = *self;

        let (mut stack, mut env_stack) = state;

        for op in opcodes   {
            match *op   {
                Opcode::NOp             => {},
                Opcode::Pop             => {
                    if stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            de dépiler sur une pile vide.".to_string());
                    }

                    let length = stack.len();
                    stack.truncate(length - 1);
                },
                Opcode::Rot2            => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de Rot2 sur une pile contenant moins de \
                            deux éléments.".to_string());
                    }

                    let ts  = stack.pop().unwrap();
                    let ts1 = stack.pop().unwrap();
                    stack.push(ts);
                    stack.push(ts1);
                },
                Opcode::Rot3            => {
                    if stack.len() < 3  {
                        return Err("Erreur d’exécution : tentative \
                            de Rot3 sur une pile contenant moins de \
                            trois éléments.".to_string());
                    }

                    let length = stack.len();
                    let ts2 = stack.remove(length - 3);
                    stack.push(ts2);
                },
                Opcode::DupTop          => {
                    if stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            de duplication sur une pile vide.".to_string());
                    }

                    let ts = stack.pop().unwrap();
                    stack.push(ts.clone());
                    stack.push(ts);
                },
                Opcode::Push(idx)       => {
                    let c = match constants.get(idx)    {
                        Some(a) => a.clone(),
                        None    => return Err("Erreur d’exécution : tentative \
                                        d’accès à une constante absente de la \
                                        table.".to_string())
                    };
                    stack.push(c);
                },
                Opcode::PushNewEnv      => {
                    let e = Environment::new();
                    env_stack.push(e);
                },
                Opcode::PopEnv          => {
                    if env_stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de dépiler un environnement sur une \
                            pile vide.".to_string());
                    } // La base ne peut être dépilée.

                    let length = env_stack.len();
                    env_stack.truncate(length - 1);
                },
                Opcode::Let(idx)        => {
                    let s = match symbols.get(idx)  {
                        Some(a) => a.clone(),
                        None    => return Err("Erreur d’exécution : tentative \
                                        d’accès à un symbole absent de la \
                                        table.".to_string())
                    };

                    if env_stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            d’accéder à un environnement sur une \
                            pile vide.".to_string());
                    }

                    if stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            de stocker le premier élément d’une \
                            pile vide.".to_string());
                    }

                    let mut e  = env_stack.pop().unwrap();
                    let     ts = stack.pop().unwrap();
                    match e.insert(s.clone(), ts)   {
                        Some(_) => return Err(format!("Erreur d’exécution : \
                                        la variable {} est déjà définie dans \
                                        cet environnement.", s)),
                        None    => env_stack.push(e)
                    }
                },
                Opcode::Store(idx)      => {
                    let s = match symbols.get(idx)  {
                        Some(a) => a.clone(),
                        None    => return Err("Erreur d’exécution : tentative \
                                        d’accès à un symbole absent de la \
                                        table.".to_string())
                    };

                    if env_stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            d’accéder à un environnement sur une \
                            pile vide.".to_string());
                    }

                    if stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            de stocker le premier élément d’une \
                            pile vide.".to_string());
                    }

                    let ts     = stack.pop().unwrap();
                    let length = env_stack.len();
                    let mut done = false;

                    for i in 0..length  {
                        let e = env_stack.get_mut(length - i - 1).unwrap();
                        if e.contains_key(&s)   {
                            match e.insert(s.clone(), ts)   {
                                Some(_) => {
                                    done = true;
                                    break;
                                },
                                None => {}
                            }
                        }
                    }

                    if !done    {
                        return Err(format!("Erreur d’exécution : la variable \
                            {} n’est pas définie dans cet environnement.", s));
                    }
                },
                Opcode::Load(idx)       => {
                    let s = match symbols.get(idx)  {
                        Some(a) => a.clone(),
                        None    => return Err("Erreur d’exécution : tentative \
                                        d’accès à un symbole absent de la \
                                        table.".to_string())
                    };

                    if env_stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            d’accéder à un environnement sur une \
                            pile vide.".to_string());
                    }

                    let length = env_stack.len();
                    let mut done = false;

                    for i in 0..length  {
                        let e = env_stack.get_mut(length - i - 1).unwrap();
                        match e.get(&s) {
                            Some(a) => {
                                stack.push(a.clone());
                                done = true;
                                break;
                            },
                            None => {}
                        }
                    }

                    if !done    {
                        return Err(format!("Erreur d’exécution : la variable \
                            {} n’est pas définie dans cet environnement.", s));
                    }
                },
                Opcode::Add             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            d’addition sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 + ts));
                },
                Opcode::Sub             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de soustraction sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 - ts));
                },
                Opcode::Mul             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de multiplication sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 * ts));
                },
                Opcode::Div             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de division sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 / ts));
                },
                Opcode::Pow             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de puissance sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    // La fonction pow() de Rust n’accepte qu’un u32 comme
                    // exposant. Cela ne correspond donc pas totalement au
                    // comportement attendu, puisque ce devrait être un i64.
                    stack.push(Const::Int(ts1.pow(ts as u32)));
                },
                Opcode::Mod             => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de modulo sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 % ts));
                },
                Opcode::BitOr           => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de ou logique sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 | ts));
                },
                Opcode::BitAnd          => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de et logique sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 & ts));
                },
                Opcode::BitXor         => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de XOR sur une pile contenant moins de \
                            deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 ^ ts));
                },
                Opcode::LShift         => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de décalage à gauche sur une pile contenant moins \
                            de deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 << ts));
                },
                Opcode::RShift         => {
                    if stack.len() < 2  {
                        return Err("Erreur d’exécution : tentative \
                            de décalage à droite sur une pile contenant moins \
                            de deux nombres.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    let ts1 = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(ts1 >> ts));
                },
                Opcode::UMinus         => {
                    if stack.len() < 1  {
                        return Err("Erreur d’exécution : tentative \
                            de moins unaire sur une pile vide.".to_string());
                    }

                    let ts = match stack.pop().unwrap() {
                        Const::Int(a) => a,
                        Const::Abort(err) => {
                            stack.push(Const::Abort(err));
                            return Ok((stack, env_stack));
                        }
                    };

                    stack.push(Const::Int(-ts));
                },
                Opcode::Abort(err)     => {
                    stack.push(Const::Abort(err));
                    return Ok((stack, env_stack));
                },
                //_                       => {},
            } // End of match.
        } // End of loop.

        Ok((stack, env_stack))
    } // End of execute() function.
} // End of impl Segment.
