use std::collections::HashMap;
use crate::code::lexer::lexer;
use crate::code::token::{Token, TypeName, TypeValue};
use crate::code::parser::{AbstractSyntaxTree, Parser};

type FunctionID = u32;
type ScopeID = u32;
type Variable = (String, ScopeID);

pub struct Interpreter {
    function_map: HashMap<String, FunctionID>,
    functions: HashMap<FunctionID, AbstractSyntaxTree>,
    variables: HashMap<Variable, Token>,
    function_id: FunctionID,
    start_function_id: FunctionID
}

impl Interpreter {
    pub fn new() -> Self {
        Self { function_map: HashMap::new(), functions: HashMap::new(), variables: HashMap::new(), function_id: 0, start_function_id: 0 }
    }
    
    pub fn run_repl(&mut self) {
        loop {
            let tokens = lexer(&input);
            let abstract_syntax_tree = Parser::new(&tokens).parse_statements(Token::TypeName(TypeName::None));
            match abstract_syntax_tree {
                Ok(AbstractSyntaxTree) => {
                    self.run_function(AbstractSyntaxTree, vec![]);
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
                _ => println!("Unknown Error occured"),
            }
        }
    }

    pub fn pre_run(&mut self, program: Vec<AbstractSyntaxTree>) {
        let mut is_start = false;
        for func in program {
            if func.is_function() {
                self.function_map.insert(
                    func.function_get_name().to_string(),
                    FunctionID::from(self.function_id)
                );
                //func.function.variables = HashMap::new();
                self.functions.insert(
                    FunctionID::from(self.function_id),
                    func.clone(),
                );
                if func.function_get_name() == "start" {
                    is_start = true;
                    self.start_function_id = self.function_id;
                }
                self.function_id += 1;
            }
        }
        if !is_start {
            panic!("No start function found");
        }
    }
    
    fn run_function(&mut self, statements: Vec<AbstractSyntaxTree>, arguments: Vec<Token>) {
        // Define args as variables
        let arg_base = self
            .functions
            .get(&self.cur_function)
            .unwrap_or_else(|| panic!("{}", "Function not found".to_string()))
            .function_get_args_format();

        for (i, arg) in arg_base.iter().enumerate() {
            let getten_value = self.eval_expr(&arguments[i].clone());
            let func = self
                .functions
                .get_mut(&self.cur_function)
                .expect("Function not found");
            func.function_insert_variable(
                arg.1.clone().to_string(),
                Token::TypeValue(getten_value),
            );
        }

        for stmt in statements {
            match stmt {
                AbstractSyntaxTree::Let { name, value, .. } => {
                    //self.variables.insert(name.clone(), lexer::Token::TypeValue(self.eval_expr(&value)));
                    let getten_value = self.eval_expr(&value.clone());

                    // check if the variable name is taken by a function
                    if self.function_map.contains_key(&name) {
                        panic!("Variable name '{}' is taken", name);
                    }

                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        name.clone(),
                        lexer::Token::TypeValue(getten_value),
                    );
                }
                AbstractSyntaxTree::Print { value } => {
                    print!("{}", self.eval_expr(&value));
                }
                AbstractSyntaxTree::Println { value } => {
                    println!("{}", self.eval_expr(&value));
                }
                AbstractSyntaxTree::For {
                    start,
                    end,
                    value,
                    statements,
                } => {
                    let start_value = self.eval_expr(&start).as_i32();
                    let end_value = self.eval_expr(&end).as_i32();
                    let by_value = self.eval_expr(&value).as_i32().try_into().unwrap();

                    for i in (start_value..end_value).step_by(by_value) {
                        let func = self
                            .functions
                            .get_mut(&self.cur_function)
                            .expect("Function not found");
                        func.function_insert_variable(
                            start.clone().to_string(),
                            Token::TypeValue(TypeValue::I32(i.try_into().unwrap())),
                        );
                        self.run_function(statements.clone(), vec![]);
                    }
                }
                AbstractSyntaxTree::If {
                    l_var,
                    logic,
                    r_var,
                    statements,
                } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let condition = match logic {
                        Token::Logical(Logical::And) => left.as_bool() && right.as_bool(),
                        Token::Logical(Logical::Or) => left.as_bool() || right.as_bool(),
                        Token::Logical(Logical::Equals) => left == right,
                        Token::Logical(Logical::NotEquals) => left != right,
                        Token::Logical(Logical::GreaterThan) => left.as_i32() > right.as_i32(),
                        Token::Logical(Logical::LessThan) => left.as_i32() < right.as_i32(),
                        Token::Logical(Logical::GreaterThanEquals) => {
                            left.as_i32() >= right.as_i32()
                        }
                        Token::Logical(Logical::LessThanEquals) => left.as_i32() <= right.as_i32(),
                        _ => panic!("Invalid logic operator"),
                    };
                    if condition {
                        self.run_function(statements, vec![]);
                    }
                }
                AbstractSyntaxTree::Assign { l_var, r_var } => {
                    //let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32((right.as_i32()).try_into().unwrap())),
                    );
                }
                AbstractSyntaxTree::AddAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() + right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AbstractSyntaxTree::SubAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() - right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AbstractSyntaxTree::MulAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() * right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AbstractSyntaxTree::DivAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() / right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AbstractSyntaxTree::RemAssign { l_var, r_var } => {
                    let left = self.eval_expr(&l_var);
                    let right = self.eval_expr(&r_var);
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_insert_variable(
                        l_var.clone().to_string(),
                        Token::TypeValue(TypeValue::I32(
                            (left.as_i32() % right.as_i32()).try_into().unwrap(),
                        )),
                    );
                }
                AbstractSyntaxTree::FunctionCall { name, args } => {
                    let func_name = name.clone().to_string();
                    let target_func_addr = self
                        .function_map
                        .get(&func_name)
                        .expect("Function not found")
                        .clone();

                    let (target_statements, this_function, target_args) = {
                        let target_func = self
                            .functions
                            .get_mut(&target_func_addr)
                            .expect("Function not found");

                        let target_statements = target_func.function_get_statements().clone();
                        let this_function = self.cur_function.clone();
                        let target_args = target_func.function_get_args_format().clone();
                        (target_statements, this_function, target_args)
                    };
                    if target_args.len() != args.len() {
                        panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.eval_expr(arg).get_type();
                        let arg_type = Token::TypeName(arg_type);
                        if arg_type != target_args[i].0 {
                            panic!("Argument type mismatch");
                        }
                    }
                    self.cur_function = target_func_addr.clone();
                    self.run_function(target_statements, args.clone());
                    self.cur_function = this_function;
                    /*let return_value = {
                    let target_func = self
                    .functions
                    .get(&target_func_addr)
                    .expect("Function not found");
                    target_func.function_get_return_value().clone()
                    };
                    self.eval_expr(&return_value)
                    // Check for argument count, and type
                    let target_args = target_func.function_get_args_format();
                    if target_args.len() != args.len() {
                    panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                    let arg_type = self.eval_expr(arg).get_type();
                    let arg_type = Token::TypeName(arg_type);
                    if arg_type != target_args[i].0 {
                    panic!("Argument type mismatch");
                    }
                    }*/
                }
                /*
                AbstractSyntaxTree::Assign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |_, right| right.clone());
                },
                AbstractSyntaxTree::AddAssign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |left, right| left.add(right));
                },
                AbstractSyntaxTree::SubAssign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |left, right| left.sub(right));
                },
                AbstractSyntaxTree::MulAssign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |left, right| left.mul(right));
                },
                AbstractSyntaxTree::DivAssign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |left, right| left.div(right));
                },
                AbstractSyntaxTree::RemAssign { l_var, r_var } => {
                self.update_variable(l_var, r_var, |left, right| left.rem(right));
                },
                 */
                AbstractSyntaxTree::Return { value } => {
                    let return_value = Token::TypeValue(self.eval_expr(&value));
                    let func = self
                        .functions
                        .get_mut(&self.cur_function)
                        .expect("Function not found");
                    func.function_set_return_value(return_value);
                    break;
                }
                _ => panic!("Invalid statement"),
            }
        }
        //println!("Function: {:?}", func);
    }

    fn eval_expr(&mut self, expr: &Token) -> TypeValue {
        if let Token::TypeValue(inner_expr) = expr {
            match inner_expr {
                TypeValue::None
                | TypeValue::I8(_)
                | TypeValue::I16(_)
                | TypeValue::I32(_)
                | TypeValue::I64(_)
                | TypeValue::U8(_)
                | TypeValue::U16(_)
                | TypeValue::U32(_)
                | TypeValue::U64(_)
                | TypeValue::QuotedString(_)
                | TypeValue::Bool(_) => inner_expr.clone(),
                TypeValue::Identifier(id) => {
                    let func = self
                        .functions
                        .get(&self.cur_function)
                        .expect("Function not found");
                    if let value = func.function_get_variable(id.to_string()) {
                        if let Token::TypeValue(inner_value) = value {
                            match inner_value {
                                TypeValue::None
                                | TypeValue::I8(_)
                                | TypeValue::I16(_)
                                | TypeValue::I32(_)
                                | TypeValue::I64(_)
                                | TypeValue::U8(_)
                                | TypeValue::U16(_)
                                | TypeValue::U32(_)
                                | TypeValue::U64(_)
                                | TypeValue::QuotedString(_)
                                | TypeValue::Bool(_) => inner_value,
                                TypeValue::Identifier(_) => {
                                    panic!("Invalid identifier reference")
                                }
                                TypeValue::FunctionCall(_, _) => {
                                    panic!("Invalid function call reference")
                                }
                            }
                        } else {
                            panic!("Invalid value type");
                        }
                    } else {
                        panic!("Undefined variable '{}'", id);
                    }
                }
                TypeValue::FunctionCall(id, args) => {
                    let func_name = id.clone();
                    let target_func_addr = self
                        .function_map
                        .get(&func_name)
                        .expect("Function not found")
                        .clone();

                    let (target_statements, this_function, target_args) = {
                        let target_func = self
                            .functions
                            .get_mut(&target_func_addr)
                            .expect("Function not found");

                        let target_statements = target_func.function_get_statements();
                        let this_function = self.cur_function.clone();
                        let target_args = target_func.function_get_args_format();
                        (target_statements, this_function, target_args)
                    };
                    if target_args.len() != args.len() {
                        //println!("{} {}", target_args.len(), args.len());
                        panic!("Argument count mismatch");
                    }
                    for (i, arg) in args.iter().enumerate() {
                        let arg_type = self.eval_expr(arg).get_type();
                        let arg_type = Token::TypeName(arg_type);
                        if arg_type != target_args[i].0 {
                            panic!("Argument type mismatch");
                        }
                    }

                    self.cur_function = target_func_addr.clone();
                    self.run_function(target_statements, args.clone());
                    self.cur_function = this_function;
                    let return_value = {
                        let target_func = self
                            .functions
                            .get(&target_func_addr)
                            .expect("Function not found");
                        target_func.function_get_return_value()
                    };
                    self.eval_expr(&return_value)
                }
            }
        } else {
            panic!("Invalid expression");
        }
    }
}
