# Turing Cogs

A Turing machine visualizer written in *raylib* and *Rust*.

## Defining a Turing Machine

Takes in an *unordered* `.csv` file with the following columns:

> [!TIP]
> The order of the rows does not matter as these are just state
> definitions for the machine.

```
<STATE NAME>,<INPUT>,<OUTPUT>,<DIRECTION>,<NEXT STATE>
```

- `<STATE NAME>`
  - The name of the state in the lookup table representation of the Turing
    machine.
  - A turing machine must have *at least* one state named `START`, which will
    be the entrypoint of the program.
  - A state named `HALT` is not allowed, as it is reserved for the halting
    state.
- `<INPUT>`
  - The character read from the memory tape.
  - Can be any ASCII character.
- `<OUTPUT>`
  - The character to write on the memory tape.
  - Can be any ASCII character.
- `<DIRECTION>`
  - The direction to move in the tape pointer.
  - Can only be either `L` (left) or `R` (right).
- `<NEXT STATE>`
  - The state that will be looked up in the table as the new state of the
    machine.
  - A turing machine can also go to a special state named `HALT`, which
    unconditionally halts the program.

## TODO
- [x] Initialize raylib project
- [x] Create a data structure for the Turing machine
  - [x] Memory tape
  - [x] State lookup table
  - [x] Undo stack
  - [x] Add tests
- [ ] Parse CSVs into a machine
  - [ ] Add tests
- [ ] Evaluate a machine based on some input
  - [ ] Add tests
- [ ] Create UI
  - [ ] Memory tape representation
  - [ ] Label for current state
  - [ ] Button for step forward and backward
  - [ ] Animation for moving the tape pointer
  - [ ] Visualize crashes
  - [ ] Assets and textures
- [ ] Output result stats of the machine
  - [ ] Write into results file
  - [ ] Accepted or not
  - [ ] Count of transitions taken
  - [ ] Transitions taken in sequence
  - [ ] Last transition before halting
