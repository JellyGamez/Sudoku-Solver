# Sudoku solver

## Introduction
This project brings together my passion for Sudoku and my eagerness to learn a new programming language. I gave myself a challenge and went with Rust, which I was completely unfamiliar with. As it turned out for me, it's a powerful and very fast language, but most importantly, it proved to be a highly enjoyable language to write in.

## Approach
The solver employs a **backtracking** algorithm to progressively solve a given puzzle. It keeps track of all the empty cells and makes use of **bitmasks** to instantly fetch all the possible candidates for a specific cell.

It can also be configured to use a **heuristic** known as **MRV** (Minimum Remaining Values) which prioritizes selecting the cells with fewer candidates first. When enabled, this option enhances the solver's efficieny by increasing the likelihood of finding a solution or identifying contradictions early in the process. 

**Theoretically**, this reduces the number of possibilities that need to be explored in the search tree, ultimately making the algorithm more efficient and faster. 

**In practice**, depending on the provided dataset, this option can either speed up the algorithm by as much as **1000x** (particularly for 17 clue grids with overall large candidates sets), or it might marginally affect performance (typically applies to easy grids, as the number of candidates for each cell is already very small, making the selection of the best cell almost redundant).

## Benchmarks
*work in progress*


## Update log
- **1.0** - Plain backtracking with no optimizations
- **1.1** - A few tweaks, including stopping the backtracking early if a solution is found

<!-- -->

- **2.0** - Implemented bitmasks and proper display traits
- **2.1** - Cleaned up code and refactored for performance
- **2.2** - Implemented MRV heuristic

<!-- -->

- **3.0** - Complete file restructure for improved readability and configurability

## Usage

### Template
Each input grid has to respect the following template:

- It should be a string with a length of exactly 81 characters, representing the initial state of the puzzle.
- Use '0' to denote empty cells and digits from '1' to '9' for cells that are already solved
- The puzzle must be a valid sudoku puzzle with at least one solution
- Example: **004300209005009001070060043006002087190007400050083000600000105003508690042910300**

**Note**: The solver assumes all grids respect the above template.


### Configuration
Once you've prepared a file with your desired puzzles, navigate to the **config.rs** file located in the **src** directory, and adjust your configuration settings as needed:

```rust
pub const FILE_NAME: &str = ""; // Input file path

pub const PRINT_STATS: bool = true; // Option to print statistics after solving all grids

pub const NUMBER_OF_GRIDS: usize = 1; // The number of grids to solve from the file

pub const PRINT_SOLVED_GRIDS: bool = false; // Option to print solved grids to standard output

pub const MRV_HEURISTIC: bool = false; // Option to enable fewer candidates first heuristic
```

## Getting started

### 1. Clone the project
```bash
git clone https://github.com/JellyGamez/Sudoku-solver.git
```

### 2. Go to the project directory
```bash
cd Sudoku-solver
```

### 3. Set up the configuration file
  The configuration process is explained [here](#configuration).
  
### 4. Launch the solver
```sh
cargo run --release
```
