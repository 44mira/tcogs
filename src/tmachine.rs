#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
  Left,
  Right,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transition {
  output: char,
  direction: Direction,
  next: String,
}

impl Transition {
  pub fn new(output: char, direction: Direction, next: String) -> Self {
    Transition {
      output,
      direction,
      next,
    }
  }
}

pub struct TuringMachine {
  memory_tape: Vec<char>,

  // TODO: Not sure about this, maybe state representations should be hashed
  current_state: Option<String>,

  // lookups are assigned by name and input, to reduce hashmap depth
  lookup: HashMap<(String, char), Transition>,

  tape_pointer: usize,
}

impl TuringMachine {
  #[allow(unused_mut)]
  pub fn new() -> Self {
    let mut memory_tape = vec!['_'; 2048];
    let mut lookup: HashMap<(String, char), Transition> = HashMap::new();

    TuringMachine {
      current_state: Some("START".to_owned()),
      tape_pointer: 0,
      memory_tape,
      lookup,
    }
  }

  pub fn from(input: &str) -> Self {
    let mut tm = Self::new();
    tm.set_tape(input);

    return tm;
  }

  /// Reads the memory tape at the tape pointer, returning the found character.
  fn read(&self) -> char {
    self.memory_tape[self.tape_pointer]
  }

  /// Writes the character `e` in the memory tape at the tape pointer.
  fn write(&mut self, e: char) {
    self.memory_tape[self.tape_pointer] = e;
  }

  /// utility function for setting the first indices of the memory tape to some
  /// string.
  pub fn set_tape(&mut self, input: &str) {
    let content = input.chars();

    for (i, item) in content.into_iter().enumerate() {
      self.memory_tape[i] = item;
    }
  }

  /// Updates the character found at the tape pointer using `char`, and then
  /// moves the tape pointer one step to the specified Direction in `dir`.
  fn step(&mut self, e: char, dir: Direction) {
    // update the tape
    self.write(e);

    // move pointer
    match dir {
      Direction::Right => self.tape_pointer += 1,
      Direction::Left => self.tape_pointer -= 1,
    }
  }

  /// Add a state to the lookup table. Replaces existing state.
  pub fn add_state(
    &mut self,
    name: &str,
    expected_input: char,
    transition: Transition,
  ) {
    _ = self
      .lookup
      .insert((name.to_owned(), expected_input), transition);
  }

  /// Update the Turing machine based on the input at the pointer and the
  /// current state.
  pub fn forward(&mut self) {
    // no-op on HALT state
    if self.current_state == Some("HALT".to_owned()) {
      return;
    }

    let input = self.read();
    let Some(state) = self
      .lookup
      .get(&(self.current_state.clone().unwrap(), input))
      .cloned()
    else {
      self.current_state = None;
      return;
    };

    self.step(state.output, state.direction);
    self.current_state = Some(state.next);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// helper function for tests
  fn add_state(
    tm: &mut TuringMachine,
    name: &str,
    input: char,
    output: char,
    dir: Direction,
    next: &str,
  ) {
    tm.add_state(name, input, Transition::new(output, dir, next.to_owned()));
  }

  #[test]
  fn tm_init() {
    let tm = TuringMachine::new();

    assert_eq!("START", tm.current_state.unwrap());
    assert_eq!(0, tm.tape_pointer);
    assert_eq!(2048, tm.memory_tape.capacity());
  }

  #[test]
  fn tm_step() {
    let mut tm = TuringMachine::new();
    tm.memory_tape = "abba".chars().collect();

    assert_eq!('a', tm.read());
    tm.step('#', Direction::Right);
    assert_eq!('b', tm.read());
    tm.step('b', Direction::Left);
    assert_eq!('#', tm.read());
  }

  #[test]
  fn tm_add_state() {
    let mut tm = TuringMachine::new();

    add_state(&mut tm, "A", 'a', 'b', Direction::Right, "START");
    add_state(&mut tm, "START", 'a', 'b', Direction::Right, "A");

    assert_eq!(
      tm.lookup.get(&("START".to_owned(), 'a')),
      Some(&Transition::new('b', Direction::Right, "A".to_owned()))
    )
  }

  #[test]
  fn tm_forward() {
    let mut tm = TuringMachine::new();
    tm.memory_tape[0] = 'a';

    add_state(&mut tm, "START", 'a', '#', Direction::Right, "2");
    add_state(&mut tm, "2", '_', 'a', Direction::Left, "START");

    assert_eq!('a', tm.read());
    tm.forward();
    assert_eq!('_', tm.read());
    assert_eq!(Some("2".to_owned()), tm.current_state);
    tm.forward();
    assert_eq!('#', tm.read());
    assert_eq!(Some("START".to_owned()), tm.current_state);
    tm.forward();
    assert_eq!(None, tm.current_state);
  }

  #[test]
  fn tm_halt() {
    let mut tm = TuringMachine::new();

    add_state(&mut tm, "START", '_', '#', Direction::Right, "HALT");

    assert_eq!(Some("START".to_owned()), tm.current_state);
    tm.forward();
    assert_eq!(Some("HALT".to_owned()), tm.current_state);
    tm.forward();
    assert_eq!(Some("HALT".to_owned()), tm.current_state);
  }

  #[test]
  fn tm_set_state() {
    let test_string = "hello!";
    let mut tm = TuringMachine::from(test_string);

    for char in test_string.chars() {
      assert_eq!(char, tm.read());
      tm.step(char, Direction::Right);
    }
    assert_eq!('_', tm.read());
  }
}
