use std::collections::HashMap;
use crate::old_code::lexer::lexer;
use crate::old_code::token::{Logical, Token, TypeName, TypeValue};
use crate::old_code::parser::{AbstractSyntaxTree, Parser};

type FunctionID = u32;
type ScopeID = u32;
type Variable = (String, ScopeID);

pub struct Interpreter {
    function_map: HashMap<String, FunctionID>,
    functions: HashMap<FunctionID, AbstractSyntaxTree>,
    variables: HashMap<Variable, Token>,
    function_id: FunctionID,
    start_function_id: FunctionID,
    cur_function: FunctionID
}

impl Interpreter {
    pub fn new() -> Self {
        Self { function_map: HashMap::new(), functions: HashMap::new(), variables: HashMap::new(), function_id: 0, start_function_id: 0, cur_function: 0 }
    }


}
