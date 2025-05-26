
# Traveling Salesman Problem Solver

This program solves the **Traveling Salesman Problem (TSP)** using **Dynamic Programming with Bitmasking**. It finds the minimum distance needed to visit all cities exactly once and return to the starting city. The program also outputs the path taken.

# Author
Fityatul Haq R. - 13523116

---

## Features
- Computes the minimum TSP route distance.
- Outputs the sequence of cities visited.
- Uses an input distance matrix representing a weighted graph.

---

## Prerequisites

### 1. Rust Compiler
Install Rust via [Rustup](https://rustup.rs/):

On Unix/Linux/macOS:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

On Windows:
- Download and install Rust from [rust-lang.org](https://www.rust-lang.org/tools/install).
- Make sure Rust is added to your system PATH.

Verify Rust installation with:
```bash
rustc --version
```

### 2. Code Editor or IDE
Use any text editor or IDE such as **Visual Studio Code**, **IntelliJ IDEA**, or others.

---

## How to Run the Program

### 1. Clone the Repository (if applicable)
```bash
git clone <repository-url>
cd Traveling_Salesman_Problem_Solver
```
Or simply copy the source code to a local folder.

### 2. Compile the Program
Navigate to the `src` directory and compile using:
```bash
rustc solver.rs
```
This creates an executable named `solver` (or `solver.exe` on Windows).

### 3. Run the Executable
Run the program with:
```bash
./solver
```
On Windows:
```bash
solver.exe
```

---

## Input
The program uses a distance matrix between cities. Example matrix inside `solver.rs`:
```rust
let dist = vec![
    vec![0, 10, 15, 20],
    vec![5, 0, 9, 10],
    vec![6, 13, 0, 12],
    vec![8, 8, 9, 0],
];
```
Here, `dist[i][j]` represents the distance from city `i` to city `j`.

---

## Output
The program prints:
- The minimum travel distance, e.g.:
  ```
  Minimum TSP travel distance is: 35
  ```
- The path taken, e.g.:
  ```
  Path taken: [0, 1, 3, 2, 0]
  ```

---

## Project Structure
```
Travelling_Salesman_Problem_Solver/
│
├── src/
│   └── solver.rs        # Main source code
│
└── README.md            # This documentation file
```

---

## Notes
- You can modify the distance matrix in `solver.rs` to test different TSP scenarios.
- Ensure Rust is correctly installed to compile and run the program.

Feel free to open issues or contact me if you have any questions! 😊
