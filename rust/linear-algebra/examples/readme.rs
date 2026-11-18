//! Linear Algebra: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p linear-algebra-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&linear_algebra_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "low_rank_approximation", "inputs": {"A": [[3, 0], [0, 1]], "k": "1"}}));
    println!();
    show(json!({"key": "adjugate", "inputs": {"A": [[1, 2], [3, 4]]}}));
    println!();
    show(json!({"key": "characteristic_polynomial", "inputs": {"A": [[2, 1], [1, 2]]}}));
    println!();
    show(json!({"key": "least_squares_polynomial", "inputs": {"x": [0, 1, 2, 3], "y": [1, 0, 3, 10], "degree": "2"}}));
    println!();
    show(json!({"key": "conjugate_gradient", "inputs": {"A": [[4, 1], [1, 3]], "b": [1, 2]}}));
    println!();
    show(json!({"key": "matrix_vector_product", "inputs": {"A": [[1, 2], [3, 4]], "x": [5, 6]}}));
    println!();
    show(json!({"key": "condition_number", "inputs": {"A": [[2, 0], [0, 0.5]]}}));
    println!();
    show(json!({"key": "matrix_exponential", "inputs": {"A": [[0, 1], [0, 0]]}}));
    println!();
    show(json!({"key": "poisson_1d", "inputs": {"f": [1, 1, 1], "L": "1"}}));
    println!();
    show(json!({"key": "clamped_cubic_spline", "inputs": {"x": [0, 1, 2], "y": [0, 1, 0], "at": "0.5", "slope_start": "1", "slope_end": "-1"}}));
    println!();
    show(json!({"key": "csc_format", "inputs": {"A": [[1, 0, 2], [0, 0, 3], [4, 5, 0]]}}));
    println!();
    show(json!({"key": "column_space", "inputs": {"A": [[1, 2, 3], [2, 4, 7]]}}));
    println!();
    show(json!({"key": "angle_between_vectors", "inputs": {"u": [1, 0], "v": [1, 1]}}));
    println!();
    show(json!({"key": "cholesky", "inputs": {"A": [[4, 12, -16], [12, 37, -43], [-16, -43, 98]]}}));
    println!();
    show(json!({"key": "determinant_2x2", "inputs": {"A": [[1, 2], [3, 4]]}}));
}
