# Turing Machine Compiler

The **Turing Machine Compiler** is a tool designed to compile high-level Turing Machine descriptions into executable code that can be run on a Turing Machine simulator.

## Features

- **High-level Language**: generate Turing Machine instructions using a high-level, human-readable language.
- **Compiler**: Compile codes into machine-readable code for execution.
- **Simulator Integration**: Integrate with Turing Machine simulators for testing and debugging.

## Getting Started

### Installation

1. Clone the repository from GitHub:

    ```bash
    git clone https://github.com/aikoschurmann/TuringCompiler
    ```

2. Build the compiler using your preferred build tool (e.g., Cargo).

### Usage

...


## Instruction Syntax

The Turing Machine instruction language supports the following constructs:

States: Define states and transitions between states.
Symbols: Define input symbols and transitions based on symbols.
Actions: Define actions to perform on the tape (e.g., write symbol, move head).

Here's an example Turing Machine description file:

```# Description of a Turing Machine to increment a binary number
# Transition format: <current-state> <read-symbol> <write-symbol> <move-direction> <next-state>

// Initial state
GO_TOTAL_RIGHT 1 1 right GO_TOTAL_RIGHT
GO_TOTAL_RIGHT 0 0 right GO_TOTAL_RIGHT
GO_TOTAL_RIGHT _ _ left  FIX_LEAST_SIGNIFICANT_BIT

// state for carying over
FIX_LEAST_SIGNIFICANT_BIT 0 1 stay DONE
FIX_LEAST_SIGNIFICANT_BIT 1 0 left FIX_LEAST_SIGNIFICANT_BIT
FIX_LEAST_SIGNIFICANT_BIT _ 1 stay DONE
```