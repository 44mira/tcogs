#![allow(dead_code)]

use std::collections::HashMap;

enum Direction {
  Left,
  Right,
}

struct TuringMachine {
  memory_tape: Vec<char>,

  // TODO: Not sure about this, maybe state representations should be hashed
  current_state: String,

  // lookups are assigned by name and input, to reduce hashmap depth
  lookup: HashMap<(String, char), String>,

  tape_pointer: usize,
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
      tape_pointer: 0,
      memory_tape,
      lookup,
    }
  }

  /// Reads the memory tape at the tape pointer, returning the found character.
  fn read(&self) -> char {
    self.memory_tape[self.tape_pointer]
  }

  /// Writes the character `e` in the memory tape at the tape pointer.
  fn write(&mut self, e: char) {
    self.memory_tape[self.tape_pointer] = e;
  }

  /// Updates the character found at the tape pointer using `updater`, and then
  /// moves the tape pointer one step to the specified Direction in `dir`.
  fn step<F>(&mut self, updater: F, dir: Direction)
  where
    F: FnOnce(char) -> char,
  {
    // update the tape
    self.write(updater(self.read()));

    // move pointer
    match dir {
      Direction::Right => self.tape_pointer += 1,
      Direction::Left => self.tape_pointer -= 1,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn tm_init() {
    let tm = TuringMachine::new();

    assert_eq!("START", tm.current_state);
    assert_eq!(0, tm.tape_pointer);
    assert_eq!(2048, tm.memory_tape.capacity());
  }

  #[test]
  fn tm_step() {
    let mut tm = TuringMachine::new();
    tm.memory_tape = "abba".chars().collect();

    assert_eq!('a', tm.read());
    tm.step(|_| '#', Direction::Right);
    assert_eq!('b', tm.read());
    tm.step(|x| x, Direction::Left);
    assert_eq!('#', tm.read());
  }
}
