**Rustine: A Constraint Programming Solver in Rust**

Rustine is a constraint programming solver written in Rust. It aim to provides a simple and efficient way to solve constraint satisfaction problems (CSPs).

**Features**

* Supports integer variables with finite domains
* Supports inequality constraints defined using closures
* Uses a recursive backtracking algorithm to find solutions

**Usage**

To use Rustine, add the following line to your `Cargo.toml` file:
```toml
[dependencies]
rustine = { path = "path/to/rustine" }
```
Then, import the `rustine` crate in your Rust file:
```rust
use rustine::*;
```
Create a solver instance and add variables and constraints using the `add_variable` and `add_constraint` methods:
```rust
let mut solver = Solver::new();
let variable1 = Variable::new("x", vec![1, 2, 3]);
let variable2 = Variable::new("y", vec![4, 5, 6]);
solver.add_variable(variable1);
solver.add_variable(variable2);
let constraint =
    InequalityConstraint::new(vec!["x".to_string(), "y".to_string()], |values| {
        values.get("x").unwrap() + values.get("y").unwrap() <= 10
    });
solver.add_constraint(constraint);
```
Finally, call the `solve` method to find a solution:
```rust
let solution = solver.solve();
if let Some(solution) = solution {
    println!("Solution found: {:?}", solution);
} else {
    println!("No solution found");
}
```
**Testing**

Rustine includes a set of tests to ensure its correctness. To run the tests, use the following command:
```
cargo test
```
**License**

Rustine is licensed under the MIT License. See the LICENSE file for details.
