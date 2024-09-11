use std::collections::HashMap;

use crate::parser::Type;

pub enum IRNode {}

pub struct IRFunction {
    pub body: Vec<IRNode>,
}

#[derive(Default)]
pub struct IRProgram {
    // pub types: HashMap<String, >
    pub functions: HashMap<String, (Vec<(String, Type)>, Option<Type>)>,
    pub body: HashMap<String, IRFunction>,
}

impl IRProgram {
    pub fn new() -> Self {
        Self::default()
    }
}
