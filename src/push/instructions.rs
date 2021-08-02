use crate::push::state::PushState;
use std::collections::HashMap;

use crate::push::boolean::*;
use crate::push::code::*;
use crate::push::execution::*;
use crate::push::float::*;
use crate::push::integer::*;

// Instructions
//
//
// Instruction as trait (Abstract class)
//
// Each instrcution is a struct
// Instruction Set is a hashmap with string key and struct as value

pub struct InstructionSet {
    pub map: HashMap<String, Instruction>,
}

impl InstructionSet {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Load the default instrcution set for the stack types
    /// bool, int, float, code, exec, name and sdr
    pub fn load(&mut self) {
        self.map
            .insert(String::from("NOOP"), Instruction::new(noop));
        load_boolean_instructions(&mut self.map);
        load_code_instructions(&mut self.map);
        load_exec_instructions(&mut self.map);
        load_int_instructions(&mut self.map);
        load_float_instructions(&mut self.map);
    }

    pub fn cache(&self) -> InstructionCache {
        InstructionCache::new(self.map.keys().cloned().collect())
    }
}

pub struct InstructionCache {
    pub list: Vec<String>,
}

impl InstructionCache {
    pub fn new(arg_list: Vec<String>) -> Self {
        Self { list: arg_list }
    }
}

pub struct Instruction {
    pub execute: Box<dyn FnMut(&mut PushState, &InstructionCache)>,
}

impl Instruction {
    pub fn new(execute: impl FnMut(&mut PushState, &InstructionCache) + 'static) -> Self {
        Self {
            execute: Box::new(execute),
        }
    }
}

/// NOOP: No operation.
fn noop(_push_state: &mut PushState, _instruction_cache: &InstructionCache) {}
