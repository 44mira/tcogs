#![allow(dead_code)]

use std::collections::HashMap;

struct TuringMachine {
  memory_tape: Vec<char>,

  // TODO: Not sure about this, maybe state representations should be hashed
  current_state: String,

  // lookups are assigned by name and input, to reduce hashmap depth
  lookup: HashMap<(String, char), String>,
}

impl TuringMachine {
  #[allow(unused_mut)]
  fn new() -> Self {
    // initialize a default 2048 size for the memory tape for optimized access
    // on small tape sizes (no amortized indexing time).
    let mut memory_tape = Vec::with_capacity(2048);
    let mut lookup: HashMap<(String, char), String> = HashMap::new();

    TuringMachine {
      current_state: "START".to_owned(),
      memory_tape,
      lookup,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tm_init() {
    let _tm = TuringMachine::new();
  }
}
