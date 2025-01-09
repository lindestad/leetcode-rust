# LeetCode Solutions in Rust
This project provides a Rust-based framework for organizing and running LeetCode problem solutions dynamically. Each problem is implemented as a separate module, and solutions are executed based on a problem number passed as a command-line argument.

## Features

- **Dynamic Problem Registration**: Automatically detects and registers problem modules (`qXX.rs`) from the `problems/` directory.
- **Single Entry Point**: Run any problem solution using `cargo run <problem_number>`.
- **Scalable Structure**: Easily add new problems by creating a new module file in the `problems/` directory.

## Project Structure

```
.
├── Cargo.toml         # Rust project configuration
├── build.rs           # Build script for generating the `mod.rs` file
├── src/
│   ├── main.rs        # Entry point of the program
│   ├── problems/      # Directory containing problem modules
│   │   ├── q20.rs     # Problem 20 solution
│   │   ├── q21.rs     # Problem 21 solution
│   │   ├── qXX.rs     # Problem XX solution
│   │   ├── mod.rs     # Auto-generated module file
```

### Example Problem Module (`q20.rs`)

Each problem solution is implemented in its own file in the `problems/` directory. The file must define a `run_function()` as the entry point.

```rust
pub fn solution() {
    println!("Running solution for problem XX!");
    // Add your solution logic here
}
```

## Usage

1. **Clone the repository**:
   ```bash
   git clone git@github.com:lindestad/leetcode-rust.git
   cd leetcode-rust
   ```

2. **Add a new problem**:
   - Create a file `qXX.rs` in the `src/problems/` directory (e.g., `q22.rs`).
   - Implement the solution in the `solution()` function.

3. **Run a solution**:
   Use `cargo run` followed by the problem number:
   ```bash
   cargo run 20
   ```
   Example output:
   ```
   Running tests for problem 20
   Compiling lc v0.1.0 (C:\Users\danie\dev\leetcode-rust)
   Finished `test` profile [unoptimized + debuginfo] target(s) in 0.38s
   Running unittests src/main.rs (target\debug\deps\lc-14bf635e9e02197c.exe)

   running 2 tests
   test problems::q20::tests::test_solution_case ... ok
   test problems::q20::tests::test_example_case ... ok

   test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
   ```

4. **Add more problems**:
   Simply add new `qXX.rs` files. The project automatically detects and registers them during the build process.

## How It Works

1. The `build.rs` script scans the `src/problems/` directory for files matching the `qXX.rs` naming convention.
2. It generates a `mod.rs` file in the `problems/` directory, which:
   - Imports each problem module.
   - Registers problem functions in a dynamic registry.
3. The `main.rs` file takes a problem number as input and executes the corresponding solution.

## Adding New Problems

1. Create a new file in `src/problems/`:
   ```bash
   touch src/problems/q<problem_number>.rs
   ```
   For problem 22:
   ```bash
   touch src/problems/q22.rs
   ```
2. Implement the solution in the file:
   ```rust
   pub fn solution() {
       println!("Running solution for problem XX!");
       // Your solution logic here
   }
   ```
3. Run the solution:
   ```bash
   cargo run <problem_number>
   ```
   For problem 22:
   ```bash
   cargo run 22
   ```
