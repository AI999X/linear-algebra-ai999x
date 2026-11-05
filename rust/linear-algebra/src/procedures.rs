//! linear_algebra: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/linear_algebra/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "adjugate",
    "angle_between_vectors",
    "bisection_root",
    "change_of_basis",
    "characteristic_polynomial",
    "cholesky",
    "clamped_cubic_spline",
    "column_space",
    "condition_number",
    "conjugate_gradient",
    "cross_product",
    "csc_format",
    "csr_format",
    "csr_to_dense",
    "determinant",
    "determinant_2x2",
    "determinant_3x3",
    "distance_between_points",
    "dot_product",
    "eigenvalues_2x2",
    "gauss_legendre_integral",
    "gauss_legendre_rule",
    "gauss_seidel",
    "general_eigenvalues",
    "gram_schmidt",
    "interpolating_polynomial",
    "inverse_iteration",
    "jacobi_iteration",
    "kronecker_product",
    "lagrange_interpolation",
    "least_squares_line",
    "least_squares_polynomial",
    "least_squares_solution",
    "linear_independence",
    "linear_interpolation",
    "linear_system_classification",
    "low_rank_approximation",
    "lu_decomposition",
    "matrix_addition",
    "matrix_exponential",
    "matrix_inverse",
    "matrix_multiplication",
    "matrix_norms",
    "matrix_power",
    "matrix_rank",
    "matrix_square_root",
    "matrix_subtraction",
    "matrix_trace",
    "matrix_transpose",
    "matrix_vector_product",
    "midpoint_rule",
    "minor_cofactor",
    "natural_cubic_spline",
    "newton_divided_differences",
    "newton_root",
    "null_space",
    "numerical_derivative",
    "outer_product",
    "parallelogram_area",
    "poisson_1d",
    "poisson_2d",
    "polar_decomposition",
    "polynomial_evaluation",
    "positive_definiteness",
    "power_iteration",
    "projection_matrix",
    "pseudoinverse",
    "qr_decomposition",
    "rotate_vector_2d",
    "rref",
    "scalar_matrix_multiplication",
    "scalar_vector_multiplication",
    "secant_root",
    "simpson_rule",
    "singular_values",
    "solve_2x2_system",
    "solve_3x3_system",
    "solve_linear_system",
    "sparse_matvec",
    "symmetric_eigenvalues",
    "symmetric_eigenvectors",
    "trapezoid_rule",
    "tridiagonal_solve",
    "triple_scalar_product",
    "unit_vector",
    "vector_addition",
    "vector_norm",
    "vector_projection",
    "vector_subtraction",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "adjugate" => {
            let r = f_adjugate(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_59(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "angle_between_vectors" => {
            let r = f_angle_between_vectors(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_109(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bisection_root" => {
            let r = f_bisection_root(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_217(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "change_of_basis" => {
            let r = f_change_of_basis(match v.get("basis") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "characteristic_polynomial" => {
            let r = f_characteristic_polynomial(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_382(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cholesky" => {
            let r = f_cholesky(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_466(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "clamped_cubic_spline" => {
            let r = f_clamped_cubic_spline(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("slope_start") { Some(x) => py::from_val(x), None => { return None } }, match v.get("slope_end") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_566(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "column_space" => {
            let r = f_column_space(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_643(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "condition_number" => {
            let r = f_condition_number(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_781(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "conjugate_gradient" => {
            let r = f_conjugate_gradient(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_887(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_product" => {
            let r = f_cross_product(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_916(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "csc_format" => {
            let r = f_csc_format(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "csr_format" => {
            let r = f_csr_format(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1049(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "csr_to_dense" => {
            let r = f_csr_to_dense(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("indices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("indptr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ncols") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1151(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "determinant" => {
            let r = f_determinant(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1219(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "determinant_2x2" => {
            let r = f_determinant_2x2(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1256(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "determinant_3x3" => {
            let r = f_determinant_3x3(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1305(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "distance_between_points" => {
            let r = f_distance_between_points(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1344(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dot_product" => {
            let r = f_dot_product(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1376(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "eigenvalues_2x2" => {
            let r = f_eigenvalues_2x2(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1425(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_legendre_integral" => {
            let r = f_gauss_legendre_integral(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_legendre_rule" => {
            let r = f_gauss_legendre_rule(match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1552(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gauss_seidel" => {
            let r = f_gauss_seidel(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1667(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "general_eigenvalues" => {
            let r = f_general_eigenvalues(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1794(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gram_schmidt" => {
            let r = f_gram_schmidt(match v.get("vectors") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1888(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "interpolating_polynomial" => {
            let r = f_interpolating_polynomial(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1979(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_iteration" => {
            let r = f_inverse_iteration(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2181(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "jacobi_iteration" => {
            let r = f_jacobi_iteration(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2278(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "kronecker_product" => {
            let r = f_kronecker_product(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lagrange_interpolation" => {
            let r = f_lagrange_interpolation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2391(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "least_squares_line" => {
            let r = f_least_squares_line(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2430(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "least_squares_polynomial" => {
            let r = f_least_squares_polynomial(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("degree") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2510(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "least_squares_solution" => {
            let r = f_least_squares_solution(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2620(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_independence" => {
            let r = f_linear_independence(match v.get("vectors") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2688(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_interpolation" => {
            let r = f_linear_interpolation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2749(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_system_classification" => {
            let r = f_linear_system_classification(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2845(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "low_rank_approximation" => {
            let r = f_low_rank_approximation(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2961(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lu_decomposition" => {
            let r = f_lu_decomposition(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_addition" => {
            let r = f_matrix_addition(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3130(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_exponential" => {
            let r = f_matrix_exponential(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_inverse" => {
            let r = f_matrix_inverse(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3289(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_multiplication" => {
            let r = f_matrix_multiplication(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_norms" => {
            let r = f_matrix_norms(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_power" => {
            let r = f_matrix_power(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3520(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_rank" => {
            let r = f_matrix_rank(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3597(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_square_root" => {
            let r = f_matrix_square_root(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3710(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_subtraction" => {
            let r = f_matrix_subtraction(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3777(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_trace" => {
            let r = f_matrix_trace(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3822(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_transpose" => {
            let r = f_matrix_transpose(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3860(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "matrix_vector_product" => {
            let r = f_matrix_vector_product(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3923(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "midpoint_rule" => {
            let r = f_midpoint_rule(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3971(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "minor_cofactor" => {
            let r = f_minor_cofactor(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("j") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4060(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "natural_cubic_spline" => {
            let r = f_natural_cubic_spline(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("at") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_divided_differences" => {
            let r = f_divided_differences(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4212(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_root" => {
            let r = f_newton_root(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "null_space" => {
            let r = f_null_space(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4370(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "numerical_derivative" => {
            let r = f_numerical_derivative(match v.get("y") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4409(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "outer_product" => {
            let r = f_outer_product(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4454(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "parallelogram_area" => {
            let r = f_parallelogram_area(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poisson_1d" => {
            let r = f_poisson_1d(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4525(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poisson_2d" => {
            let r = f_poisson_2d(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4595(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polar_decomposition" => {
            let r = f_polar_decomposition(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4712(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "polynomial_evaluation" => {
            let r = f_polynomial_evaluation(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4779(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "positive_definiteness" => {
            let r = f_positive_definiteness(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_iteration" => {
            let r = f_power_iteration(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5053(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "projection_matrix" => {
            let r = f_projection_matrix(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5140(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pseudoinverse" => {
            let r = f_pseudoinverse(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5237(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qr_decomposition" => {
            let r = f_qr_decomposition(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5361(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rotate_vector_2d" => {
            let r = f_rotate_vector_2d(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("angle") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5420(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rref" => {
            let r = f_rref(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5484(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "scalar_matrix_multiplication" => {
            let r = f_scalar_matrix_multiplication(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5540(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "scalar_vector_multiplication" => {
            let r = f_scalar_vector_multiplication(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5571(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "secant_root" => {
            let r = f_secant_root(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5656(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simpson_rule" => {
            let r = f_simpson_rule(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5696(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "singular_values" => {
            let r = f_singular_values(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solve_2x2_system" => {
            let r = f_solve_2x2_system(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5893(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solve_3x3_system" => {
            let r = f_solve_3x3_system(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5979(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "solve_linear_system" => {
            let r = f_solve_linear_system(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6069(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sparse_matvec" => {
            let r = f_sparse_matvec(match v.get("data") { Some(x) => py::from_val(x), None => { return None } }, match v.get("indices") { Some(x) => py::from_val(x), None => { return None } }, match v.get("indptr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "symmetric_eigenvalues" => {
            let r = f_symmetric_eigenvalues(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6232(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "symmetric_eigenvectors" => {
            let r = f_symmetric_eigenvectors(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6336(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "trapezoid_rule" => {
            let r = f_trapezoid_rule(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6415(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "tridiagonal_solve" => {
            let r = f_tridiagonal_solve(match v.get("sub") { Some(x) => py::from_val(x), None => { return None } }, match v.get("diag") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sup") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6466(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "triple_scalar_product" => {
            let r = f_triple_scalar_product(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6523(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "unit_vector" => {
            let r = f_unit_vector(match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6570(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_addition" => {
            let r = f_vector_addition(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6603(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_norm" => {
            let r = f_vector_norm(match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6633(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_projection" => {
            let r = f_vector_projection(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6670(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vector_subtraction" => {
            let r = f_vector_subtraction(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6707(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_BAD() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MAX_DIM() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Int(10i64)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__F(mut v_x: Py, mut v_name: Py) -> R {
    let _ = f__finite(v_x.clone(), v_name.clone())?;
    return Ok((if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() { py::fraction(&[py::int(&v_x.clone())?])? } else { py::fraction(&[Py::str(&py::repr(&py::float(&v_x.clone())?))])? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__FM(mut v_A: Py) -> R {
    return Ok({ let mut out = vec![]; for t6369 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6369;
out.push({ let mut out = vec![]; for t6370 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t6370;
out.push(f__F(v_x.clone(), Py::str("input"))?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__FV(mut v_v: Py) -> R {
    return Ok({ let mut out = vec![]; for t6704 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6704;
out.push(f__F(v_x.clone(), Py::str("input"))?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__T(mut v_A: Py) -> R {
    return Ok({ let mut out = vec![]; for t6362 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_A.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t6362;
out.push(Py::list((v_c.clone()).items()?)); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__compress(mut v_A: Py, mut v_by_rows: Py) -> R {
    let mut v_data: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_o: Py = Py::None;
    let mut v_outer: Py = Py::None;
    let mut v_ptr: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(50i64))?;
    let t1042 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t1042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1042[0].clone();
    v_n = t1042[1].clone();
    let t1043 = ((if (v_by_rows.clone()).truthy() { Py::tuple(vec![v_m.clone(), v_n.clone()]) } else { Py::tuple(vec![v_n.clone(), v_m.clone()]) })).items()?;
    if t1043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_outer = t1043[0].clone();
    v_inner = t1043[1].clone();
    let t1044 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![Py::Int(0i64)])])).items()?;
    if t1044.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_data = t1044[0].clone();
    v_idx = t1044[1].clone();
    v_ptr = t1044[2].clone();
    for t1045 in py::range_iter(&[(v_outer.clone()).i()?]) {
        v_o = t1045;
        for t1046 in py::range_iter(&[(v_inner.clone()).i()?]) {
            v_k = t1046;
            v_v = (if (v_by_rows.clone()).truthy() { py::getitem(&py::getitem(&v_A.clone(), &v_o.clone())?, &v_k.clone())? } else { py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_o.clone())? });
            if ({ let t1047 = v_v.clone(); Py::Bool({ let t1048 = Py::Int(0i64); if py::ne(&t1047, &t1048) { true } else { false } }) }).truthy() {
                let _ = py::method(&v_data.clone(), "append", &[py::float(&v_v.clone())?])?;
                let _ = py::method(&v_idx.clone(), "append", &[v_k.clone()])?;
            }
        }
        let _ = py::method(&v_ptr.clone(), "append", &[py::len(&v_data.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("data"), v_data.clone()), (Py::str("indices"), v_idx.clone()), (Py::str("indptr"), v_ptr.clone()), (Py::str("nnz"), py::len(&v_data.clone())?), (Py::str("density"), py::truediv(&py::len(&v_data.clone())?, &py::mul(&v_m.clone(), &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cos_sin_deg(mut v_t: Py) -> R {
    let mut v_exact: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_r = py::modulo(&v_t.clone(), &Py::Int(360i64))?;
    v_exact = Py::dict(vec![(Py::Int(0i64), Py::tuple(vec![Py::Float(1.0), Py::Float(0.0)])), (Py::Int(90i64), Py::tuple(vec![Py::Float(0.0), Py::Float(1.0)])), (Py::Int(180i64), Py::tuple(vec![py::neg(&Py::Float(1.0))?, Py::Float(0.0)])), (Py::Int(270i64), Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?]))]);
    if ({ let t5417 = v_r.clone(); Py::Bool({ let t5418 = v_exact.clone(); if py::contains(&t5418, &t5417)? { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&v_exact.clone(), &v_r.clone())?);
    }
    return Ok(Py::tuple(vec![py::math1("cos", &py::math1("radians", &v_r.clone())?)?, py::math1("sin", &py::math1("radians", &v_r.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cramer(mut v_A: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_Aj: Py = Py::None;
    let mut v_FA: Py = Py::None;
    let mut v_Fb: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_sol: Py = Py::None;
    let t5955 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), v_n.clone(), Py::None, c_MAX_DIM()?)?, f__need_vec(v_b.clone(), Py::str("b"), v_n.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t5955.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5955[0].clone();
    v_b = t5955[1].clone();
    let t5959 = (Py::tuple(vec![f__FM(v_A.clone())?, f__FV(v_b.clone())?])).items()?;
    if t5959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FA = t5959[0].clone();
    v_Fb = t5959[1].clone();
    v_d = f__det_F(v_FA.clone())?;
    if ({ let t5972 = v_d.clone(); Py::Bool({ let t5973 = Py::Int(0i64); if py::eq(&t5972, &t5973) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the determinant of the coefficient matrix is 0: the system has no unique solution"))));
    }
    v_sol = Py::list(vec![]);
    for t5974 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t5974;
        v_Aj = { let mut out = vec![]; for t5975 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5975;
out.push({ let mut out = vec![]; for t5976 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t5976;
out.push((if ({ let t5977 = v_c.clone(); Py::Bool({ let t5978 = v_j.clone(); if py::eq(&t5977, &t5978) { true } else { false } }) }).truthy() { py::getitem(&v_Fb.clone(), &v_r.clone())? } else { py::getitem(&py::getitem(&v_FA.clone(), &v_r.clone())?, &v_c.clone())? })); } Py::list(out) }); } Py::list(out) };
        let _ = py::method(&v_sol.clone(), "append", &[py::float(&py::truediv(&f__det_F(v_Aj.clone())?, &v_d.clone())?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("solution"), v_sol.clone()), (Py::str("det"), py::float(&v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__det_F(mut v_M: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_M = { let mut out = vec![]; for t6510 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t6510;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    let t6511 = (Py::tuple(vec![py::len(&v_M.clone())?, py::fraction(&[Py::Int(1i64)])?])).items()?;
    if t6511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6511[0].clone();
    v_det = t6511[1].clone();
    for t6512 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t6512;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_M = v_M.clone(); let v_c = v_c.clone(); let v_det = v_det.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t6513 = py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?; Py::Bool({ let t6514 = Py::Int(0i64); if py::eq(&t6513, &t6514) { true } else { false } }) }).truthy() {
            return Ok(py::fraction(&[Py::Int(0i64)])?);
        }
        if ({ let t6515 = v_p.clone(); Py::Bool({ let t6516 = v_c.clone(); if py::ne(&t6515, &t6516) { true } else { false } }) }).truthy() {
            let t6517 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
            if t6517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_M.clone(), &v_c.clone(), t6517[0].clone())?;
            py::setitem(&v_M.clone(), &v_p.clone(), t6517[1].clone())?;
            v_det = py::neg(&v_det.clone())?;
        }
        v_det = py::mul(&v_det, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
        for t6518 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t6518;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if (v_f.clone()).truthy() {
                for t6519 in py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?]) {
                    v_k = t6519;
                    let t6520 = py::getitem(&v_M.clone(), &v_r.clone())?; let t6521 = v_k.clone();
                    py::setitem(&t6520, &t6521, py::sub(&py::getitem(&t6520, &t6521)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    return Ok(v_det.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_x: Py, mut v_y: Py) -> R {
    if ({ let t6528 = Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_))); if t6528.truthy() { t6528 } else { Py::Bool(matches!(v_y.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t6532 = py::not(&{ let t6529 = Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_))); if !t6529.truthy() { t6529 } else { Py::Bool(matches!(v_y.clone(), Py::List(_) | Py::Tuple(_))) } }); if t6532.truthy() { t6532 } else { { let t6530 = py::len(&v_x.clone())?; Py::Bool({ let t6531 = py::len(&v_y.clone())?; if py::ne(&t6530, &t6531) { true } else { false } }) } } }).truthy() {
            return Ok(c_BAD()?);
        }
        return Ok({ let t = { let mut out = vec![]; for t6533 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6534 = (t6533).items()?;
if t6534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6534[0].clone();
v_b = t6534[1].clone();
out.push(f__diff(v_a.clone(), v_b.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t6537 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t6537.truthy() { t6537 } else { { let t6536 = Py::Bool(matches!(v_y.clone(), Py::Bool(_))); if t6536.truthy() { t6536 } else { { let t6535 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t6535.truthy() { t6535 } else { Py::Bool(matches!(v_y.clone(), Py::Str(_))) } } } } } }).truthy() {
        return Ok((if ({ let t6538 = v_x.clone(); Py::Bool({ let t6539 = v_y.clone(); if py::eq(&t6538, &t6539) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? }));
    }
    return Ok(f__rel(v_x.clone(), v_y.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dot(mut v_u: Py, mut v_v: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t6675 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6676 = (t6675).items()?;
if t6676.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6676[0].clone();
v_b = t6676[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eig_qr(mut v_A: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cc: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rots: Py = Py::None;
    let mut v_ss: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let mut v_total: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_H = { let mut out = vec![]; for t4934 in py::iter(&(f__hessenberg(v_A.clone())?))? { let mut v_r: Py; 
v_r = t4934;
out.push({ let mut out = vec![]; for t4935 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4935;
out.push(py::complex(&[v_x.clone()])?); } Py::list(out) }); } Py::list(out) };
    v_eps = Py::Float(2.2e-16);
    let t4936 = (Py::tuple(vec![Py::list(vec![]), py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(0i64), Py::Int(0i64)])).items()?;
    if t4936.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_out = t4936[0].clone();
    v_hi = t4936[1].clone();
    v_it = t4936[2].clone();
    v_total = t4936[3].clone();
    while ({ let t4974 = v_hi.clone(); Py::Bool({ let t4975 = Py::Int(0i64); if py::ge(&t4974, &t4975)? { true } else { false } }) }).truthy() {
        if ({ let t4937 = v_hi.clone(); Py::Bool({ let t4938 = Py::Int(0i64); if py::eq(&t4937, &t4938) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::getitem(&py::getitem(&v_H.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?])?;
            break;
        }
        v_l = v_hi.clone();
        while ({ let t4946 = { let t4939 = v_l.clone(); Py::Bool({ let t4940 = Py::Int(0i64); if py::gt(&t4939, &t4940)? { true } else { false } }) }; if !t4946.truthy() { t4946 } else { { let t4945 = { let t4941 = py::abs(&py::getitem(&py::getitem(&v_H.clone(), &v_l.clone())?, &py::sub(&v_l.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t4942 = py::mul(&v_eps.clone(), &py::add(&py::abs(&py::getitem(&py::getitem(&v_H.clone(), &v_l.clone())?, &v_l.clone())?)?, &py::abs(&py::getitem(&py::getitem(&v_H.clone(), &py::sub(&v_l.clone(), &Py::Int(1i64))?)?, &py::sub(&v_l.clone(), &Py::Int(1i64))?)?)?)?)?; if py::gt(&t4941, &t4942)? { true } else { false } }) }; if !t4945.truthy() { t4945 } else { { let t4943 = py::abs(&py::getitem(&py::getitem(&v_H.clone(), &v_l.clone())?, &py::sub(&v_l.clone(), &Py::Int(1i64))?)?)?; Py::Bool({ let t4944 = Py::Float(1e-300); if py::gt(&t4943, &t4944)? { true } else { false } }) } } } } }).truthy() {
            v_l = py::sub(&v_l, &Py::Int(1i64))?;
        }
        if ({ let t4947 = v_l.clone(); Py::Bool({ let t4948 = v_hi.clone(); if py::eq(&t4947, &t4948) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::getitem(&py::getitem(&v_H.clone(), &v_hi.clone())?, &v_hi.clone())?])?;
            let t4949 = (Py::tuple(vec![py::sub(&v_hi.clone(), &Py::Int(1i64))?, Py::Int(0i64)])).items()?;
            if t4949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_hi = t4949[0].clone();
            v_it = t4949[1].clone();
            continue;
        }
        let t4950 = (Py::tuple(vec![py::getitem(&py::getitem(&v_H.clone(), &py::sub(&v_hi.clone(), &Py::Int(1i64))?)?, &py::sub(&v_hi.clone(), &Py::Int(1i64))?)?, py::getitem(&py::getitem(&v_H.clone(), &py::sub(&v_hi.clone(), &Py::Int(1i64))?)?, &v_hi.clone())?, py::getitem(&py::getitem(&v_H.clone(), &v_hi.clone())?, &py::sub(&v_hi.clone(), &Py::Int(1i64))?)?, py::getitem(&py::getitem(&v_H.clone(), &v_hi.clone())?, &v_hi.clone())?])).items()?;
        if t4950.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_a = t4950[0].clone();
        v_b = t4950[1].clone();
        v_c = t4950[2].clone();
        v_d = t4950[3].clone();
        let t4951 = (Py::tuple(vec![py::add(&v_a.clone(), &v_d.clone())?, py::sub(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)?])).items()?;
        if t4951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_tr = t4951[0].clone();
        v_det = t4951[1].clone();
        v_disc = py::cmath("sqrt", &[py::sub(&py::truediv(&py::mul(&v_tr.clone(), &v_tr.clone())?, &Py::Int(4i64))?, &v_det.clone())?])?;
        v_mu = py::minmax_key(&[Py::tuple(vec![py::add(&py::truediv(&v_tr.clone(), &Py::Int(2i64))?, &v_disc.clone())?, py::sub(&py::truediv(&v_tr.clone(), &Py::Int(2i64))?, &v_disc.clone())?])], &{ let v_A = v_A.clone(); let v_H = v_H.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_det = v_det.clone(); let v_disc = v_disc.clone(); let v_eps = v_eps.clone(); let v_hi = v_hi.clone(); let v_it = v_it.clone(); let v_l = v_l.clone(); let v_n = v_n.clone(); let v_out = v_out.clone(); let v_total = v_total.clone(); let v_tr = v_tr.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&v_z.clone(), &v_d.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        if ({ let t4954 = v_it.clone(); if !t4954.truthy() { t4954 } else { { let t4952 = py::modulo(&v_it.clone(), &Py::Int(11i64))?; Py::Bool({ let t4953 = Py::Int(0i64); if py::eq(&t4952, &t4953) { true } else { false } }) } } }).truthy() {
            v_mu = py::add(&v_d.clone(), &py::abs(&v_c.clone())?)?;
        }
        for t4955 in py::range_iter(&[(v_l.clone()).i()?, (py::add(&v_hi.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t4955;
            let t4956 = py::getitem(&v_H.clone(), &v_k.clone())?; let t4957 = v_k.clone();
            py::setitem(&t4956, &t4957, py::sub(&py::getitem(&t4956, &t4957)?, &v_mu.clone())?)?;
        }
        v_rots = Py::list(vec![]);
        for t4958 in py::range_iter(&[(v_l.clone()).i()?, (v_hi.clone()).i()?]) {
            v_k = t4958;
            let t4959 = (Py::tuple(vec![py::getitem(&py::getitem(&v_H.clone(), &v_k.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_H.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_k.clone())?])).items()?;
            if t4959.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t4959[0].clone();
            v_y = t4959[1].clone();
            v_r = py::hypot(&[py::abs(&v_x.clone())?, py::abs(&v_y.clone())?])?;
            let t4960 = ((if (v_r.clone()).truthy() { Py::tuple(vec![py::truediv(&v_x.clone(), &v_r.clone())?, py::truediv(&v_y.clone(), &v_r.clone())?]) } else { Py::tuple(vec![py::add(&Py::Float(1.0), &Py::Complex(0.0, 0.0))?, Py::Complex(0.0, 0.0)]) })).items()?;
            if t4960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cc = t4960[0].clone();
            v_ss = t4960[1].clone();
            for t4961 in py::range_iter(&[(v_k.clone()).i()?, (py::add(&v_hi.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t4961;
                let t4962 = (Py::tuple(vec![py::getitem(&py::getitem(&v_H.clone(), &v_k.clone())?, &v_j.clone())?, py::getitem(&py::getitem(&v_H.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_j.clone())?])).items()?;
                if t4962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_t1 = t4962[0].clone();
                v_t2 = t4962[1].clone();
                py::setitem(&py::getitem(&v_H.clone(), &v_k.clone())?, &v_j.clone(), py::add(&py::mul(&py::method(&v_cc.clone(), "conjugate", &[])?, &v_t1.clone())?, &py::mul(&py::method(&v_ss.clone(), "conjugate", &[])?, &v_t2.clone())?)?)?;
                py::setitem(&py::getitem(&v_H.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_j.clone(), py::add(&py::mul(&py::neg(&v_ss.clone())?, &v_t1.clone())?, &py::mul(&v_cc.clone(), &v_t2.clone())?)?)?;
            }
            let _ = py::method(&v_rots.clone(), "append", &[Py::tuple(vec![v_cc.clone(), v_ss.clone()])])?;
        }
        for t4963 in py::iter(&(py::enumerate(&v_rots.clone(), (v_l.clone()).i()?)?))? {
            let t4966 = (t4963).items()?;
            if t4966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_k = t4966[0].clone();
            let t4967 = (t4966[1].clone()).items()?;
            if t4967.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_cc = t4967[0].clone();
            v_ss = t4967[1].clone();
            for t4964 in py::range_iter(&[(v_l.clone()).i()?, (py::add(&py::minmax(&[py::add(&v_k.clone(), &Py::Int(2i64))?, v_hi.clone()], false)?, &Py::Int(1i64))?).i()?]) {
                v_i = t4964;
                let t4965 = (Py::tuple(vec![py::getitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])).items()?;
                if t4965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_t1 = t4965[0].clone();
                v_t2 = t4965[1].clone();
                py::setitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &v_k.clone(), py::add(&py::mul(&v_t1.clone(), &v_cc.clone())?, &py::mul(&v_t2.clone(), &v_ss.clone())?)?)?;
                py::setitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?, py::add(&py::mul(&py::neg(&v_t1.clone())?, &py::method(&v_ss.clone(), "conjugate", &[])?)?, &py::mul(&v_t2.clone(), &py::method(&v_cc.clone(), "conjugate", &[])?)?)?)?;
            }
        }
        for t4968 in py::range_iter(&[(v_l.clone()).i()?, (py::add(&v_hi.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t4968;
            let t4969 = py::getitem(&v_H.clone(), &v_k.clone())?; let t4970 = v_k.clone();
            py::setitem(&t4969, &t4970, py::iadd(&py::getitem(&t4969, &t4970)?, &v_mu.clone())?)?;
        }
        let t4971 = (Py::tuple(vec![py::add(&v_it.clone(), &Py::Int(1i64))?, py::add(&v_total.clone(), &Py::Int(1i64))?])).items()?;
        if t4971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_it = t4971[0].clone();
        v_total = t4971[1].clone();
        if ({ let t4972 = v_total.clone(); Py::Bool({ let t4973 = py::mul(&Py::Int(200i64), &v_n.clone())?; if py::gt(&t4972, &t4973)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the QR algorithm did not converge"))));
        }
    }
    return Ok(f__real_spectrum(v_out.clone(), f__fro(v_A.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__expand(mut v_data: Py, mut v_indices: Py, mut v_indptr: Py, mut v_outer: Py, mut v_inner: Py, mut v_by_rows: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_o: Py = Py::None;
    v_M = { let mut out = vec![]; for t1081 in py::range_iter(&[(v_outer.clone()).i()?]) { let mut v__: Py; 
v__ = t1081;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_inner.clone())?); } Py::list(out) };
    for t1082 in py::range_iter(&[(v_outer.clone()).i()?]) {
        v_o = t1082;
        for t1083 in py::range_iter(&[(py::int(&py::getitem(&v_indptr.clone(), &v_o.clone())?)?).i()?, (py::int(&py::getitem(&v_indptr.clone(), &py::add(&v_o.clone(), &Py::Int(1i64))?)?)?).i()?]) {
            v_k = t1083;
            let t1084 = py::getitem(&v_M.clone(), &v_o.clone())?; let t1085 = py::int(&py::getitem(&v_indices.clone(), &v_k.clone())?)?;
            py::setitem(&t1084, &t1085, py::iadd(&py::getitem(&t1084, &t1085)?, &py::getitem(&v_data.clone(), &v_k.clone())?)?)?;
        }
    }
    return Ok((if (v_by_rows.clone()).truthy() { v_M.clone() } else { f__T(v_M.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__expm(mut v_A: Py, mut v_extra: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_norm1: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_norm1 = py::minmax(&[{ let mut out = vec![]; for t3178 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3178;
out.push(py::fsum(&{ let mut out = vec![]; for t3179 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3179;
out.push(py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }], true)?;
    v_s = (if ({ let t3180 = v_norm1.clone(); Py::Bool({ let t3181 = Py::Float(0.5); if py::gt(&t3180, &t3181)? { true } else { false } }) }).truthy() { py::add(&py::minmax(&[Py::Int(0i64), py::math1("ceil", &py::math1("log2", &py::truediv(&v_norm1.clone(), &Py::Float(0.5))?)?)?], true)?, &v_extra.clone())? } else { v_extra.clone() });
    v_B = { let mut out = vec![]; for t3182 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3182;
out.push({ let mut out = vec![]; for t3183 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3183;
out.push(py::truediv(&py::float(&v_x.clone())?, &py::pow(&Py::Int(2i64), &v_s.clone())?)?); } Py::list(out) }); } Py::list(out) };
    let t3188 = (Py::tuple(vec![f__eye(v_n.clone())?, f__eye(v_n.clone())?])).items()?;
    if t3188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_E = t3188[0].clone();
    v_T = t3188[1].clone();
    for t3189 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(40i64)).i()?]) {
        v_k = t3189;
        v_T = { let mut out = vec![]; for t3194 in py::iter(&(f__mm(v_T.clone(), v_B.clone())?))? { let mut v_r: Py; 
v_r = t3194;
out.push({ let mut out = vec![]; for t3195 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3195;
out.push(py::truediv(&v_x.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
        v_E = { let mut out = vec![]; for t3196 in py::iter(&(py::zip(&[v_E.clone(), v_T.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t3197 = (t3196).items()?;
if t3197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3197[0].clone();
v_t = t3197[1].clone();
out.push({ let mut out = vec![]; for t3198 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3199 = (t3198).items()?;
if t3199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3199[0].clone();
v_b = t3199[1].clone();
out.push(py::add(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) };
        if ({ let t3200 = py::minmax(&[{ let mut out = vec![]; for t3202 in py::iter(&(v_T.clone()))? { let mut v_r: Py; 
v_r = t3202;
for t3203 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3203;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?; Py::Bool({ let t3201 = py::mul(&Py::Float(1e-18), &py::minmax(&[{ let mut out = vec![]; for t3204 in py::iter(&(v_E.clone()))? { let mut v_r: Py; 
v_r = t3204;
for t3205 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3205;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?)?; if py::le(&t3200, &t3201)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    for t3206 in py::range_iter(&[(v_s.clone()).i()?]) {
        v__ = t3206;
        v_E = f__mm(v_E.clone(), v_E.clone())?;
    }
    return Ok(v_E.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eye(mut v_n: Py) -> R {
    return Ok({ let mut out = vec![]; for t6290 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6290;
out.push({ let mut out = vec![]; for t6291 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6291;
out.push((if ({ let t6292 = v_i.clone(); Py::Bool({ let t6293 = v_j.clone(); if py::eq(&t6292, &t6293) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__finite(mut v_x: Py, mut v_name: Py) -> R {
    if ({ let t6688 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t6688.truthy() { t6688 } else { { let t6687 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t6687.truthy() { t6687 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must contain finite real numbers".to_string()].concat()))));
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fixed_point(mut v_step: Py, mut v_x: Py, mut v_A: Py, mut v_b: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_best = Py::None;
    for t2261 in py::range_iter(&[(Py::Int(50000i64)).i()?]) {
        v__ = t2261;
        v_y = py::call(&v_step.clone(), &[v_x.clone()])?;
        v_d = py::minmax(&[{ let mut out = vec![]; for t2262 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_p: Py; let mut v_q: Py; 
let t2263 = (t2262).items()?;
if t2263.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t2263[0].clone();
v_q = t2263[1].clone();
out.push(py::abs(&py::sub(&v_p.clone(), &v_q.clone())?)?); } Py::list(out) }], true)?;
        v_x = v_y.clone();
        if ({ let t2264 = v_d.clone(); Py::Bool({ let t2265 = py::mul(&Py::Float(2.3e-16), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2266 in py::iter(&(v_x.clone()))? { let mut v_t: Py; 
v_t = t2266;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t2264, &t2265)? { true } else { false } }) }).truthy() {
            return Ok(v_x.clone());
        }
        if ({ let t2275 = { let t2267 = v_best.clone(); Py::Bool({ let t2268 = Py::None; if !py::is(&t2267, &t2268) { true } else { false } }) }; if !t2275.truthy() { t2275 } else { { let t2274 = { let t2269 = v_d.clone(); Py::Bool({ let t2270 = v_best.clone(); if py::ge(&t2269, &t2270)? { true } else { false } }) }; if !t2274.truthy() { t2274 } else { { let t2271 = v_d.clone(); Py::Bool({ let t2272 = py::mul(&Py::Float(1e-13), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2273 in py::iter(&(v_x.clone()))? { let mut v_t: Py; 
v_t = t2273;
out.push(py::abs(&v_t.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t2271, &t2272)? { true } else { false } }) } } } } }).truthy() {
            return Ok(v_x.clone());
        }
        v_best = (if ({ let t2276 = v_best.clone(); Py::Bool({ let t2277 = Py::None; if py::is(&t2276, &t2277) { true } else { false } }) }).truthy() { v_d.clone() } else { py::minmax(&[v_best.clone(), v_d.clone()], false)? });
    }
    return Err(py::value_error(py::to_str(&Py::str("the iteration did not converge"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fl(mut v_M: Py) -> R {
    if (Py::Bool(matches!(v_M.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t6465 in py::iter(&(v_M.clone()))? { let mut v_x: Py; 
v_x = t6465;
out.push(f__fl(v_x.clone())?); } Py::list(out) });
    }
    return Ok(py::float(&v_M.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fro(mut v_A: Py) -> R {
    return Ok(py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t6487 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6487;
for t6488 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t6488;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); }} Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gl_check(mut v_c: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_half: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_right: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let mut v_xs: Py = Py::None;
    let t1508 = (f__legendre_nodes(v_n.clone())?).items()?;
    if t1508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t1508[0].clone();
    v_ws = t1508[1].clone();
    let t1509 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?])).items()?;
    if t1509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mid = t1509[0].clone();
    v_half = t1509[1].clone();
    v_left = py::fsum(&{ let mut out = vec![]; for t1510 in py::iter(&(py::zip(&[v_xs.clone(), v_ws.clone()])?))? { let mut v_w: Py; let mut v_x: Py; 
let t1511 = (t1510).items()?;
if t1511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1511[0].clone();
v_w = t1511[1].clone();
if ({ let t1512 = v_x.clone(); Py::Bool({ let t1513 = Py::Int(0i64); if py::lt(&t1512, &t1513)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_w.clone(), &f__peval(Py::list((v_c.clone()).items()?), py::add(&v_mid.clone(), &py::mul(&v_half.clone(), &v_x.clone())?)?)?)?); }} Py::list(out) })?;
    v_right = py::fsum(&{ let mut out = vec![]; for t1514 in py::iter(&(py::zip(&[v_xs.clone(), v_ws.clone()])?))? { let mut v_w: Py; let mut v_x: Py; 
let t1515 = (t1514).items()?;
if t1515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1515[0].clone();
v_w = t1515[1].clone();
if ({ let t1516 = v_x.clone(); Py::Bool({ let t1517 = Py::Int(0i64); if py::ge(&t1516, &t1517)? { true } else { false } }) }).truthy() { out.push(py::mul(&v_w.clone(), &f__peval(Py::list((v_c.clone()).items()?), py::add(&v_mid.clone(), &py::mul(&v_half.clone(), &v_x.clone())?)?)?)?); }} Py::list(out) })?;
    return Ok(py::mul(&v_half.clone(), &py::add(&v_left.clone(), &v_right.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gram_power_sums(mut v_A: Py, mut v_kmax: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_F = f__FM(v_A.clone())?;
    v_G = (if ({ let t5806 = py::len(&v_A.clone())?; Py::Bool({ let t5807 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::ge(&t5806, &t5807)? { true } else { false } }) }).truthy() { f__mm_F(f__T(v_F.clone())?, v_F.clone())? } else { f__mm_F(v_F.clone(), f__T(v_F.clone())?)? });
    let t5813 = (Py::tuple(vec![{ let mut out = vec![]; for t5812 in py::iter(&(v_G.clone()))? { let mut v_r: Py; 
v_r = t5812;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) }, Py::list(vec![])])).items()?;
    if t5813.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t5813[0].clone();
    v_out = t5813[1].clone();
    for t5814 in py::range_iter(&[(v_kmax.clone()).i()?]) {
        v__ = t5814;
        let _ = py::method(&v_out.clone(), "append", &[py::float(&py::sum(&{ let mut out = vec![]; for t5815 in py::range_iter(&[(py::len(&v_G.clone())?).i()?]) { let mut v_i: Py; 
v_i = t5815;
out.push(py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?])?;
        v_P = f__mm_F(v_P.clone(), v_G.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hessenberg(mut v_A: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_H = { let mut out = vec![]; for t4911 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t4911;
out.push({ let mut out = vec![]; for t4912 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4912;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    for t4913 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(2i64))?).i()?]) {
        v_k = t4913;
        v_x = { let mut out = vec![]; for t4914 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4914;
out.push(py::getitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &v_k.clone())?); } Py::list(out) };
        v_nx = f__norm(v_x.clone())?;
        if ({ let t4916 = v_nx.clone(); Py::Bool({ let t4917 = Py::Int(0i64); if py::eq(&t4916, &t4917) { true } else { false } }) }).truthy() {
            continue;
        }
        v_alpha = py::neg(&py::copysign(&v_nx.clone(), &py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)?;
        v_v = py::slice(&v_x.clone(), None, None, None)?;
        let t4918 = v_v.clone(); let t4919 = Py::Int(0i64);
        py::setitem(&t4918, &t4919, py::sub(&py::getitem(&t4918, &t4919)?, &v_alpha.clone())?)?;
        v_nv = f__norm(v_v.clone())?;
        if ({ let t4920 = v_nv.clone(); Py::Bool({ let t4921 = Py::Int(0i64); if py::eq(&t4920, &t4921) { true } else { false } }) }).truthy() {
            continue;
        }
        v_v = { let mut out = vec![]; for t4922 in py::iter(&(v_v.clone()))? { let mut v_t: Py; 
v_t = t4922;
out.push(py::truediv(&v_t.clone(), &v_nv.clone())?); } Py::list(out) };
        v_m = py::len(&v_v.clone())?;
        for t4923 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t4923;
            v_d = py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t4924 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t4924;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_H.clone(), &py::add(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?)?, &v_j.clone())?)?); } Py::list(out) })?)?;
            for t4925 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_i = t4925;
                let t4926 = py::getitem(&v_H.clone(), &py::add(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_i.clone())?)?; let t4927 = v_j.clone();
                py::setitem(&t4926, &t4927, py::sub(&py::getitem(&t4926, &t4927)?, &py::mul(&v_d.clone(), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?)?;
            }
        }
        for t4928 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t4928;
            v_d = py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t4929 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t4929;
out.push(py::mul(&py::getitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &py::add(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_j.clone())?)?, &py::getitem(&v_v.clone(), &v_j.clone())?)?); } Py::list(out) })?)?;
            for t4930 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_j = t4930;
                let t4931 = py::getitem(&v_H.clone(), &v_i.clone())?; let t4932 = py::add(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_j.clone())?;
                py::setitem(&t4931, &t4932, py::sub(&py::getitem(&t4931, &t4932)?, &py::mul(&v_d.clone(), &py::getitem(&v_v.clone(), &v_j.clone())?)?)?)?;
            }
        }
        for t4933 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(2i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t4933;
            py::setitem(&py::getitem(&v_H.clone(), &v_i.clone())?, &v_k.clone(), Py::Float(0.0))?;
        }
    }
    return Ok(v_H.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__inv_F(mut v_M: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_aug: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_aug = { let mut out = vec![]; for t5225 in py::iter(&(py::enumerate(&v_M.clone(), 0)?))? { let mut v_i: Py; let mut v_r: Py; 
let t5226 = (t5225).items()?;
if t5226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t5226[0].clone();
v_r = t5226[1].clone();
out.push(py::add(&Py::list((v_r.clone()).items()?), &{ let mut out = vec![]; for t5227 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5227;
out.push(py::fraction(&[py::int(&{ let t5228 = v_i.clone(); Py::Bool({ let t5229 = v_j.clone(); if py::eq(&t5228, &t5229) { true } else { false } }) })?])?); } Py::list(out) })?); } Py::list(out) };
    let t5230 = (f__rref_F(v_aug.clone())?).items()?;
    if t5230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t5230[0].clone();
    v_piv = t5230[1].clone();
    if ({ let t5231 = py::slice(&v_piv.clone(), None, Some((v_n.clone()).i()?), None)?; Py::Bool({ let t5232 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t5231, &t5232) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix is singular (its determinant is 0), so it has no inverse"))));
    }
    return Ok({ let mut out = vec![]; for t5233 in py::iter(&(v_R.clone()))? { let mut v_r: Py; 
v_r = t5233;
out.push(py::slice(&v_r.clone(), Some((v_n.clone()).i()?), None, None)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__iterate(mut v_A: Py, mut v_step: Py, mut v_extreme: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_lead: Py = Py::None;
    let mut v_mods: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_second: Py = Py::None;
    let mut v_start: Py = Py::None;
    let mut v_target: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_ev = f__eig_qr(v_A.clone())?;
    v_mods = py::sorted(&{ let mut out = vec![]; for t4990 in py::iter(&(v_ev.clone()))? { let mut v_z: Py; 
v_z = t4990;
out.push(py::abs(&py::complex(&[v_z.clone()])?)?); } Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_target = (if ({ let t4991 = v_extreme.clone(); Py::Bool({ let t4992 = Py::str("max"); if py::eq(&t4991, &t4992) { true } else { false } }) }).truthy() { py::minmax_key(&[v_ev.clone()], &{ let v_A = v_A.clone(); let v_ev = v_ev.clone(); let v_extreme = v_extreme.clone(); let v_mods = v_mods.clone(); let v_n = v_n.clone(); let v_step = v_step.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::complex(&[v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)? } else { py::minmax_key(&[v_ev.clone()], &{ let v_A = v_A.clone(); let v_ev = v_ev.clone(); let v_extreme = v_extreme.clone(); let v_mods = v_mods.clone(); let v_n = v_n.clone(); let v_step = v_step.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::complex(&[v_z.clone()])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)? });
    v_second = (if ({ let t4993 = v_extreme.clone(); Py::Bool({ let t4994 = Py::str("max"); if py::eq(&t4993, &t4994) { true } else { false } }) }).truthy() { py::getitem(&v_mods.clone(), &py::neg(&Py::Int(2i64))?)? } else { py::getitem(&v_mods.clone(), &Py::Int(1i64))? });
    v_lead = (if ({ let t4995 = v_extreme.clone(); Py::Bool({ let t4996 = Py::str("max"); if py::eq(&t4995, &t4996) { true } else { false } }) }).truthy() { py::getitem(&v_mods.clone(), &py::neg(&Py::Int(1i64))?)? } else { py::getitem(&v_mods.clone(), &Py::Int(0i64))? });
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    if ({ let t4999 = Py::Bool(matches!(v_target.clone(), Py::Complex(..))); if t4999.truthy() { t4999 } else { { let t4997 = py::abs(&py::sub(&v_lead.clone(), &v_second.clone())?)?; Py::Bool({ let t4998 = py::mul(&Py::Float(1e-06), &v_sc.clone())?; if py::le(&t4997, &t4998)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&py::add(&Py::str("there is no single real eigenvalue of "), &(if ({ let t5000 = v_extreme.clone(); Py::Bool({ let t5001 = Py::str("max"); if py::eq(&t5000, &t5001) { true } else { false } }) }).truthy() { Py::str("largest") } else { Py::str("smallest") }))?, &Py::str(" magnitude, so the iteration does not converge"))?)));
    }
    for t5004 in py::iter(&(Py::tuple(vec![{ let mut out = vec![]; for t5002 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5002;
out.push(py::add(&Py::Float(1.0), &py::truediv(&v_k.clone(), &py::mul(&Py::Float(2.0), &v_n.clone())?)?)?); } Py::list(out) }, { let mut out = vec![]; for t5003 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5003;
out.push(py::mul(&py::pow(&py::neg(&Py::Float(1.0))?, &v_k.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?); } Py::list(out) }, py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?])))? {
        v_start = t5004;
        v_x = f__unit(v_start.clone())?;
        for t5013 in py::range_iter(&[(Py::Int(20000i64)).i()?]) {
            v__ = t5013;
            v_y = f__unit(py::call(&v_step.clone(), &[v_x.clone()])?)?;
            if ({ let t5014 = f__norm({ let mut out = vec![]; for t5016 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5017 = (t5016).items()?;
if t5017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5017[0].clone();
v_b = t5017[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?; Py::Bool({ let t5015 = Py::Float(1e-10); if py::lt(&t5014, &t5015)? { true } else { false } }) }).truthy() {
                v_x = v_y.clone();
                break;
            }
            v_x = v_y.clone();
        }
        v_lam = py::truediv(&f__dot(v_x.clone(), f__mv(v_A.clone(), v_x.clone())?)?, &f__dot(v_x.clone(), v_x.clone())?)?;
        let t5045 = (f__refine(v_A.clone(), v_lam.clone(), v_x.clone(), Py::Int(4i64))?).items()?;
        if t5045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lam = t5045[0].clone();
        v_x = t5045[1].clone();
        if ({ let t5052 = { let t5046 = py::abs(&py::sub(&v_lam.clone(), &py::float(&v_target.clone())?)?)?; Py::Bool({ let t5047 = py::mul(&Py::Float(1e-08), &v_sc.clone())?; if py::le(&t5046, &t5047)? { true } else { false } }) }; if !t5052.truthy() { t5052 } else { { let t5048 = f__norm({ let mut out = vec![]; for t5050 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_x.clone())?, v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5051 = (t5050).items()?;
if t5051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5051[0].clone();
v_b = t5051[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_lam.clone(), &v_b.clone())?)?); } Py::list(out) })?; Py::Bool({ let t5049 = py::mul(&Py::Float(1e-12), &v_sc.clone())?; if py::le(&t5048, &t5049)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::dict(vec![(Py::str("eigenvalue"), v_lam.clone()), (Py::str("eigenvector"), v_x.clone())]));
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("the iteration did not converge to the extreme eigenvalue"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jacobi_eig(mut v_A: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_akp: Py = Py::None;
    let mut v_akq: Py = Py::None;
    let mut v_apk: Py = Py::None;
    let mut v_aqk: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fro: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_off: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_theta: Py = Py::None;
    let mut v_vkp: Py = Py::None;
    let mut v_vkq: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_a = { let mut out = vec![]; for t6288 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6288;
out.push({ let mut out = vec![]; for t6289 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t6289;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_V = f__eye(v_n.clone())?;
    v_fro = f__fro(v_a.clone())?;
    let mut t6296 = false;
    for t6297 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t6297;
        v_off = py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t6298 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6298;
for t6299 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6299;
out.push(py::pow(&py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_j.clone())?, &Py::Int(2i64))?); }} Py::list(out) })?)?;
        if ({ let t6304 = { let t6300 = v_off.clone(); Py::Bool({ let t6301 = Py::Int(0i64); if py::eq(&t6300, &t6301) { true } else { false } }) }; if t6304.truthy() { t6304 } else { { let t6302 = v_off.clone(); Py::Bool({ let t6303 = py::mul(&Py::Float(1e-17), &v_fro.clone())?; if py::le(&t6302, &t6303)? { true } else { false } }) } } }).truthy() {
            { t6296 = true; break; }
        }
        for t6305 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_p = t6305;
            for t6306 in py::range_iter(&[(py::add(&v_p.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_q = t6306;
                if ({ let t6307 = py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?; Py::Bool({ let t6308 = Py::Int(0i64); if py::eq(&t6307, &t6308) { true } else { false } }) }).truthy() {
                    continue;
                }
                v_theta = py::truediv(&py::sub(&py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_q.clone())?, &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_p.clone())?)?, &py::mul(&Py::Int(2i64), &py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_q.clone())?)?)?;
                v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_theta.clone())?, &py::add(&py::abs(&v_theta.clone())?, &py::hypot(&[Py::Float(1.0), v_theta.clone()])?)?)?;
                v_c = py::truediv(&Py::Int(1i64), &py::hypot(&[Py::Float(1.0), v_t.clone()])?)?;
                v_s = py::mul(&v_t.clone(), &v_c.clone())?;
                for t6309 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t6309;
                    let t6310 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t6310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_akp = t6310[0].clone();
                    v_akq = t6310[1].clone();
                    let t6311 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_akp.clone())?, &py::mul(&v_s.clone(), &v_akq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_akp.clone())?, &py::mul(&v_c.clone(), &v_akq.clone())?)?])).items()?;
                    if t6311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_p.clone(), t6311[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_k.clone())?, &v_q.clone(), t6311[1].clone())?;
                }
                for t6312 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t6312;
                    let t6313 = (Py::tuple(vec![py::getitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone())?, py::getitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone())?])).items()?;
                    if t6313.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_apk = t6313[0].clone();
                    v_aqk = t6313[1].clone();
                    let t6314 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_apk.clone())?, &py::mul(&v_s.clone(), &v_aqk.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_apk.clone())?, &py::mul(&v_c.clone(), &v_aqk.clone())?)?])).items()?;
                    if t6314.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_a.clone(), &v_p.clone())?, &v_k.clone(), t6314[0].clone())?;
                    py::setitem(&py::getitem(&v_a.clone(), &v_q.clone())?, &v_k.clone(), t6314[1].clone())?;
                }
                for t6315 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_k = t6315;
                    let t6316 = (Py::tuple(vec![py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone())?, py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone())?])).items()?;
                    if t6316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    v_vkp = t6316[0].clone();
                    v_vkq = t6316[1].clone();
                    let t6317 = (Py::tuple(vec![py::sub(&py::mul(&v_c.clone(), &v_vkp.clone())?, &py::mul(&v_s.clone(), &v_vkq.clone())?)?, py::add(&py::mul(&v_s.clone(), &v_vkp.clone())?, &py::mul(&v_c.clone(), &v_vkq.clone())?)?])).items()?;
                    if t6317.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_p.clone(), t6317[0].clone())?;
                    py::setitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_q.clone(), t6317[1].clone())?;
                }
            }
        }
    }
    if !t6296 {
        return Err(py::value_error(py::to_str(&Py::str("the Jacobi iteration did not converge"))));
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t6318 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6318;
out.push(py::getitem(&py::getitem(&v_a.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, v_V.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__legendre_nodes(mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_dx: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_p0: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let mut v_x: Py = Py::None;
    let c_xs = py::cell_new();
    let t1528 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t1528.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    py::cell_set(&c_xs, t1528[0].clone());
    v_ws = t1528[1].clone();
    for t1529 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_i = t1529;
        v_x = py::math1("cos", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &py::sub(&v_i.clone(), &Py::Float(0.25))?)?, &py::add(&v_n.clone(), &Py::Float(0.5))?)?)?;
        for t1530 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
            v__ = t1530;
            let t1531 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
            if t1531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p0 = t1531[0].clone();
            v_p1 = t1531[1].clone();
            for t1532 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                v_k = t1532;
                let t1533 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
                if t1533.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p0 = t1533[0].clone();
                v_p1 = t1533[1].clone();
            }
            v_dp = (if ({ let t1534 = v_n.clone(); Py::Bool({ let t1535 = Py::Int(1i64); if py::gt(&t1534, &t1535)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { Py::Float(1.0) });
            if ({ let t1536 = v_n.clone(); Py::Bool({ let t1537 = Py::Int(1i64); if py::eq(&t1536, &t1537) { true } else { false } }) }).truthy() {
                let t1538 = (Py::tuple(vec![v_x.clone(), Py::Float(1.0)])).items()?;
                if t1538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_p1 = t1538[0].clone();
                v_dp = t1538[1].clone();
            }
            v_dx = py::truediv(&v_p1.clone(), &v_dp.clone())?;
            v_x = py::sub(&v_x, &v_dx.clone())?;
            if ({ let t1539 = py::abs(&v_dx.clone())?; Py::Bool({ let t1540 = Py::Float(1e-16); if py::le(&t1539, &t1540)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        let t1541 = (Py::tuple(vec![Py::Float(1.0), v_x.clone()])).items()?;
        if t1541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p0 = t1541[0].clone();
        v_p1 = t1541[1].clone();
        for t1542 in py::range_iter(&[(Py::Int(2i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t1542;
            let t1543 = (Py::tuple(vec![v_p1.clone(), py::truediv(&py::sub(&py::mul(&py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &v_x.clone())?, &v_p1.clone())?, &py::mul(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_p0.clone())?)?, &v_k.clone())?])).items()?;
            if t1543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_p0 = t1543[0].clone();
            v_p1 = t1543[1].clone();
        }
        v_dp = (if ({ let t1544 = v_n.clone(); Py::Bool({ let t1545 = Py::Int(1i64); if py::gt(&t1544, &t1545)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&v_n.clone(), &py::sub(&py::mul(&v_x.clone(), &v_p1.clone())?, &v_p0.clone())?)?, &py::sub(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)? } else { Py::Float(1.0) });
        let _ = py::method(&py::cell_get(&c_xs)?, "append", &[v_x.clone()])?;
        let _ = py::method(&v_ws.clone(), "append", &[py::truediv(&Py::Int(2i64), &py::mul(&py::mul(&py::sub(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?, &v_dp.clone())?, &v_dp.clone())?)?])?;
    }
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v__ = v__.clone(); let v_dp = v_dp.clone(); let v_dx = v_dx.clone(); let v_i = v_i.clone(); let v_n = v_n.clone(); let v_p0 = v_p0.clone(); let v_p1 = v_p1.clone(); let v_ws = v_ws.clone(); let v_x = v_x.clone(); let c_xs = c_xs.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&py::cell_get(&c_xs)?, &v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    let t1548 = (Py::tuple(vec![{ let mut out = vec![]; for t1546 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t1546;
out.push(py::getitem(&py::cell_get(&c_xs)?, &v_k.clone())?); } Py::list(out) }, { let mut out = vec![]; for t1547 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t1547;
out.push(py::getitem(&v_ws.clone(), &v_k.clone())?); } Py::list(out) }])).items()?;
    if t1548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    py::cell_set(&c_xs, t1548[0].clone());
    v_ws = t1548[1].clone();
    for t1549 in py::range_iter(&[(py::floordiv(&v_n.clone(), &Py::Int(2i64))?).i()?]) {
        v_k = t1549;
        v_m = py::truediv(&py::sub(&py::getitem(&py::cell_get(&c_xs)?, &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?, &py::getitem(&py::cell_get(&c_xs)?, &v_k.clone())?)?, &Py::Int(2i64))?;
        v_w = py::truediv(&py::add(&py::getitem(&v_ws.clone(), &v_k.clone())?, &py::getitem(&v_ws.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?)?)?, &Py::Int(2i64))?;
        let t1550 = (Py::tuple(vec![py::neg(&v_m.clone())?, v_m.clone(), v_w.clone(), v_w.clone()])).items()?;
        if t1550.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&py::cell_get(&c_xs)?, &v_k.clone(), t1550[0].clone())?;
        py::setitem(&py::cell_get(&c_xs)?, &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?, t1550[1].clone())?;
        py::setitem(&v_ws.clone(), &v_k.clone(), t1550[2].clone())?;
        py::setitem(&v_ws.clone(), &py::sub(&py::sub(&v_n.clone(), &Py::Int(1i64))?, &v_k.clone())?, t1550[3].clone())?;
    }
    if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() {
        py::setitem(&py::cell_get(&c_xs)?, &py::floordiv(&v_n.clone(), &Py::Int(2i64))?, Py::Float(0.0))?;
    }
    return Ok(Py::tuple(vec![py::cell_get(&c_xs)?, v_ws.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lu_solver(mut v_M: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_perm: Py = Py::None;
    let mut v_solve: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_U = { let mut out = vec![]; for t5023 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t5023;
out.push({ let mut out = vec![]; for t5024 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t5024;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_perm = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    v_L = { let mut out = vec![]; for t5025 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t5025;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t5026 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t5026;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_L = v_L.clone(); let v_M = v_M.clone(); let v_U = v_U.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_perm = v_perm.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t5027 = py::getitem(&py::getitem(&v_U.clone(), &v_p.clone())?, &v_k.clone())?; Py::Bool({ let t5028 = Py::Int(0i64); if py::eq(&t5027, &t5028) { true } else { false } }) }).truthy() {
            py::setitem(&py::getitem(&v_U.clone(), &v_p.clone())?, &v_k.clone(), py::add(&Py::Float(1e-300), &py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), f__fro(v_M.clone())?], true)?)?)?)?;
        }
        let t5029 = (Py::tuple(vec![py::getitem(&v_U.clone(), &v_p.clone())?, py::getitem(&v_U.clone(), &v_k.clone())?, py::getitem(&v_L.clone(), &v_p.clone())?, py::getitem(&v_L.clone(), &v_k.clone())?, py::getitem(&v_perm.clone(), &v_p.clone())?, py::getitem(&v_perm.clone(), &v_k.clone())?])).items()?;
        if t5029.len() != 6 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_U.clone(), &v_k.clone(), t5029[0].clone())?;
        py::setitem(&v_U.clone(), &v_p.clone(), t5029[1].clone())?;
        py::setitem(&v_L.clone(), &v_k.clone(), t5029[2].clone())?;
        py::setitem(&v_L.clone(), &v_p.clone(), t5029[3].clone())?;
        py::setitem(&v_perm.clone(), &v_k.clone(), t5029[4].clone())?;
        py::setitem(&v_perm.clone(), &v_p.clone(), t5029[5].clone())?;
        for t5030 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t5030;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_U.clone(), &v_k.clone())?, &v_k.clone())?)?;
            py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone(), v_f.clone())?;
            py::setitem(&v_U.clone(), &v_i.clone(), { let mut out = vec![]; for t5031 in py::iter(&(py::zip(&[py::getitem(&v_U.clone(), &v_i.clone())?, py::getitem(&v_U.clone(), &v_k.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5032 = (t5031).items()?;
if t5032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5032[0].clone();
v_b = t5032[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
        }
    }
    v_solve = { let v_L = v_L.clone(); let v_M = v_M.clone(); let v_U = v_U.clone(); let v_f = v_f.clone(); let v_i = v_i.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_p = v_p.clone(); let v_perm = v_perm.clone(); let v_solve = v_solve.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = Py::None;
            let mut v_z: Py = Py::None;
            v_z = { let mut out = vec![]; for t5033 in py::iter(&(v_perm.clone()))? { let mut v_p: Py; 
v_p = t5033;
out.push(py::getitem(&v_y.clone(), &v_p.clone())?); } Py::list(out) };
            for t5034 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_i = t5034;
                let t5035 = v_z.clone(); let t5036 = v_i.clone();
                py::setitem(&t5035, &t5036, py::sub(&py::getitem(&t5035, &t5036)?, &py::fsum(&{ let mut out = vec![]; for t5037 in py::range_iter(&[(v_i.clone()).i()?]) { let mut v_k: Py; 
v_k = t5037;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_z.clone(), &v_k.clone())?)?); } Py::list(out) })?)?)?;
            }
            for t5038 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
                v_i = t5038;
                py::setitem(&v_z.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t5039 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5039;
out.push(py::mul(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_z.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
            }
            return Ok(v_z.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(v_solve.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__minor_F(mut v_FA: Py, mut v_i: Py, mut v_j: Py) -> R {
    return Ok(f__det_F({ let mut out = vec![]; for t4048 in py::iter(&(py::enumerate(&v_FA.clone(), 0)?))? { let mut v_k: Py; let mut v_r: Py; 
let t4049 = (t4048).items()?;
if t4049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t4049[0].clone();
v_r = t4049[1].clone();
if ({ let t4050 = v_k.clone(); Py::Bool({ let t4051 = v_i.clone(); if py::ne(&t4050, &t4051) { true } else { false } }) }).truthy() { out.push({ let mut out = vec![]; for t4052 in py::iter(&(py::enumerate(&v_r.clone(), 0)?))? { let mut v_c: Py; let mut v_x: Py; 
let t4053 = (t4052).items()?;
if t4053.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t4053[0].clone();
v_x = t4053[1].clone();
if ({ let t4054 = v_c.clone(); Py::Bool({ let t4055 = v_j.clone(); if py::ne(&t4054, &t4055) { true } else { false } }) }).truthy() { out.push(v_x.clone()); }} Py::list(out) }); }} Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_Bt: Py = Py::None;
    v_Bt = Py::list((py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_B.clone()).items()?); v })?).items()?);
    return Ok({ let mut out = vec![]; for t6358 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6358;
out.push({ let mut out = vec![]; for t6359 in py::iter(&(v_Bt.clone()))? { let mut v_c: Py; 
v_c = t6359;
out.push(py::fsum(&{ let mut out = vec![]; for t6360 in py::iter(&(py::zip(&[v_r.clone(), v_c.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6361 = (t6360).items()?;
if t6361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6361[0].clone();
v_b = t6361[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mm_F(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_Bt: Py = Py::None;
    v_Bt = Py::list((py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_B.clone()).items()?); v })?).items()?);
    return Ok({ let mut out = vec![]; for t6375 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6375;
out.push({ let mut out = vec![]; for t6376 in py::iter(&(v_Bt.clone()))? { let mut v_c: Py; 
v_c = t6376;
out.push(py::sum(&{ let mut out = vec![]; for t6377 in py::iter(&(py::zip(&[v_r.clone(), v_c.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6378 = (t6377).items()?;
if t6378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6378[0].clone();
v_b = t6378[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mv(mut v_A: Py, mut v_x: Py) -> R {
    return Ok({ let mut out = vec![]; for t6481 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6481;
out.push(py::fsum(&{ let mut out = vec![]; for t6482 in py::iter(&(py::zip(&[v_r.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6483 = (t6482).items()?;
if t6483.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6483[0].clone();
v_b = t6483[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) })?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need_mat(mut v_A: Py, mut v_name: Py, mut v_square: Py, mut v_rows: Py, mut v_cols: Py, mut v_max_dim: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    if ({ let t6256 = py::not(&Py::Bool(matches!(v_A.clone(), Py::List(_) | Py::Tuple(_)))); if t6256.truthy() { t6256 } else { { let t6255 = py::not(&v_A.clone()); if t6255.truthy() { t6255 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t6253 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6253;
out.push({ let t6254 = Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_))); if !t6254.truthy() { t6254 } else { v_r.clone() } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix (a list of rows)".to_string()].concat()))));
    }
    v_w = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?;
    if (Py::Bool(({ let mut out = vec![]; for t6257 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6257;
out.push({ let t6258 = py::len(&v_r.clone())?; Py::Bool({ let t6259 = v_w.clone(); if py::ne(&t6258, &t6259) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular (rows of equal length)".to_string()].concat()))));
    }
    for t6260 in py::iter(&(v_A.clone()))? {
        v_r = t6260;
        for t6261 in py::iter(&(v_r.clone()))? {
            v_x = t6261;
            let _ = f__finite(v_x.clone(), v_name.clone())?;
        }
    }
    if ({ let t6266 = v_square.clone(); if !t6266.truthy() { t6266 } else { { let t6264 = py::len(&v_A.clone())?; Py::Bool({ let t6265 = v_w.clone(); if py::ne(&t6264, &t6265) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be square".to_string()].concat()))));
    }
    if ({ let t6271 = { let t6267 = v_rows.clone(); Py::Bool({ let t6268 = Py::None; if !py::is(&t6267, &t6268) { true } else { false } }) }; if !t6271.truthy() { t6271 } else { { let t6269 = py::len(&v_A.clone())?; Py::Bool({ let t6270 = v_rows.clone(); if py::ne(&t6269, &t6270) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_rows.clone(), "")?, " rows".to_string()].concat()))));
    }
    if ({ let t6276 = { let t6272 = v_cols.clone(); Py::Bool({ let t6273 = Py::None; if !py::is(&t6272, &t6273) { true } else { false } }) }; if !t6276.truthy() { t6276 } else { { let t6274 = v_w.clone(); Py::Bool({ let t6275 = v_cols.clone(); if py::ne(&t6274, &t6275) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have ".to_string(), py::format(&v_cols.clone(), "")?, " columns".to_string()].concat()))));
    }
    if ({ let t6282 = v_max_dim.clone(); if !t6282.truthy() { t6282 } else { { let t6281 = { let t6277 = py::len(&v_A.clone())?; Py::Bool({ let t6278 = v_max_dim.clone(); if py::gt(&t6277, &t6278)? { true } else { false } }) }; if t6281.truthy() { t6281 } else { { let t6279 = v_w.clone(); Py::Bool({ let t6280 = v_max_dim.clone(); if py::gt(&t6279, &t6280)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " may have at most ".to_string(), py::format(&v_max_dim.clone(), "")?, " rows and columns".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t6283 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t6283;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need_poly(mut v_c: Py, mut v_name: Py) -> R {
    v_c = f__need_vec(v_c.clone(), v_name.clone(), Py::None, Py::Int(1i64), Py::Int(13i64))?;
    while ({ let t5601 = { let t5597 = py::len(&v_c.clone())?; Py::Bool({ let t5598 = Py::Int(1i64); if py::gt(&t5597, &t5598)? { true } else { false } }) }; if !t5601.truthy() { t5601 } else { { let t5599 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t5600 = Py::Int(0i64); if py::eq(&t5599, &t5600) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need_symmetric(mut v_A: Py, mut v_name: Py) -> R {
    v_A = f__need_mat(v_A.clone(), v_name.clone(), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    if (Py::Bool(({ let mut out = vec![]; for t6284 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6284;
for t6285 in py::range_iter(&[(v_i.clone()).i()?]) { let mut v_j: Py; 
v_j = t6285;
out.push({ let t6286 = f__F(py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, Py::str("input"))?; Py::Bool({ let t6287 = f__F(py::getitem(&py::getitem(&v_A.clone(), &v_j.clone())?, &v_i.clone())?, Py::str("input"))?; if py::ne(&t6286, &t6287) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))));
    }
    return Ok(v_A.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need_vec(mut v_v: Py, mut v_name: Py, mut v_n: Py, mut v_min_n: Py, mut v_max_n: Py) -> R {
    let mut v_x: Py = Py::None;
    if ({ let t6685 = py::not(&Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))); if t6685.truthy() { t6685 } else { Py::Bool(({ let mut out = vec![]; for t6684 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6684;
out.push(Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))));
    }
    for t6686 in py::iter(&(v_v.clone()))? {
        v_x = t6686;
        let _ = f__finite(v_x.clone(), v_name.clone())?;
    }
    if ({ let t6689 = py::len(&v_v.clone())?; Py::Bool({ let t6690 = v_min_n.clone(); if py::lt(&t6689, &t6690)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_min_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    if ({ let t6695 = { let t6691 = v_n.clone(); Py::Bool({ let t6692 = Py::None; if !py::is(&t6691, &t6692) { true } else { false } }) }; if !t6695.truthy() { t6695 } else { { let t6693 = py::len(&v_v.clone())?; Py::Bool({ let t6694 = v_n.clone(); if py::ne(&t6693, &t6694) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must have exactly ".to_string(), py::format(&v_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    if ({ let t6700 = { let t6696 = v_max_n.clone(); Py::Bool({ let t6697 = Py::None; if !py::is(&t6696, &t6697) { true } else { false } }) }; if !t6700.truthy() { t6700 } else { { let t6698 = py::len(&v_v.clone())?; Py::Bool({ let t6699 = v_max_n.clone(); if py::gt(&t6698, &t6699)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " allows at most ".to_string(), py::format(&v_max_n.clone(), "")?, " entries".to_string()].concat()))));
    }
    return Ok(Py::list((v_v.clone()).items()?));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__neville(mut v_x: Py, mut v_y: Py, mut v_t: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t2393 = (Py::tuple(vec![f__FV(v_x.clone())?, f__FV(v_y.clone())?])).items()?;
    if t2393.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2393[0].clone();
    v_Y = t2393[1].clone();
    v_P = Py::list((v_Y.clone()).items()?);
    v_n = py::len(&v_X.clone())?;
    for t2394 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_k = t2394;
        for t2395 in py::range_iter(&[(py::sub(&v_n.clone(), &v_k.clone())?).i()?]) {
            v_i = t2395;
            py::setitem(&v_P.clone(), &v_i.clone(), py::truediv(&py::add(&py::mul(&py::sub(&v_t.clone(), &py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &v_k.clone())?)?)?, &py::getitem(&v_P.clone(), &v_i.clone())?)?, &py::mul(&py::sub(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_t.clone())?, &py::getitem(&v_P.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::sub(&py::getitem(&v_X.clone(), &v_i.clone())?, &py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &v_k.clone())?)?)?)?)?;
        }
    }
    return Ok(py::float(&py::getitem(&v_P.clone(), &Py::Int(0i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__newton_like(mut v_c: Py, mut v_step: Py, mut v_x: Py, mut v_x_prev: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut t5612 = false;
    for t5613 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t5613;
        v_nx = py::call(&v_step.clone(), &[v_x.clone(), v_x_prev.clone()])?;
        if ({ let t5614 = v_nx.clone(); Py::Bool({ let t5615 = Py::None; if py::is(&t5614, &t5615) { true } else { false } }) }).truthy() {
            { t5612 = true; break; }
        }
        if (py::not(&py::math1("isfinite", &v_nx.clone())?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the iteration diverged"))));
        }
        v_done = { let t5620 = { let t5616 = py::abs(&py::sub(&v_nx.clone(), &v_x.clone())?)?; Py::Bool({ let t5617 = py::mul(&Py::Float(4.4e-16), &py::minmax(&[py::abs(&v_nx.clone())?, Py::Float(1e-300)], true)?)?; if py::le(&t5616, &t5617)? { true } else { false } }) }; if t5620.truthy() { t5620 } else { { let t5618 = f__peval(v_c.clone(), v_nx.clone())?; Py::Bool({ let t5619 = Py::Int(0i64); if py::eq(&t5618, &t5619) { true } else { false } }) } } };
        let t5621 = (Py::tuple(vec![v_x.clone(), v_nx.clone()])).items()?;
        if t5621.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x_prev = t5621[0].clone();
        v_x = t5621[1].clone();
        if (v_done.clone()).truthy() {
            { t5612 = true; break; }
        }
    }
    if !t5612 {
        return Err(py::value_error(py::to_str(&Py::str("the iteration did not converge in 200 steps"))));
    }
    for t5622 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t5622;
        v_d = f__peval(f__pder(v_c.clone())?, v_x.clone())?;
        if ({ let t5626 = v_d.clone(); Py::Bool({ let t5627 = Py::Int(0i64); if py::eq(&t5626, &t5627) { true } else { false } }) }).truthy() {
            break;
        }
        v_nx = py::sub(&v_x.clone(), &py::truediv(&f__peval(v_c.clone(), v_x.clone())?, &v_d.clone())?)?;
        if ({ let t5630 = py::not(&py::math1("isfinite", &v_nx.clone())?); if t5630.truthy() { t5630 } else { { let t5628 = py::abs(&py::sub(&v_nx.clone(), &v_x.clone())?)?; Py::Bool({ let t5629 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?; if py::gt(&t5628, &t5629)? { true } else { false } }) } } }).truthy() {
            break;
        }
        v_x = v_nx.clone();
    }
    if ({ let t5637 = { let t5631 = f__peval(v_c.clone(), v_x.clone())?; Py::Bool({ let t5632 = Py::Int(0i64); if py::ne(&t5631, &t5632) { true } else { false } }) }; if !t5637.truthy() { t5637 } else { py::not(&f__well_conditioned_root(v_c.clone(), v_x.clone())?) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the iteration did not reach a simple, well-conditioned root"))));
    }
    if ({ let t5638 = f__root_residual(Py::dict(vec![(Py::str("coefficients"), v_c.clone())]), Py::dict(vec![(Py::str("root"), v_x.clone())]), Py::None)?; Py::Bool({ let t5639 = Py::Float(0.0); if py::ne(&t5638, &t5639) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the iteration did not converge to a root"))));
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__norm(mut v_v: Py) -> R {
    return Ok(py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t6674 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6674;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) })?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__normal_solve(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_At: Py = Py::None;
    v_At = f__T(v_A.clone())?;
    let t2613: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(f__solve_F(f__mm_F(v_At.clone(), v_A.clone())?, { let mut out = vec![]; for t2610 in py::iter(&(v_At.clone()))? { let mut v_r: Py; 
v_r = t2610;
out.push(py::sum(&{ let mut out = vec![]; for t2611 in py::iter(&(py::zip(&[v_r.clone(), v_b.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t2612 = (t2611).items()?;
if t2612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2612[0].clone();
v_c = t2612[1].clone();
out.push(py::mul(&v_a.clone(), &v_c.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) })?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2613 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Err(py::value_error(py::to_str(&Py::str("the columns are linearly dependent: the least-squares solution is not unique"))));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pder(mut v_c: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    return Ok({ let t5625 = { let mut out = vec![]; for t5623 in py::iter(&(py::enumerate(&py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_a: Py; let mut v_k: Py; 
let t5624 = (t5623).items()?;
if t5624.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5624[0].clone();
v_a = t5624[1].clone();
out.push(py::mul(&v_a.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?); } Py::list(out) }; if t5625.truthy() { t5625 } else { Py::list(vec![Py::Float(0.0)]) } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__peval(mut v_c: Py, mut v_x: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t5606 in py::iter(&(v_c.clone()))? {
        v_a = t5606;
        v_v = py::add(&py::mul(&v_v.clone(), &v_x.clone())?, &v_a.clone())?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__power_sums_exact(mut v_A: Py, mut v_kmax: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_F = f__FM(v_A.clone())?;
    let t6372 = (Py::tuple(vec![{ let mut out = vec![]; for t6371 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t6371;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) }, Py::list(vec![])])).items()?;
    if t6372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t6372[0].clone();
    v_out = t6372[1].clone();
    for t6373 in py::range_iter(&[(v_kmax.clone()).i()?]) {
        v__ = t6373;
        let _ = py::method(&v_out.clone(), "append", &[py::float(&py::sum(&{ let mut out = vec![]; for t6374 in py::range_iter(&[(py::len(&v_F.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6374;
out.push(py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?])?;
        v_P = f__mm_F(v_P.clone(), v_F.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pscale(mut v_c: Py, mut v_x: Py) -> R {
    return Ok(py::fsum(&{ let mut out = vec![]; for t5635 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_a: Py; let mut v_k: Py; 
let t5636 = (t5635).items()?;
if t5636.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t5636[0].clone();
v_a = t5636[1].clone();
out.push(py::mul(&py::abs(&v_a.clone())?, &py::pow(&py::abs(&v_x.clone())?, &py::sub(&py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?)?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qr_house(mut v_A: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_nx: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t5323 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t5323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5323[0].clone();
    v_n = t5323[1].clone();
    v_R = { let mut out = vec![]; for t5324 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t5324;
out.push({ let mut out = vec![]; for t5325 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t5325;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_vs = Py::list(vec![]);
    for t5326 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t5326;
        v_x = { let mut out = vec![]; for t5327 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t5327;
out.push(py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_k.clone())?); } Py::list(out) };
        v_nx = f__norm(v_x.clone())?;
        v_alpha = py::neg(&py::copysign(&v_nx.clone(), &py::getitem(&v_x.clone(), &Py::Int(0i64))?)?)?;
        v_v = py::slice(&v_x.clone(), None, None, None)?;
        let t5329 = v_v.clone(); let t5330 = Py::Int(0i64);
        py::setitem(&t5329, &t5330, py::sub(&py::getitem(&t5329, &t5330)?, &v_alpha.clone())?)?;
        v_nv = f__norm(v_v.clone())?;
        if ({ let t5331 = v_nv.clone(); Py::Bool({ let t5332 = Py::Int(0i64); if py::eq(&t5331, &t5332) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_vs.clone(), "append", &[Py::None])?;
            continue;
        }
        v_v = { let mut out = vec![]; for t5333 in py::iter(&(v_v.clone()))? { let mut v_t: Py; 
v_t = t5333;
out.push(py::truediv(&v_t.clone(), &v_nv.clone())?); } Py::list(out) };
        let _ = py::method(&v_vs.clone(), "append", &[v_v.clone()])?;
        for t5334 in py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?]) {
            v_j = t5334;
            v_d = py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t5335 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t5335;
out.push(py::mul(&py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?, &py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?)?;
            for t5336 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) {
                v_i = t5336;
                let t5337 = py::getitem(&v_R.clone(), &v_i.clone())?; let t5338 = v_j.clone();
                py::setitem(&t5337, &t5338, py::sub(&py::getitem(&t5337, &t5338)?, &py::mul(&v_d.clone(), &py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?)?)?)?;
            }
        }
    }
    v_Q = { let mut out = vec![]; for t5339 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t5339;
out.push({ let mut out = vec![]; for t5340 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5340;
out.push((if ({ let t5341 = v_i.clone(); Py::Bool({ let t5342 = v_j.clone(); if py::eq(&t5341, &t5342) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    for t5343 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_k = t5343;
        v_v = py::getitem(&v_vs.clone(), &v_k.clone())?;
        if ({ let t5344 = v_v.clone(); Py::Bool({ let t5345 = Py::None; if py::is(&t5344, &t5345) { true } else { false } }) }).truthy() {
            continue;
        }
        for t5346 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t5346;
            v_d = py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t5347 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t5347;
out.push(py::mul(&py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?, &py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?)?;
            for t5348 in py::range_iter(&[(v_k.clone()).i()?, (v_m.clone()).i()?]) {
                v_i = t5348;
                let t5349 = py::getitem(&v_Q.clone(), &v_i.clone())?; let t5350 = v_j.clone();
                py::setitem(&t5349, &t5350, py::sub(&py::getitem(&t5349, &t5350)?, &py::mul(&v_d.clone(), &py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?)?)?)?;
            }
        }
    }
    v_R = { let mut out = vec![]; for t5351 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5351;
out.push({ let mut out = vec![]; for t5352 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5352;
out.push((if ({ let t5353 = v_j.clone(); Py::Bool({ let t5354 = v_i.clone(); if py::ge(&t5353, &t5354)? { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_j.clone())? } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    for t5355 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5355;
        if ({ let t5356 = py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_i.clone())?; Py::Bool({ let t5357 = Py::Int(0i64); if py::lt(&t5356, &t5357)? { true } else { false } }) }).truthy() {
            py::setitem(&v_R.clone(), &v_i.clone(), { let mut out = vec![]; for t5358 in py::iter(&(py::getitem(&v_R.clone(), &v_i.clone())?))? { let mut v_x: Py; 
v_x = t5358;
out.push(py::neg(&v_x.clone())?); } Py::list(out) })?;
            for t5359 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_r = t5359;
                py::setitem(&py::getitem(&v_Q.clone(), &v_r.clone())?, &v_i.clone(), py::neg(&py::getitem(&py::getitem(&v_Q.clone(), &v_r.clone())?, &v_i.clone())?)?)?;
            }
        }
    }
    return Ok(Py::tuple(vec![v_Q.clone(), v_R.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__raise(mut v_msg: Py) -> R {
    return Err(py::value_error(py::to_str(&v_msg.clone())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rank_F(mut v_M: Py) -> R {
    return Ok(py::len(&py::getitem(&f__rref_F(v_M.clone())?, &Py::Int(1i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__real_spectrum(mut v_vals: Py, mut v_fro: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_neg: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_pos: Py = Py::None;
    let mut v_real: Py = Py::None;
    let mut v_tol: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_tol = py::mul(&Py::Float(1e-11), &py::minmax(&[Py::Float(1.0), v_fro.clone()], true)?)?;
    v_real = { let mut out = vec![]; for t4976 in py::iter(&(v_vals.clone()))? { let mut v_z: Py; 
v_z = t4976;
if ({ let t4977 = py::abs(&py::imag(&py::complex(&[v_z.clone()])?)?)?; Py::Bool({ let t4978 = v_tol.clone(); if py::le(&t4977, &t4978)? { true } else { false } }) }).truthy() { out.push(py::real(&py::complex(&[v_z.clone()])?)?); }} Py::list(out) };
    v_pos = py::sorted_key(&{ let mut out = vec![]; for t4979 in py::iter(&(v_vals.clone()))? { let mut v_z: Py; 
v_z = t4979;
if ({ let t4980 = py::imag(&py::complex(&[v_z.clone()])?)?; Py::Bool({ let t4981 = v_tol.clone(); if py::gt(&t4980, &t4981)? { true } else { false } }) }).truthy() { out.push(py::complex(&[v_z.clone()])?); }} Py::list(out) }, &{ let v_fro = v_fro.clone(); let v_real = v_real.clone(); let v_tol = v_tol.clone(); let v_vals = v_vals.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_z.clone())?, py::imag(&v_z.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_neg = { let mut out = vec![]; for t4982 in py::iter(&(v_vals.clone()))? { let mut v_z: Py; 
v_z = t4982;
if ({ let t4983 = py::imag(&py::complex(&[v_z.clone()])?)?; Py::Bool({ let t4984 = py::neg(&v_tol.clone())?; if py::lt(&t4983, &t4984)? { true } else { false } }) }).truthy() { out.push(py::complex(&[v_z.clone()])?); }} Py::list(out) };
    if ({ let t4985 = py::len(&v_pos.clone())?; Py::Bool({ let t4986 = py::len(&v_neg.clone())?; if py::ne(&t4985, &t4986) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the eigenvalues could not be separated into conjugate pairs"))));
    }
    v_out = py::sorted(&v_real.clone(), (Py::Bool(false)).truthy())?;
    v_pairs = Py::list(vec![]);
    for t4987 in py::iter(&(v_pos.clone()))? {
        v_z = t4987;
        v_w = py::minmax_key(&[v_neg.clone()], &{ let v_fro = v_fro.clone(); let v_neg = v_neg.clone(); let v_out = v_out.clone(); let v_pairs = v_pairs.clone(); let v_pos = v_pos.clone(); let v_real = v_real.clone(); let v_tol = v_tol.clone(); let v_vals = v_vals.clone(); let v_z = v_z.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::method(&v_u.clone(), "conjugate", &[])?, &v_z.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
        let _ = py::method(&v_neg.clone(), "remove", &[v_w.clone()])?;
        v_m = py::truediv(&py::add(&v_z.clone(), &py::method(&v_w.clone(), "conjugate", &[])?)?, &Py::Int(2i64))?;
        v_pairs = py::iadd(&v_pairs, &Py::list(vec![v_m.clone(), py::method(&v_m.clone(), "conjugate", &[])?]))?;
    }
    v_pairs = py::sorted_key(&v_pairs.clone(), &{ let v_fro = v_fro.clone(); let v_m = v_m.clone(); let v_neg = v_neg.clone(); let v_out = v_out.clone(); let v_pairs = v_pairs.clone(); let v_pos = v_pos.clone(); let v_real = v_real.clone(); let v_tol = v_tol.clone(); let v_vals = v_vals.clone(); let v_w = v_w.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_z.clone())?, py::neg(&py::imag(&v_z.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(py::add(&v_out.clone(), &v_pairs.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__refine(mut v_A: Py, mut v_lam: Py, mut v_v: Py, mut v_steps: Py) -> R {
    let mut v_Av: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_solve: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_solve = f__lu_solver({ let mut out = vec![]; for t5040 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5040;
out.push({ let mut out = vec![]; for t5041 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5041;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &(if ({ let t5042 = v_i.clone(); Py::Bool({ let t5043 = v_j.clone(); if py::eq(&t5042, &t5043) { true } else { false } }) }).truthy() { v_lam.clone() } else { Py::Float(0.0) }))?); } Py::list(out) }); } Py::list(out) })?;
    for t5044 in py::range_iter(&[(v_steps.clone()).i()?]) {
        v__ = t5044;
        v_v = f__unit(py::call(&v_solve.clone(), &[v_v.clone()])?)?;
    }
    v_Av = f__mv(v_A.clone(), v_v.clone())?;
    return Ok(Py::tuple(vec![f__dot(v_v.clone(), v_Av.clone())?, v_v.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::complex(&[v_a.clone()])?, &py::complex(&[v_b.clone()])?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_a.clone()])?)?, py::abs(&py::complex(&[v_b.clone()])?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__root_residual(mut v_i: Py, mut v_o: Py, mut v_interval: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_delta: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t5641 = (Py::tuple(vec![{ let mut out = vec![]; for t5640 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_t: Py; 
v_t = t5640;
out.push(py::float(&v_t.clone())?); } Py::list(out) }, py::getitem(&v_o.clone(), &Py::str("root"))?])).items()?;
    if t5641.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5641[0].clone();
    v_x = t5641[1].clone();
    while ({ let t5646 = { let t5642 = py::len(&v_c.clone())?; Py::Bool({ let t5643 = Py::Int(1i64); if py::gt(&t5642, &t5643)? { true } else { false } }) }; if !t5646.truthy() { t5646 } else { { let t5644 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t5645 = Py::Int(0i64); if py::eq(&t5644, &t5645) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    if ({ let t5650 = v_interval.clone(); if !t5650.truthy() { t5650 } else { py::not(&{ let t5647 = py::getitem(&v_interval.clone(), &Py::Int(0i64))?; Py::Bool({ let t5648 = v_x.clone(); if py::le(&t5647, &t5648)? { { let t5649 = py::getitem(&v_interval.clone(), &Py::Int(1i64))?; if py::le(&t5648, &t5649)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t5651 = f__peval(v_c.clone(), v_x.clone())?; Py::Bool({ let t5652 = Py::Int(0i64); if py::eq(&t5651, &t5652) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_delta = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?;
    let t5653 = (Py::tuple(vec![f__peval(v_c.clone(), py::sub(&v_x.clone(), &v_delta.clone())?)?, f__peval(v_c.clone(), py::add(&v_x.clone(), &v_delta.clone())?)?])).items()?;
    if t5653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5653[0].clone();
    v_hi = t5653[1].clone();
    return Ok((if ({ let t5654 = py::mul(&v_lo.clone(), &v_hi.clone())?; Py::Bool({ let t5655 = Py::Int(0i64); if py::le(&t5654, &t5655)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rref_F(mut v_M: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_M = { let mut out = vec![]; for t6044 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t6044;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) };
    let t6045 = (Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?])).items()?;
    if t6045.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t6045[0].clone();
    v_cols = t6045[1].clone();
    let t6046 = (Py::tuple(vec![Py::list(vec![]), Py::Int(0i64)])).items()?;
    if t6046.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_piv = t6046[0].clone();
    v_r = t6046[1].clone();
    for t6047 in py::range_iter(&[(v_cols.clone()).i()?]) {
        v_c = t6047;
        if ({ let t6048 = v_r.clone(); Py::Bool({ let t6049 = v_rows.clone(); if py::eq(&t6048, &t6049) { true } else { false } }) }).truthy() {
            break;
        }
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_r.clone()).i()?, (v_rows.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_M = v_M.clone(); let v_c = v_c.clone(); let v_cols = v_cols.clone(); let v_piv = v_piv.clone(); let v_r = v_r.clone(); let v_rows = v_rows.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t6050 = py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?; Py::Bool({ let t6051 = Py::Int(0i64); if py::eq(&t6050, &t6051) { true } else { false } }) }).truthy() {
            continue;
        }
        let t6052 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_r.clone())?])).items()?;
        if t6052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_r.clone(), t6052[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t6052[1].clone())?;
        v_pv = py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?;
        py::setitem(&v_M.clone(), &v_r.clone(), { let mut out = vec![]; for t6053 in py::iter(&(py::getitem(&v_M.clone(), &v_r.clone())?))? { let mut v_x: Py; 
v_x = t6053;
out.push(py::truediv(&v_x.clone(), &v_pv.clone())?); } Py::list(out) })?;
        for t6054 in py::range_iter(&[(v_rows.clone()).i()?]) {
            v_i = t6054;
            if ({ let t6059 = { let t6055 = v_i.clone(); Py::Bool({ let t6056 = v_r.clone(); if py::ne(&t6055, &t6056) { true } else { false } }) }; if !t6059.truthy() { t6059 } else { { let t6057 = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_c.clone())?; Py::Bool({ let t6058 = Py::Int(0i64); if py::ne(&t6057, &t6058) { true } else { false } }) } } }).truthy() {
                v_f = py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_c.clone())?;
                py::setitem(&v_M.clone(), &v_i.clone(), { let mut out = vec![]; for t6060 in py::iter(&(py::zip(&[py::getitem(&v_M.clone(), &v_i.clone())?, py::getitem(&v_M.clone(), &v_r.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6061 = (t6060).items()?;
if t6061.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6061[0].clone();
v_b = t6061[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)?); } Py::list(out) })?;
            }
        }
        let _ = py::method(&v_piv.clone(), "append", &[v_c.clone()])?;
        v_r = py::iadd(&v_r, &Py::Int(1i64))?;
    }
    return Ok(Py::tuple(vec![v_M.clone(), v_piv.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sdd(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    v_F = f__FM(v_A.clone())?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t2248 in py::range_iter(&[(py::len(&v_F.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2248;
out.push({ let t2249 = py::abs(&py::getitem(&py::getitem(&v_F.clone(), &v_i.clone())?, &v_i.clone())?)?; Py::Bool({ let t2250 = py::sum(&{ let mut out = vec![]; for t2251 in py::iter(&(py::enumerate(&py::getitem(&v_F.clone(), &v_i.clone())?, 0)?))? { let mut v_j: Py; let mut v_x: Py; 
let t2252 = (t2251).items()?;
if t2252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t2252[0].clone();
v_x = t2252[1].clone();
if ({ let t2253 = v_j.clone(); Py::Bool({ let t2254 = v_i.clone(); if py::ne(&t2253, &t2254) { true } else { false } }) }).truthy() { out.push(py::abs(&v_x.clone())?); }} Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?; if py::gt(&t2249, &t2250)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__shape(mut v_M: Py) -> R {
    return Ok((if ({ let t6342 = v_M.clone(); if !t6342.truthy() { t6342 } else { Py::Bool(matches!(py::getitem(&v_M.clone(), &Py::Int(0i64))?, Py::List(_) | Py::Tuple(_))) } }).truthy() { Py::tuple(vec![py::len(&v_M.clone())?, py::len(&py::getitem(&v_M.clone(), &Py::Int(0i64))?)?]) } else { Py::tuple(vec![py::len(&v_M.clone())?]) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_fix(mut v_v: Py, mut v_tol: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_x: Py = Py::None;
    if (py::not(&v_v.clone())).truthy() {
        return Ok(v_v.clone());
    }
    v_big = py::minmax(&[{ let mut out = vec![]; for t6328 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6328;
out.push(py::abs(&py::complex(&[v_x.clone()])?)?); } Py::list(out) }], true)?;
    for t6329 in py::iter(&(v_v.clone()))? {
        v_x = t6329;
        if ({ let t6330 = py::abs(&py::complex(&[v_x.clone()])?)?; Py::Bool({ let t6331 = py::sub(&v_big.clone(), &py::mul(&v_tol.clone(), &py::minmax(&[Py::Float(1.0), v_big.clone()], true)?)?)?; if py::ge(&t6330, &t6331)? { true } else { false } }) }).truthy() {
            return Ok((if ({ let t6332 = py::real(&py::complex(&[v_x.clone()])?)?; Py::Bool({ let t6333 = Py::Int(0i64); if py::lt(&t6332, &t6333)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t6334 in py::iter(&(v_v.clone()))? { let mut v_y: Py; 
v_y = t6334;
out.push(py::neg(&v_y.clone())?); } Py::list(out) } } else { Py::list((v_v.clone()).items()?) }));
        }
    }
    return Ok(Py::list((v_v.clone()).items()?));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sign_ok(mut v_v: Py, mut v_tol: Py) -> R {
    return Ok({ let t6346 = f__sign_fix(v_v.clone(), v_tol.clone())?; Py::Bool({ let t6347 = Py::list((v_v.clone()).items()?); if py::eq(&t6346, &t6347) { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_F(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_aug: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_piv: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_aug = { let mut out = vec![]; for t6042 in py::iter(&(py::zip(&[v_A.clone(), v_b.clone()])?))? { let mut v_bi: Py; let mut v_r: Py; 
let t6043 = (t6042).items()?;
if t6043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t6043[0].clone();
v_bi = t6043[1].clone();
out.push(py::add(&Py::list((v_r.clone()).items()?), &Py::list(vec![v_bi.clone()]))?); } Py::list(out) };
    let t6062 = (f__rref_F(v_aug.clone())?).items()?;
    if t6062.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t6062[0].clone();
    v_piv = t6062[1].clone();
    if ({ let t6063 = v_piv.clone(); Py::Bool({ let t6064 = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::ne(&t6063, &t6064) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the system has no unique solution (the coefficient matrix is singular)"))));
    }
    return Ok({ let mut out = vec![]; for t6065 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6065;
out.push(py::getitem(&py::getitem(&v_R.clone(), &v_i.clone())?, &v_n.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__spd(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_F = f__FM(v_A.clone())?;
    v_n = py::len(&v_F.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1623 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1623;
for t1624 in py::range_iter(&[(v_i.clone()).i()?]) { let mut v_j: Py; 
v_j = t1624;
out.push({ let t1625 = py::getitem(&py::getitem(&v_F.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t1626 = py::getitem(&py::getitem(&v_F.clone(), &v_j.clone())?, &v_i.clone())?; if py::ne(&t1625, &t1626) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(Py::Bool(false));
    }
    return Ok(Py::Bool(({ let mut out = vec![]; for t1627 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1627;
out.push({ let t1628 = f__det_F({ let mut out = vec![]; for t1642 in py::iter(&(py::slice(&v_F.clone(), None, Some((v_k.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t1642;
out.push(py::slice(&v_r.clone(), None, Some((v_k.clone()).i()?), None)?); } Py::list(out) })?; Py::Bool({ let t1629 = Py::Int(0i64); if py::gt(&t1628, &t1629)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__spline_value(mut v_X: Py, mut v_Y: Py, mut v_t: Py, mut v_s0: Py, mut v_sn: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::sub(&py::len(&v_X.clone())?, &Py::Int(1i64))?;
    v_h = { let mut out = vec![]; for t4116 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4116;
out.push(py::sub(&py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_X.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_M = { let mut out = vec![]; for t4117 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t4117;
out.push(py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    v_r = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    if ({ let t4118 = v_s0.clone(); Py::Bool({ let t4119 = Py::None; if py::is(&t4118, &t4119) { true } else { false } }) }).truthy() {
        let t4120 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, py::fraction(&[Py::Int(1i64)])?])).items()?;
        if t4120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(0i64), t4120[0].clone())?;
        py::setitem(&py::getitem(&v_M.clone(), &v_n.clone())?, &v_n.clone(), t4120[1].clone())?;
    } else {
        let t4121 = (Py::tuple(vec![py::mul(&Py::Int(2i64), &py::getitem(&v_h.clone(), &Py::Int(0i64))?)?, py::getitem(&v_h.clone(), &Py::Int(0i64))?])).items()?;
        if t4121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(0i64), t4121[0].clone())?;
        py::setitem(&py::getitem(&v_M.clone(), &Py::Int(0i64))?, &Py::Int(1i64), t4121[1].clone())?;
        py::setitem(&v_r.clone(), &Py::Int(0i64), py::mul(&Py::Int(6i64), &py::sub(&py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &Py::Int(1i64))?, &py::getitem(&v_Y.clone(), &Py::Int(0i64))?)?, &py::getitem(&v_h.clone(), &Py::Int(0i64))?)?, &v_s0.clone())?)?)?;
        let t4122 = (Py::tuple(vec![py::getitem(&v_h.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, py::mul(&Py::Int(2i64), &py::getitem(&v_h.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?])).items()?;
        if t4122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&py::getitem(&v_M.clone(), &v_n.clone())?, &py::sub(&v_n.clone(), &Py::Int(1i64))?, t4122[0].clone())?;
        py::setitem(&py::getitem(&v_M.clone(), &v_n.clone())?, &v_n.clone(), t4122[1].clone())?;
        py::setitem(&v_r.clone(), &v_n.clone(), py::mul(&Py::Int(6i64), &py::sub(&v_sn.clone(), &py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &v_n.clone())?, &py::getitem(&v_Y.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_h.clone(), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?)?)?;
    }
    for t4123 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_i = t4123;
        let t4124 = (Py::tuple(vec![py::getitem(&v_h.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, py::mul(&Py::Int(2i64), &py::add(&py::getitem(&v_h.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_h.clone(), &v_i.clone())?)?)?, py::getitem(&v_h.clone(), &v_i.clone())?])).items()?;
        if t4124.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &py::sub(&v_i.clone(), &Py::Int(1i64))?, t4124[0].clone())?;
        py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_i.clone(), t4124[1].clone())?;
        py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &py::add(&v_i.clone(), &Py::Int(1i64))?, t4124[2].clone())?;
        py::setitem(&v_r.clone(), &v_i.clone(), py::mul(&Py::Int(6i64), &py::sub(&py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_Y.clone(), &v_i.clone())?)?, &py::getitem(&v_h.clone(), &v_i.clone())?)?, &py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &v_i.clone())?, &py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_h.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?)?)?;
    }
    v_m = f__solve_F(v_M.clone(), v_r.clone())?;
    v_k = (if ({ let t4149 = v_t.clone(); Py::Bool({ let t4150 = py::getitem(&v_X.clone(), &py::neg(&Py::Int(1i64))?)?; if py::lt(&t4149, &t4150)? { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t4151 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4151;
if ({ let t4152 = py::getitem(&v_X.clone(), &v_j.clone())?; Py::Bool({ let t4153 = v_t.clone(); if py::le(&t4152, &t4153)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }], true)? } else { py::sub(&v_n.clone(), &Py::Int(1i64))? });
    let t4154 = (Py::tuple(vec![py::sub(&py::getitem(&v_X.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_t.clone())?, py::sub(&v_t.clone(), &py::getitem(&v_X.clone(), &v_k.clone())?)?])).items()?;
    if t4154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4154[0].clone();
    v_b = t4154[1].clone();
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::mul(&py::getitem(&v_m.clone(), &v_k.clone())?, &py::pow(&v_a.clone(), &Py::Int(3i64))?)?, &py::mul(&Py::Int(6i64), &py::getitem(&v_h.clone(), &v_k.clone())?)?)?, &py::truediv(&py::mul(&py::getitem(&v_m.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::pow(&v_b.clone(), &Py::Int(3i64))?)?, &py::mul(&Py::Int(6i64), &py::getitem(&v_h.clone(), &v_k.clone())?)?)?)?, &py::mul(&py::sub(&py::truediv(&py::getitem(&v_Y.clone(), &v_k.clone())?, &py::getitem(&v_h.clone(), &v_k.clone())?)?, &py::truediv(&py::mul(&py::getitem(&v_m.clone(), &v_k.clone())?, &py::getitem(&v_h.clone(), &v_k.clone())?)?, &Py::Int(6i64))?)?, &v_a.clone())?)?, &py::mul(&py::sub(&py::truediv(&py::getitem(&v_Y.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_h.clone(), &v_k.clone())?)?, &py::truediv(&py::mul(&py::getitem(&v_m.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_h.clone(), &v_k.clone())?)?, &Py::Int(6i64))?)?, &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sturm_count(mut v_c: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_changes: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_num: Py = Py::None;
    v_P = Py::list(vec![f__FV(v_c.clone())?]);
    v_d = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    let _ = py::method(&v_P.clone(), "append", &[{ let mut out = vec![]; for t166 in py::iter(&(py::enumerate(&py::slice(&py::getitem(&v_P.clone(), &Py::Int(0i64))?, None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_k: Py; let mut v_t: Py; 
let t167 = (t166).items()?;
if t167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t167[0].clone();
v_t = t167[1].clone();
out.push(py::mul(&v_t.clone(), &py::sub(&v_d.clone(), &v_k.clone())?)?); } Py::list(out) }])?;
    while ({ let t178 = py::len(&py::getitem(&v_P.clone(), &py::neg(&Py::Int(1i64))?)?)?; Py::Bool({ let t179 = Py::Int(1i64); if py::gt(&t178, &t179)? { true } else { false } }) }).truthy() {
        let t168 = (Py::tuple(vec![py::slice(&py::getitem(&v_P.clone(), &py::neg(&Py::Int(2i64))?)?, None, None, None)?, py::getitem(&v_P.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_num = t168[0].clone();
        v_den = t168[1].clone();
        while ({ let t172 = py::len(&v_num.clone())?; Py::Bool({ let t173 = py::len(&v_den.clone())?; if py::ge(&t172, &t173)? { true } else { false } }) }).truthy() {
            v_f = py::truediv(&py::getitem(&v_num.clone(), &Py::Int(0i64))?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?;
            for t169 in py::range_iter(&[(py::len(&v_den.clone())?).i()?]) {
                v_k = t169;
                let t170 = v_num.clone(); let t171 = v_k.clone();
                py::setitem(&t170, &t171, py::sub(&py::getitem(&t170, &t171)?, &py::mul(&v_f.clone(), &py::getitem(&v_den.clone(), &v_k.clone())?)?)?)?;
            }
            v_num = py::slice(&v_num.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
        }
        while ({ let t176 = v_num.clone(); if !t176.truthy() { t176 } else { { let t174 = py::getitem(&v_num.clone(), &Py::Int(0i64))?; Py::Bool({ let t175 = Py::Int(0i64); if py::eq(&t174, &t175) { true } else { false } }) } } }).truthy() {
            v_num = py::slice(&v_num.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
        }
        if (py::not(&v_num.clone())).truthy() {
            break;
        }
        let _ = py::method(&v_P.clone(), "append", &[{ let mut out = vec![]; for t177 in py::iter(&(v_num.clone()))? { let mut v_t: Py; 
v_t = t177;
out.push(py::neg(&v_t.clone())?); } Py::list(out) }])?;
    }
    v_changes = { let v_P = v_P.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_changes = v_changes.clone(); let v_d = v_d.clone(); let v_den = v_den.clone(); let v_f = v_f.clone(); let v_k = v_k.clone(); let v_num = v_num.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_vals: Py = Py::None;
            v_vals = { let mut out = vec![]; for t180 in py::iter(&(v_P.clone()))? { let mut v_p: Py; 
v_p = t180;
out.push(py::sum(&{ let mut out = vec![]; for t181 in py::iter(&(py::enumerate(&v_p.clone(), 0)?))? { let mut v_k: Py; let mut v_t: Py; 
let t182 = (t181).items()?;
if t182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t182[0].clone();
v_t = t182[1].clone();
out.push(py::mul(&v_t.clone(), &py::pow(&v_x.clone(), &py::sub(&py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) };
            v_vals = { let mut out = vec![]; for t183 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t183;
if ({ let t184 = v_v.clone(); Py::Bool({ let t185 = Py::Int(0i64); if py::ne(&t184, &t185) { true } else { false } }) }).truthy() { out.push(v_v.clone()); }} Py::list(out) };
            return Ok(py::sum(&{ let mut out = vec![]; for t186 in py::iter(&(py::zip(&[v_vals.clone(), py::slice(&v_vals.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t187 = (t186).items()?;
if t187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t187[0].clone();
v_v = t187[1].clone();
if ({ let t188 = { let t190 = v_u.clone(); Py::Bool({ let t191 = Py::Int(0i64); if py::gt(&t190, &t191)? { true } else { false } }) }; Py::Bool({ let t189 = { let t192 = v_v.clone(); Py::Bool({ let t193 = Py::Int(0i64); if py::gt(&t192, &t193)? { true } else { false } }) }; if py::ne(&t188, &t189) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::sub(&py::call(&v_changes.clone(), &[f__F(v_a.clone(), Py::str("input"))?])?, &py::call(&v_changes.clone(), &[f__F(v_b.clone(), Py::str("input"))?])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__svd_jacobi(mut v_A: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_ci: Py = Py::None;
    let mut v_cj: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_rotated: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sv: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_vi: Py = Py::None;
    let mut v_vj: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let t5753 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t5753.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5753[0].clone();
    v_n = t5753[1].clone();
    v_cols = { let mut out = vec![]; for t5754 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5754;
out.push({ let mut out = vec![]; for t5755 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_i: Py; 
v_i = t5755;
out.push(py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    v_V = { let mut out = vec![]; for t5756 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5756;
out.push({ let mut out = vec![]; for t5757 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5757;
out.push((if ({ let t5758 = v_i.clone(); Py::Bool({ let t5759 = v_j.clone(); if py::eq(&t5758, &t5759) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.0) })); } Py::list(out) }); } Py::list(out) };
    let mut t5760 = false;
    for t5761 in py::range_iter(&[(Py::Int(80i64)).i()?]) {
        v__ = t5761;
        v_rotated = Py::Bool(false);
        for t5762 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t5762;
            for t5763 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
                v_j = t5763;
                let t5764 = (Py::tuple(vec![py::getitem(&v_cols.clone(), &v_i.clone())?, py::getitem(&v_cols.clone(), &v_j.clone())?])).items()?;
                if t5764.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_ci = t5764[0].clone();
                v_cj = t5764[1].clone();
                let t5767 = (Py::tuple(vec![f__dot(v_ci.clone(), v_ci.clone())?, f__dot(v_cj.clone(), v_cj.clone())?, f__dot(v_ci.clone(), v_cj.clone())?])).items()?;
                if t5767.len() != 3 { return Err(PyErr::Value("unpack".into())); }
                v_a = t5767[0].clone();
                v_b = t5767[1].clone();
                v_g = t5767[2].clone();
                if ({ let t5772 = { let t5768 = v_g.clone(); Py::Bool({ let t5769 = Py::Int(0i64); if py::eq(&t5768, &t5769) { true } else { false } }) }; if t5772.truthy() { t5772 } else { { let t5770 = py::abs(&v_g.clone())?; Py::Bool({ let t5771 = py::mul(&py::minmax(&[Py::Float(1e-15), py::mul(&Py::Float(2.3e-16), &v_m.clone())?], true)?, &py::math1("sqrt", &py::mul(&v_a.clone(), &v_b.clone())?)?)?; if py::le(&t5770, &t5771)? { true } else { false } }) } } }).truthy() {
                    continue;
                }
                v_rotated = Py::Bool(true);
                v_zeta = py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &py::mul(&Py::Int(2i64), &v_g.clone())?)?;
                v_t = py::truediv(&py::copysign(&Py::Float(1.0), &v_zeta.clone())?, &py::add(&py::abs(&v_zeta.clone())?, &py::hypot(&[Py::Float(1.0), v_zeta.clone()])?)?)?;
                v_c = py::truediv(&Py::Int(1i64), &py::hypot(&[Py::Float(1.0), v_t.clone()])?)?;
                v_s = py::mul(&v_c.clone(), &v_t.clone())?;
                let t5777 = (Py::tuple(vec![{ let mut out = vec![]; for t5773 in py::iter(&(py::zip(&[v_ci.clone(), v_cj.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5774 = (t5773).items()?;
if t5774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5774[0].clone();
v_y = t5774[1].clone();
out.push(py::sub(&py::mul(&v_c.clone(), &v_x.clone())?, &py::mul(&v_s.clone(), &v_y.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t5775 in py::iter(&(py::zip(&[v_ci.clone(), v_cj.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5776 = (t5775).items()?;
if t5776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5776[0].clone();
v_y = t5776[1].clone();
out.push(py::add(&py::mul(&v_s.clone(), &v_x.clone())?, &py::mul(&v_c.clone(), &v_y.clone())?)?); } Py::list(out) }])).items()?;
                if t5777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_cols.clone(), &v_i.clone(), t5777[0].clone())?;
                py::setitem(&v_cols.clone(), &v_j.clone(), t5777[1].clone())?;
                let t5778 = (Py::tuple(vec![py::getitem(&v_V.clone(), &v_i.clone())?, py::getitem(&v_V.clone(), &v_j.clone())?])).items()?;
                if t5778.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_vi = t5778[0].clone();
                v_vj = t5778[1].clone();
                let t5783 = (Py::tuple(vec![{ let mut out = vec![]; for t5779 in py::iter(&(py::zip(&[v_vi.clone(), v_vj.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5780 = (t5779).items()?;
if t5780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5780[0].clone();
v_y = t5780[1].clone();
out.push(py::sub(&py::mul(&v_c.clone(), &v_x.clone())?, &py::mul(&v_s.clone(), &v_y.clone())?)?); } Py::list(out) }, { let mut out = vec![]; for t5781 in py::iter(&(py::zip(&[v_vi.clone(), v_vj.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t5782 = (t5781).items()?;
if t5782.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t5782[0].clone();
v_y = t5782[1].clone();
out.push(py::add(&py::mul(&v_s.clone(), &v_x.clone())?, &py::mul(&v_c.clone(), &v_y.clone())?)?); } Py::list(out) }])).items()?;
                if t5783.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                py::setitem(&v_V.clone(), &v_i.clone(), t5783[0].clone())?;
                py::setitem(&v_V.clone(), &v_j.clone(), t5783[1].clone())?;
            }
        }
        if (py::not(&v_rotated.clone())).truthy() {
            { t5760 = true; break; }
        }
    }
    if !t5760 {
        return Err(py::value_error(py::to_str(&Py::str("the Jacobi SVD did not converge"))));
    }
    v_sv = { let mut out = vec![]; for t5784 in py::iter(&(v_cols.clone()))? { let mut v_c: Py; 
v_c = t5784;
out.push(f__norm(v_c.clone())?); } Py::list(out) };
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_A = v_A.clone(); let v_V = v_V.clone(); let v__ = v__.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_ci = v_ci.clone(); let v_cj = v_cj.clone(); let v_cols = v_cols.clone(); let v_g = v_g.clone(); let v_i = v_i.clone(); let v_j = v_j.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_rotated = v_rotated.clone(); let v_s = v_s.clone(); let v_sv = v_sv.clone(); let v_t = v_t.clone(); let v_vi = v_vi.clone(); let v_vj = v_vj.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::neg(&py::getitem(&v_sv.clone(), &v_k.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_U = { let mut out = vec![]; for t5786 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t5786;
out.push((if ({ let t5787 = py::getitem(&v_sv.clone(), &v_k.clone())?; Py::Bool({ let t5788 = Py::Int(0i64); if py::gt(&t5787, &t5788)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t5789 in py::iter(&(py::getitem(&v_cols.clone(), &v_k.clone())?))? { let mut v_x: Py; 
v_x = t5789;
out.push(py::truediv(&v_x.clone(), &py::getitem(&v_sv.clone(), &v_k.clone())?)?); } Py::list(out) } } else { py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())? })); } Py::list(out) };
    return Ok(Py::tuple(vec![v_U.clone(), { let mut out = vec![]; for t5790 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t5790;
out.push(py::getitem(&v_sv.clone(), &v_k.clone())?); } Py::list(out) }, { let mut out = vec![]; for t5791 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t5791;
out.push(py::getitem(&v_V.clone(), &v_k.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__table(mut v_x: Py, mut v_y: Py, mut v_min_n: Py, mut v_distinct: Py, mut v_increasing: Py) -> R {
    let mut v_X: Py = Py::None;
    let t6401 = (Py::tuple(vec![f__need_vec(v_x.clone(), Py::str("x"), Py::None, v_min_n.clone(), Py::Int(60i64))?, f__need_vec(v_y.clone(), Py::str("y"), Py::None, v_min_n.clone(), Py::Int(60i64))?])).items()?;
    if t6401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t6401[0].clone();
    v_y = t6401[1].clone();
    if ({ let t6402 = py::len(&v_x.clone())?; Py::Bool({ let t6403 = py::len(&v_y.clone())?; if py::ne(&t6402, &t6403) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y must have the same number of values"))));
    }
    v_X = f__FV(v_x.clone())?;
    if ({ let t6407 = v_distinct.clone(); if !t6407.truthy() { t6407 } else { { let t6405 = py::len(&py::set(&v_X.clone())?)?; Py::Bool({ let t6406 = py::len(&v_X.clone())?; if py::ne(&t6405, &t6406) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the x values must be distinct"))));
    }
    if ({ let t6412 = v_increasing.clone(); if !t6412.truthy() { t6412 } else { Py::Bool(({ let mut out = vec![]; for t6408 in py::iter(&(py::zip(&[v_X.clone(), py::slice(&v_X.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6409 = (t6408).items()?;
if t6409.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6409[0].clone();
v_b = t6409[1].clone();
out.push({ let t6410 = v_a.clone(); Py::Bool({ let t6411 = v_b.clone(); if py::ge(&t6410, &t6411)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the x values must be strictly increasing"))));
    }
    return Ok(Py::tuple(vec![v_X.clone(), f__FV(v_y.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__unit(mut v_v: Py) -> R {
    let mut v_nv: Py = Py::None;
    v_nv = f__norm(v_v.clone())?;
    return Ok(f__sign_fix({ let mut out = vec![]; for t5012 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t5012;
out.push(py::truediv(&v_x.clone(), &v_nv.clone())?); } Py::list(out) }, Py::Float(1e-09))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_adjugate(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = py::getitem(&f_adjugate(py::getitem(&v_i.clone(), &Py::str("A"))?)?, &Py::str("adjugate"))?;
    return Ok(f__diff(py::getitem(&v_o.clone(), &Py::str("adjugate"))?, v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_angle(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_in_range: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sin_side: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_uu: Py = Py::None;
    let mut v_uv: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let t110 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?, py::getitem(&v_o.clone(), &Py::str("angle_radians"))?])).items()?;
    if t110.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t110[0].clone();
    v_v = t110[1].clone();
    v_t = t110[2].clone();
    let t112 = (Py::tuple(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?])).items()?;
    if t112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U = t112[0].clone();
    v_V = t112[1].clone();
    let t117 = ({ let mut out = vec![]; for t113 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_U.clone(), v_U.clone()]), Py::tuple(vec![v_V.clone(), v_V.clone()]), Py::tuple(vec![v_U.clone(), v_V.clone()])])))? { let mut v_X: Py; let mut v_Y: Py; 
let t114 = (t113).items()?;
if t114.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_X = t114[0].clone();
v_Y = t114[1].clone();
out.push(py::sum(&{ let mut out = vec![]; for t115 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t116 = (t115).items()?;
if t116.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t116[0].clone();
v_b = t116[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }).items()?;
    if t117.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_uu = t117[0].clone();
    v_vv = t117[1].clone();
    v_uv = t117[2].clone();
    v_s = py::mul(&py::math1("sqrt", &py::float(&v_uu.clone())?)?, &py::math1("sqrt", &py::float(&v_vv.clone())?)?)?;
    v_sin_side = py::math1("sqrt", &py::float(&py::sub(&py::mul(&v_uu.clone(), &v_vv.clone())?, &py::mul(&v_uv.clone(), &v_uv.clone())?)?)?)?;
    v_in_range = (if ({ let t118 = py::neg(&Py::Float(1e-15))?; Py::Bool({ let t119 = v_t.clone(); if py::le(&t118, &t119)? { { let t120 = py::add(&Py::Float(std::f64::consts::PI), &Py::Float(1e-15))?; if py::le(&t119, &t120)? { true } else { false } } } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? });
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("cos", &v_t.clone())?, &v_s.clone())?, &py::float(&v_uv.clone())?)?)?, &v_s.clone())?, py::truediv(&py::abs(&py::sub(&py::mul(&py::math1("sin", &v_t.clone())?, &v_s.clone())?, &v_sin_side.clone())?)?, &v_s.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("angle_degrees"))?, &py::math1("degrees", &v_t.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_o.clone(), &Py::str("angle_degrees"))?)?], true)?)?, v_in_range.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_charpoly(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let t383 = (Py::tuple(vec![f__FM(py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::len(&py::getitem(&v_i.clone(), &Py::str("A"))?)?])).items()?;
    if t383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t383[0].clone();
    v_n = t383[1].clone();
    v_co = py::getitem(&v_o.clone(), &Py::str("coefficients"))?;
    if ({ let t384 = py::len(&v_co.clone())?; Py::Bool({ let t385 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::ne(&t384, &t385) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    let t387 = (Py::tuple(vec![{ let mut out = vec![]; for t386 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t386;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) }, Py::list(vec![])])).items()?;
    if t387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t387[0].clone();
    v_p = t387[1].clone();
    for t388 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t388;
        let _ = py::method(&v_p.clone(), "append", &[py::sum(&{ let mut out = vec![]; for t389 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t389;
out.push(py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_j.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?])?;
        v_P = f__mm_F(v_P.clone(), v_F.clone())?;
    }
    v_c = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    for t390 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t390;
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::neg(&py::add(&py::getitem(&v_p.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &py::sum(&{ let mut out = vec![]; for t391 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_k.clone()).i()?]) { let mut v_m: Py; 
v_m = t391;
out.push(py::mul(&py::getitem(&v_c.clone(), &v_m.clone())?, &py::getitem(&v_p.clone(), &py::sub(&py::sub(&v_k.clone(), &Py::Int(1i64))?, &v_m.clone())?)?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?, &v_k.clone())?])?;
    }
    return Ok(f__diff(v_co.clone(), { let mut out = vec![]; for t404 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t404;
out.push(py::float(&v_x.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_chol(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_LLt: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t467 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("L"))?])).items()?;
    if t467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t467[0].clone();
    v_L = t467[1].clone();
    v_n = py::len(&v_A.clone())?;
    if ({ let t479 = { let t468 = f__shape(v_L.clone())?; Py::Bool({ let t469 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t468, &t469) { true } else { false } }) }; if t479.truthy() { t479 } else { { let t478 = Py::Bool(({ let mut out = vec![]; for t471 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t471;
for t472 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t472;
out.push({ let t473 = py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t474 = Py::Int(0i64); if py::ne(&t473, &t474) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t478.truthy() { t478 } else { Py::Bool(({ let mut out = vec![]; for t475 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t475;
out.push({ let t476 = py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_r.clone())?; Py::Bool({ let t477 = Py::Int(0i64); if py::le(&t476, &t477)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_LLt = f__mm(v_L.clone(), f__T(v_L.clone())?)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t485 in py::iter(&(py::zip(&[v_LLt.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t486 = (t485).items()?;
if t486.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t486[0].clone();
v_t = t486[1].clone();
for t487 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t488 = (t487).items()?;
if t488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t488[0].clone();
v_b = t488[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cond(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_A = py::getitem(&v_i.clone(), &Py::str("A"))?;
    v_s = py::getitem(&f__svd_jacobi({ let mut out = vec![]; for t783 in py::iter(&(f__T(v_A.clone())?))? { let mut v_r: Py; 
v_r = t783;
out.push(py::slice(&v_r.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?); } Py::list(out) })?, &Py::Int(1i64))?;
    v_F = f__FM(v_A.clone())?;
    v_n1 = { let v_A = v_A.clone(); let v_F = v_F.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[{ let mut out = vec![]; for t784 in py::range_iter(&[(py::len(&v_M.clone())?).i()?]) { let mut v_j: Py; 
v_j = t784;
out.push(py::sum(&{ let mut out = vec![]; for t785 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t785;
out.push(py::abs(&py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(py::minmax(&[(if ({ let t786 = py::getitem(&v_o.clone(), &Py::str("cond2"))?; Py::Bool({ let t787 = Py::Int(0i64); if py::gt(&t786, &t787)? { true } else { false } }) }).truthy() { f__rel(py::getitem(&v_o.clone(), &Py::str("cond2"))?, py::truediv(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?)?)? } else { c_BAD()? }), f__rel(py::getitem(&v_o.clone(), &Py::str("cond1"))?, py::float(&py::mul(&py::call(&v_n1.clone(), &[v_F.clone()])?, &py::call(&v_n1.clone(), &[f__inv_F(v_F.clone())?])?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_cross(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_det: Py = Py::None;
    let mut v_lag: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t917 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?, py::getitem(&v_o.clone(), &Py::str("cross"))?])).items()?;
    if t917.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t917[0].clone();
    v_v = t917[1].clone();
    v_w = t917[2].clone();
    if ({ let t918 = py::len(&v_w.clone())?; Py::Bool({ let t919 = Py::Int(3i64); if py::ne(&t918, &t919) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_s = py::minmax(&[Py::Float(1.0), py::mul(&f__norm(v_u.clone())?, &f__norm(v_v.clone())?)?], true)?;
    v_lag = py::sub(&py::mul(&py::pow(&f__norm(v_u.clone())?, &Py::Int(2i64))?, &py::pow(&f__norm(v_v.clone())?, &Py::Int(2i64))?)?, &py::pow(&f__dot(v_u.clone(), v_v.clone())?, &Py::Int(2i64))?)?;
    v_det = py::fsum(&Py::list(vec![py::mul(&py::getitem(&v_u.clone(), &Py::Int(0i64))?, &py::sub(&py::mul(&py::getitem(&v_v.clone(), &Py::Int(1i64))?, &py::getitem(&v_w.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_v.clone(), &Py::Int(2i64))?, &py::getitem(&v_w.clone(), &Py::Int(1i64))?)?)?)?, py::mul(&py::neg(&py::getitem(&v_u.clone(), &Py::Int(1i64))?)?, &py::sub(&py::mul(&py::getitem(&v_v.clone(), &Py::Int(0i64))?, &py::getitem(&v_w.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_v.clone(), &Py::Int(2i64))?, &py::getitem(&v_w.clone(), &Py::Int(0i64))?)?)?)?, py::mul(&py::getitem(&v_u.clone(), &Py::Int(2i64))?, &py::sub(&py::mul(&py::getitem(&v_v.clone(), &Py::Int(0i64))?, &py::getitem(&v_w.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_v.clone(), &Py::Int(1i64))?, &py::getitem(&v_w.clone(), &Py::Int(0i64))?)?)?)?]))?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&f__dot(v_w.clone(), v_u.clone())?)?, &py::mul(&v_s.clone(), &py::minmax(&[Py::Float(1.0), f__norm(v_u.clone())?], true)?)?)?, py::truediv(&py::abs(&f__dot(v_w.clone(), v_v.clone())?)?, &py::mul(&v_s.clone(), &py::minmax(&[Py::Float(1.0), f__norm(v_v.clone())?], true)?)?)?, py::truediv(&py::abs(&py::sub(&f__dot(v_w.clone(), v_w.clone())?, &v_lag.clone())?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?, py::truediv(&py::abs(&py::sub(&v_det.clone(), &f__dot(v_w.clone(), v_w.clone())?)?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_det_value(mut v_i: Py, mut v_o: Py, mut v_key: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = py::float(&f__det_F(f__FM(py::getitem(&v_i.clone(), &Py::str("A"))?)?)?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &v_key.clone())?, &v_want.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_want.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_eig2(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tr: Py = Py::None;
    v_A = py::getitem(&v_i.clone(), &Py::str("A"))?;
    let t1426 = (Py::tuple(vec![py::complex(&[py::getitem(&v_o.clone(), &Py::str("lambda1"))?])?, py::complex(&[py::getitem(&v_o.clone(), &Py::str("lambda2"))?])?])).items()?;
    if t1426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t1426[0].clone();
    v_l2 = t1426[1].clone();
    let t1427 = (Py::tuple(vec![py::add(&py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?])).items()?;
    if t1427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t1427[0].clone();
    v_det = t1427[1].clone();
    v_s = py::minmax(&[Py::Float(1.0), py::add(&py::abs(&v_l1.clone())?, &py::abs(&v_l2.clone())?)?, f__fro(v_A.clone())?], true)?;
    v_order = Py::Float(0.0);
    if ({ let t1434 = { let t1430 = py::abs(&py::imag(&v_l1.clone())?)?; Py::Bool({ let t1431 = Py::Int(0i64); if py::gt(&t1430, &t1431)? { true } else { false } }) }; if t1434.truthy() { t1434 } else { { let t1432 = py::abs(&py::imag(&v_l2.clone())?)?; Py::Bool({ let t1433 = Py::Int(0i64); if py::gt(&t1432, &t1433)? { true } else { false } }) } } }).truthy() {
        v_order = (if ({ let t1439 = { let t1435 = py::imag(&v_l1.clone())?; Py::Bool({ let t1436 = Py::Int(0i64); if py::gt(&t1435, &t1436)? { true } else { false } }) }; if !t1439.truthy() { t1439 } else { { let t1437 = py::abs(&py::sub(&v_l1.clone(), &py::method(&v_l2.clone(), "conjugate", &[])?)?)?; Py::Bool({ let t1438 = py::mul(&Py::Float(1e-12), &v_s.clone())?; if py::le(&t1437, &t1438)? { true } else { false } }) } } }).truthy() { Py::Float(0.0) } else { c_BAD()? });
    } else {
        if ({ let t1440 = py::real(&v_l1.clone())?; Py::Bool({ let t1441 = py::real(&v_l2.clone())?; if py::gt(&t1440, &t1441)? { true } else { false } }) }).truthy() {
            v_order = c_BAD()?;
        }
    }
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::add(&v_l1.clone(), &v_l2.clone())?, &v_tr.clone())?)?, &v_s.clone())?, py::truediv(&py::abs(&py::sub(&py::mul(&v_l1.clone(), &v_l2.clone())?, &v_det.clone())?)?, &py::mul(&v_s.clone(), &v_s.clone())?)?, v_order.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_eigvecs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t6337 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("eigenvalues"))?, py::getitem(&v_o.clone(), &Py::str("eigenvectors"))?])).items()?;
    if t6337.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t6337[0].clone();
    v_lam = t6337[1].clone();
    v_V = t6337[2].clone();
    v_n = py::len(&v_A.clone())?;
    if ({ let t6350 = { let t6338 = py::len(&v_lam.clone())?; Py::Bool({ let t6339 = v_n.clone(); if py::ne(&t6338, &t6339) { true } else { false } }) }; if t6350.truthy() { t6350 } else { { let t6349 = { let t6340 = f__shape(v_V.clone())?; Py::Bool({ let t6341 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t6340, &t6341) { true } else { false } }) }; if t6349.truthy() { t6349 } else { { let t6348 = { let t6343 = v_lam.clone(); Py::Bool({ let t6344 = py::sorted(&v_lam.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t6343, &t6344) { true } else { false } }) }; if t6348.truthy() { t6348 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t6345 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t6345;
out.push(f__sign_ok(v_v.clone(), Py::Float(1e-09))?); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    v_e1 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t6351 in py::iter(&(py::zip(&[v_lam.clone(), v_V.clone()])?))? { let mut v_l: Py; let mut v_v: Py; 
let t6352 = (t6351).items()?;
if t6352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_l = t6352[0].clone();
v_v = t6352[1].clone();
out.push(f__norm({ let mut out = vec![]; for t6356 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_v.clone())?, v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6357 = (t6356).items()?;
if t6357.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6357[0].clone();
v_b = t6357[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_l.clone(), &v_b.clone())?)?); } Py::list(out) })?); } Py::list(out) }], true)?, &v_sc.clone())?;
    v_e2 = py::minmax(&[{ let mut out = vec![]; for t6363 in py::iter(&(py::zip(&[f__mm(v_V.clone(), f__T(v_V.clone())?)?, f__eye(v_n.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t6364 = (t6363).items()?;
if t6364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t6364[0].clone();
v_t = t6364[1].clone();
for t6365 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6366 = (t6365).items()?;
if t6366.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6366[0].clone();
v_b = t6366[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone(), f__verify_spectrum(v_A.clone(), v_lam.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_expm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_AE: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_EA: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t3208 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("exponential"))?])).items()?;
    if t3208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3208[0].clone();
    v_E = t3208[1].clone();
    if ({ let t3209 = f__shape(v_E.clone())?; Py::Bool({ let t3210 = f__shape(v_A.clone())?; if py::ne(&t3209, &t3210) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_E2 = f__expm(v_A.clone(), Py::Int(2i64))?;
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t3212 in py::iter(&(v_E2.clone()))? { let mut v_r: Py; 
v_r = t3212;
for t3213 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3213;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?], true)?;
    v_e1 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3214 in py::iter(&(py::zip(&[v_E.clone(), v_E2.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t3215 = (t3214).items()?;
if t3215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3215[0].clone();
v_t = t3215[1].clone();
for t3216 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3217 = (t3216).items()?;
if t3217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3217[0].clone();
v_b = t3217[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &v_sc.clone())?;
    let t3218 = (Py::tuple(vec![f__mm(v_A.clone(), v_E.clone())?, f__mm(v_E.clone(), v_A.clone())?])).items()?;
    if t3218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_AE = t3218[0].clone();
    v_EA = t3218[1].clone();
    v_e2 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3219 in py::iter(&(py::zip(&[v_AE.clone(), v_EA.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t3220 = (t3219).items()?;
if t3220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3220[0].clone();
v_t = t3220[1].clone();
for t3221 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3222 = (t3221).items()?;
if t3222.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3222[0].clone();
v_b = t3222[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::mul(&v_sc.clone(), &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?)?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_general_eig(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_canon: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let t1795 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("eigenvalues"))?])).items()?;
    if t1795.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1795[0].clone();
    v_ev = t1795[1].clone();
    if ({ let t1796 = py::len(&v_ev.clone())?; Py::Bool({ let t1797 = py::len(&v_A.clone())?; if py::ne(&t1796, &t1797) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    let t1798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_canon = f__real_spectrum(v_ev.clone(), f__fro(v_A.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    if (Py::Bool(({ let mut out = vec![]; for t1799 in py::iter(&(py::zip(&[v_canon.clone(), v_ev.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1800 = (t1799).items()?;
if t1800.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1800[0].clone();
v_b = t1800[1].clone();
out.push({ let t1801 = py::complex(&[v_a.clone()])?; Py::Bool({ let t1802 = py::complex(&[v_b.clone()])?; if py::ne(&t1801, &t1802) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(c_BAD()?);
    }
    return Ok(py::minmax(&[f__verify_spectrum(v_A.clone(), v_ev.clone())?, f__rel(py::getitem(&v_o.clone(), &Py::str("spectral_radius"))?, py::minmax(&[{ let mut out = vec![]; for t1817 in py::iter(&(v_ev.clone()))? { let mut v_z: Py; 
v_z = t1817;
out.push(py::abs(&py::complex(&[v_z.clone()])?)?); } Py::list(out) }], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gl(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_exact: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_n = py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?;
    let t1553 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("nodes"))?, py::getitem(&v_o.clone(), &Py::str("weights"))?])).items()?;
    if t1553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t1553[0].clone();
    v_ws = t1553[1].clone();
    if ({ let t1561 = { let t1554 = py::len(&v_xs.clone())?; Py::Bool({ let t1555 = v_n.clone(); if py::ne(&t1554, &t1555) { true } else { false } }) }; if t1561.truthy() { t1561 } else { { let t1560 = { let t1556 = py::len(&v_ws.clone())?; Py::Bool({ let t1557 = v_n.clone(); if py::ne(&t1556, &t1557) { true } else { false } }) }; if t1560.truthy() { t1560 } else { { let t1558 = v_xs.clone(); Py::Bool({ let t1559 = py::sorted(&v_xs.clone(), (Py::Bool(false)).truthy())?; if py::ne(&t1558, &t1559) { true } else { false } }) } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_worst = Py::Float(0.0);
    for t1562 in py::range_iter(&[(py::mul(&Py::Int(2i64), &v_n.clone())?).i()?]) {
        v_k = t1562;
        v_exact = (if (py::modulo(&v_k.clone(), &Py::Int(2i64))?).truthy() { Py::Float(0.0) } else { py::truediv(&Py::Float(2.0), &py::add(&v_k.clone(), &Py::Int(1i64))?)? });
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1563 in py::iter(&(py::zip(&[v_xs.clone(), v_ws.clone()])?))? { let mut v_w: Py; let mut v_x: Py; 
let t1564 = (t1563).items()?;
if t1564.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1564[0].clone();
v_w = t1564[1].clone();
out.push(py::mul(&v_w.clone(), &py::pow(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) })?, &v_exact.clone())?)?, &py::minmax(&[v_exact.clone(), py::truediv(&Py::Float(0.001), &py::add(&v_k.clone(), &Py::Int(1i64))?)?], true)?)?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_gs(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_back: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_e3: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t1889 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("vectors"))?, py::getitem(&v_o.clone(), &Py::str("orthonormal"))?])).items()?;
    if t1889.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_V = t1889[0].clone();
    v_E = t1889[1].clone();
    v_k = py::len(&v_V.clone())?;
    if ({ let t1890 = f__shape(v_E.clone())?; Py::Bool({ let t1891 = f__shape(v_V.clone())?; if py::ne(&t1890, &t1891) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_G = f__mm(v_E.clone(), f__T(v_E.clone())?)?;
    v_e1 = py::minmax(&[{ let mut out = vec![]; for t1902 in py::iter(&(py::zip(&[v_G.clone(), f__eye(v_k.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t1903 = (t1902).items()?;
if t1903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1903[0].clone();
v_t = t1903[1].clone();
for t1904 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1905 = (t1904).items()?;
if t1905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1905[0].clone();
v_b = t1905[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    v_R = f__mm(v_E.clone(), f__T(v_V.clone())?)?;
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_V.clone())?], true)?;
    v_e2 = py::truediv(&{ let t = { let mut out = vec![]; for t1908 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_a: Py; 
v_a = t1908;
for t1909 in py::range_iter(&[(v_a.clone()).i()?]) { let mut v_b: Py; 
v_b = t1909;
out.push(py::abs(&py::getitem(&py::getitem(&v_R.clone(), &v_a.clone())?, &v_b.clone())?)?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } }, &v_sc.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t1910 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_a: Py; 
v_a = t1910;
out.push({ let t1911 = py::getitem(&py::getitem(&v_R.clone(), &v_a.clone())?, &v_a.clone())?; Py::Bool({ let t1912 = Py::Int(0i64); if py::le(&t1911, &t1912)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(c_BAD()?);
    }
    v_back = f__mm(f__T(v_R.clone())?, v_E.clone())?;
    v_e3 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1913 in py::iter(&(py::zip(&[v_back.clone(), v_V.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t1914 = (t1913).items()?;
if t1914.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1914[0].clone();
v_t = t1914[1].clone();
for t1915 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t1916 = (t1915).items()?;
if t1916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1916[0].clone();
v_y = t1916[1].clone();
out.push(py::abs(&py::sub(&v_x.clone(), &v_y.clone())?)?); }} Py::list(out) }], true)?, &v_sc.clone())?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone(), v_e3.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_inverse(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3290 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("inverse"))?])).items()?;
    if t3290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3290[0].clone();
    v_X = t3290[1].clone();
    v_n = py::len(&v_A.clone())?;
    if ({ let t3291 = f__shape(v_X.clone())?; Py::Bool({ let t3292 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t3291, &t3292) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_s = py::minmax(&[Py::Float(1.0), py::mul(&f__fro(v_A.clone())?, &f__fro(v_X.clone())?)?], true)?;
    v_I = f__eye(v_n.clone())?;
    v_r1 = py::minmax(&[{ let mut out = vec![]; for t3304 in py::iter(&(py::zip(&[f__mm(v_A.clone(), v_X.clone())?, v_I.clone()])?))? { let mut v_ra: Py; let mut v_rb: Py; 
let t3305 = (t3304).items()?;
if t3305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ra = t3305[0].clone();
v_rb = t3305[1].clone();
for t3306 in py::iter(&(py::zip(&[v_ra.clone(), v_rb.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3307 = (t3306).items()?;
if t3307.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3307[0].clone();
v_b = t3307[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    v_r2 = py::minmax(&[{ let mut out = vec![]; for t3308 in py::iter(&(py::zip(&[f__mm(v_X.clone(), v_A.clone())?, v_I.clone()])?))? { let mut v_ra: Py; let mut v_rb: Py; 
let t3309 = (t3308).items()?;
if t3309.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_ra = t3309[0].clone();
v_rb = t3309[1].clone();
for t3310 in py::iter(&(py::zip(&[v_ra.clone(), v_rb.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3311 = (t3310).items()?;
if t3311.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3311[0].clone();
v_b = t3311[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    return Ok(py::truediv(&py::minmax(&[v_r1.clone(), v_r2.clone()], true)?, &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_line(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_my: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sst: Py = Py::None;
    let mut v_sx: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2431 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("y"))?])).items()?;
    if t2431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2431[0].clone();
    v_y = t2431[1].clone();
    v_r = { let mut out = vec![]; for t2432 in py::iter(&(py::zip(&[v_x.clone(), v_y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2433 = (t2432).items()?;
if t2433.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2433[0].clone();
v_b = t2433[1].clone();
out.push(py::sub(&py::sub(&v_b.clone(), &py::getitem(&v_o.clone(), &Py::str("intercept"))?)?, &py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &v_a.clone())?)?); } Py::list(out) };
    v_sc = py::minmax(&[Py::Float(1.0), py::add(&py::add(&py::fsum(&{ let mut out = vec![]; for t2434 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t2434;
out.push(py::abs(&v_b.clone())?); } Py::list(out) })?, &py::fsum(&{ let mut out = vec![]; for t2435 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t2435;
out.push(py::abs(&py::mul(&py::getitem(&v_o.clone(), &Py::str("slope"))?, &v_a.clone())?)?); } Py::list(out) })?)?, &py::mul(&py::len(&v_x.clone())?, &py::abs(&py::getitem(&v_o.clone(), &Py::str("intercept"))?)?)?)?], true)?;
    v_sx = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2436 in py::iter(&(v_x.clone()))? { let mut v_a: Py; 
v_a = t2436;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }], true)?], true)?;
    v_my = py::truediv(&py::fsum(&v_y.clone())?, &py::len(&v_y.clone())?)?;
    v_sst = py::fsum(&{ let mut out = vec![]; for t2437 in py::iter(&(v_y.clone()))? { let mut v_b: Py; 
v_b = t2437;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_r2 = py::sub(&Py::Int(1i64), &py::truediv(&py::fsum(&{ let mut out = vec![]; for t2438 in py::iter(&(v_r.clone()))? { let mut v_e: Py; 
v_e = t2438;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) })?, &v_sst.clone())?)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::fsum(&v_r.clone())?)?, &v_sc.clone())?, py::truediv(&py::abs(&py::fsum(&{ let mut out = vec![]; for t2439 in py::iter(&(py::zip(&[v_x.clone(), v_r.clone()])?))? { let mut v_a: Py; let mut v_e: Py; 
let t2440 = (t2439).items()?;
if t2440.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2440[0].clone();
v_e = t2440[1].clone();
out.push(py::mul(&v_a.clone(), &v_e.clone())?); } Py::list(out) })?)?, &py::mul(&v_sc.clone(), &v_sx.clone())?)?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("r_squared"))?, &v_r2.clone())?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lowrank(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2962 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t2962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2962[0].clone();
    v_k = t2962[1].clone();
    v_w = f_low_rank_approximation({ let mut out = vec![]; for t2963 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t2963;
out.push(py::slice(&v_r.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?); } Py::list(out) }, v_k.clone())?;
    v_B = { let mut out = vec![]; for t2964 in py::iter(&(py::getitem(&v_w.clone(), &Py::str("approximation"))?))? { let mut v_r: Py; 
v_r = t2964;
out.push(py::slice(&v_r.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?); } Py::list(out) };
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    return Ok((if ({ let t2967 = f__shape(py::getitem(&v_o.clone(), &Py::str("approximation"))?)?; Py::Bool({ let t2968 = f__shape(v_A.clone())?; if py::eq(&t2967, &t2968) { true } else { false } }) }).truthy() { py::minmax(&[py::truediv(&py::minmax(&[{ let mut out = vec![]; for t2970 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("approximation"))?, v_B.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t2971 = (t2970).items()?;
if t2971.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t2971[0].clone();
v_t = t2971[1].clone();
for t2972 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2973 = (t2972).items()?;
if t2973.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2973[0].clone();
v_b = t2973[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &v_sc.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("error2"))?, &py::getitem(&v_w.clone(), &Py::str("error2"))?)?)?, &v_sc.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("errorF"))?, &py::getitem(&v_w.clone(), &Py::str("errorF"))?)?)?, &v_sc.clone())?], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lstsq(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2621 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_o.clone(), &Py::str("solution"))?])).items()?;
    if t2621.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2621[0].clone();
    v_b = t2621[1].clone();
    v_x = t2621[2].clone();
    if ({ let t2622 = py::len(&v_x.clone())?; Py::Bool({ let t2623 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::ne(&t2622, &t2623) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_r = { let mut out = vec![]; for t2627 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_x.clone())?, v_b.clone()])?))? { let mut v_bi: Py; let mut v_v: Py; 
let t2628 = (t2627).items()?;
if t2628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2628[0].clone();
v_bi = t2628[1].clone();
out.push(py::sub(&v_bi.clone(), &v_v.clone())?); } Py::list(out) };
    v_g = f__mv(f__T(v_A.clone())?, v_r.clone())?;
    v_s = py::mul(&f__fro(v_A.clone())?, &py::add(&f__norm(v_b.clone())?, &py::mul(&f__fro(v_A.clone())?, &f__norm(v_x.clone())?)?)?)?;
    return Ok(py::minmax(&[py::truediv(&f__norm(v_g.clone())?, &py::minmax(&[Py::Float(1e-300), v_s.clone()], true)?)?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("residual_norm"))?, &f__norm(v_r.clone())?)?)?, &py::minmax(&[Py::Float(1.0), f__norm(v_b.clone())?], true)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lu(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_PLU: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t3044 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("P"))?, py::getitem(&v_o.clone(), &Py::str("L"))?, py::getitem(&v_o.clone(), &Py::str("U"))?])).items()?;
    if t3044.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3044[0].clone();
    v_P = t3044[1].clone();
    v_L = t3044[2].clone();
    v_U = t3044[3].clone();
    v_n = py::len(&v_A.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t3045 in py::iter(&(Py::tuple(vec![v_P.clone(), v_L.clone(), v_U.clone()])))? { let mut v_M: Py; 
v_M = t3045;
out.push({ let t3046 = f__shape(v_M.clone())?; Py::Bool({ let t3047 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t3046, &t3047) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t3060 = { let t3049 = py::sorted(&{ let mut out = vec![]; for t3051 in py::iter(&(v_P.clone()))? { let mut v_r: Py; 
v_r = t3051;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) }, (Py::Bool(false)).truthy())?; Py::Bool({ let t3050 = py::sorted(&{ let mut out = vec![]; for t3056 in py::iter(&(f__eye(v_n.clone())?))? { let mut v_r: Py; 
v_r = t3056;
out.push(Py::tuple((v_r.clone()).items()?)); } Py::list(out) }, (Py::Bool(false)).truthy())?; if py::ne(&t3049, &t3050) { true } else { false } }) }; if t3060.truthy() { t3060 } else { Py::Bool(({ let mut out = vec![]; for t3057 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_P.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t3057;
out.push({ let t3058 = py::sum(&v_c.clone(), None)?; Py::Bool({ let t3059 = Py::Int(1i64); if py::ne(&t3058, &t3059) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t3068 = Py::Bool(({ let mut out = vec![]; for t3061 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3061;
out.push({ let t3062 = py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_r.clone())?; Py::Bool({ let t3063 = Py::Float(1.0); if py::ne(&t3062, &t3063) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3068.truthy() { t3068 } else { Py::Bool(({ let mut out = vec![]; for t3064 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3064;
for t3065 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t3065;
out.push({ let t3066 = py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t3067 = Py::Int(0i64); if py::ne(&t3066, &t3067) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t3077 = Py::Bool(({ let mut out = vec![]; for t3069 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3069;
for t3070 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_c: Py; 
v_c = t3070;
out.push({ let t3071 = py::abs(&py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_c.clone())?)?; Py::Bool({ let t3072 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::gt(&t3071, &t3072)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t3077.truthy() { t3077 } else { Py::Bool(({ let mut out = vec![]; for t3073 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3073;
for t3074 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_c: Py; 
v_c = t3074;
out.push({ let t3075 = py::getitem(&py::getitem(&v_U.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t3076 = Py::Int(0i64); if py::ne(&t3075, &t3076) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_PLU = f__mm(v_P.clone(), f__mm(v_L.clone(), v_U.clone())?)?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3082 in py::iter(&(py::zip(&[v_PLU.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t3083 = (t3082).items()?;
if t3083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3083[0].clone();
v_t = t3083[1].clone();
for t3084 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3085 = (t3084).items()?;
if t3085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3085[0].clone();
v_b = t3085[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_matmul(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3357 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, py::getitem(&v_o.clone(), &Py::str("product"))?])).items()?;
    if t3357.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3357[0].clone();
    v_B = t3357[1].clone();
    v_C = t3357[2].clone();
    if ({ let t3358 = f__shape(v_C.clone())?; Py::Bool({ let t3359 = Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_B.clone(), &Py::Int(0i64))?)?]); if py::ne(&t3358, &t3359) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_worst = Py::Float(0.0);
    for t3363 in py::iter(&(Py::tuple(vec![py::mul(&Py::list(vec![Py::Float(1.0)]), &py::len(&py::getitem(&v_B.clone(), &Py::Int(0i64))?)?)?, { let mut out = vec![]; for t3361 in py::range_iter(&[(py::len(&py::getitem(&v_B.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t3361;
out.push(py::float(&py::add(&v_j.clone(), &Py::Int(1i64))?)?); } Py::list(out) }, { let mut out = vec![]; for t3362 in py::range_iter(&[(py::len(&py::getitem(&v_B.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t3362;
out.push(py::pow(&py::neg(&Py::Float(1.0))?, &v_j.clone())?); } Py::list(out) }])))? {
        v_x = t3363;
        let t3367 = (Py::tuple(vec![f__mv(v_C.clone(), v_x.clone())?, f__mv(v_A.clone(), f__mv(v_B.clone(), v_x.clone())?)?])).items()?;
        if t3367.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lhs = t3367[0].clone();
        v_rhs = t3367[1].clone();
        v_scale = py::minmax(&[Py::Float(1.0), py::mul(&py::mul(&f__fro(v_A.clone())?, &f__fro(v_B.clone())?)?, &f__norm(v_x.clone())?)?], true)?;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3371 in py::iter(&(py::zip(&[v_lhs.clone(), v_rhs.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3372 = (t3371).items()?;
if t3372.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3372[0].clone();
v_b = t3372[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }], true)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_norm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_F = f__FV(py::getitem(&v_i.clone(), &Py::str("v"))?)?;
    v_s = py::float(&py::sum(&{ let mut out = vec![]; for t6634 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6634;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("norm"))?, &Py::Int(2i64))?, &v_s.clone())?)?, &py::minmax(&[Py::Float(1.0), v_s.clone()], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("norm1"))?, py::float(&py::sum(&{ let mut out = vec![]; for t6635 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6635;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }, None)?)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("norm_inf"))?, py::float(&py::minmax(&[{ let mut out = vec![]; for t6636 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6636;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }], true)?)?)?, (if ({ let t6637 = py::getitem(&v_o.clone(), &Py::str("norm"))?; Py::Bool({ let t6638 = Py::Int(0i64); if py::ge(&t6637, &t6638)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_null(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = f_null_space(py::getitem(&v_i.clone(), &Py::str("A"))?)?;
    return Ok(py::minmax(&[f__diff(py::getitem(&v_o.clone(), &Py::str("nullity"))?, py::getitem(&v_want.clone(), &Py::str("nullity"))?)?, f__diff(py::getitem(&v_o.clone(), &Py::str("basis"))?, py::getitem(&v_want.clone(), &Py::str("basis"))?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_pinv(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_AX: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_XA: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_e3: Py = Py::None;
    let mut v_e4: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t5238 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("pseudoinverse"))?])).items()?;
    if t5238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5238[0].clone();
    v_X = t5238[1].clone();
    if ({ let t5239 = f__shape(v_X.clone())?; Py::Bool({ let t5240 = Py::tuple(vec![py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::len(&v_A.clone())?]); if py::ne(&t5239, &t5240) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    let t5246 = (Py::tuple(vec![f__mm(v_A.clone(), v_X.clone())?, f__mm(v_X.clone(), v_A.clone())?])).items()?;
    if t5246.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_AX = t5246[0].clone();
    v_XA = t5246[1].clone();
    v_s = py::minmax(&[Py::Float(1.0), py::mul(&f__fro(v_A.clone())?, &f__fro(v_X.clone())?)?], true)?;
    v_e1 = py::truediv(&f__fro({ let mut out = vec![]; for t5249 in py::iter(&(py::zip(&[f__mm(v_AX.clone(), v_A.clone())?, v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5250 = (t5249).items()?;
if t5250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5250[0].clone();
v_t = t5250[1].clone();
out.push({ let mut out = vec![]; for t5251 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5252 = (t5251).items()?;
if t5252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5252[0].clone();
v_b = t5252[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) })?, &py::minmax(&[Py::Float(1e-300), py::mul(&f__fro(v_A.clone())?, &v_s.clone())?], true)?)?;
    v_e2 = py::truediv(&f__fro({ let mut out = vec![]; for t5253 in py::iter(&(py::zip(&[f__mm(v_XA.clone(), v_X.clone())?, v_X.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5254 = (t5253).items()?;
if t5254.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5254[0].clone();
v_t = t5254[1].clone();
out.push({ let mut out = vec![]; for t5255 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5256 = (t5255).items()?;
if t5256.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5256[0].clone();
v_b = t5256[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) })?, &py::minmax(&[Py::Float(1e-300), py::mul(&f__fro(v_X.clone())?, &v_s.clone())?], true)?)?;
    v_e3 = py::truediv(&f__fro({ let mut out = vec![]; for t5257 in py::iter(&(py::zip(&[v_AX.clone(), f__T(v_AX.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5258 = (t5257).items()?;
if t5258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5258[0].clone();
v_t = t5258[1].clone();
out.push({ let mut out = vec![]; for t5259 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5260 = (t5259).items()?;
if t5260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5260[0].clone();
v_b = t5260[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) })?, &v_s.clone())?;
    v_e4 = py::truediv(&f__fro({ let mut out = vec![]; for t5261 in py::iter(&(py::zip(&[v_XA.clone(), f__T(v_XA.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5262 = (t5261).items()?;
if t5262.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5262[0].clone();
v_t = t5262[1].clone();
out.push({ let mut out = vec![]; for t5263 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5264 = (t5263).items()?;
if t5264.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5264[0].clone();
v_b = t5264[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) })?, &v_s.clone())?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone(), v_e3.clone(), v_e4.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_poisson1d(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_ext: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t4526 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_o.clone(), &Py::str("u"))?])).items()?;
    if t4526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t4526[0].clone();
    v_u = t4526[1].clone();
    v_n = py::len(&v_f.clone())?;
    if ({ let t4527 = py::len(&v_u.clone())?; Py::Bool({ let t4528 = v_n.clone(); if py::ne(&t4527, &t4528) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_h = py::truediv(&py::getitem(&v_i.clone(), &Py::str("L"))?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_ext = py::add(&py::add(&Py::list(vec![Py::Float(0.0)]), &Py::list((v_u.clone()).items()?))?, &Py::list(vec![Py::Float(0.0)]))?;
    v_res = { let mut out = vec![]; for t4529 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t4529;
out.push(py::sub(&py::sub(&py::sub(&py::mul(&Py::Int(2i64), &py::getitem(&v_ext.clone(), &v_k.clone())?)?, &py::getitem(&v_ext.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?, &py::getitem(&v_ext.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::mul(&v_h.clone(), &v_h.clone())?, &py::getitem(&v_f.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?); } Py::list(out) };
    v_sc = py::minmax(&[Py::Float(1e-300), py::add(&py::mul(&Py::Int(4i64), &py::minmax(&[{ let mut out = vec![]; for t4530 in py::iter(&(v_ext.clone()))? { let mut v_x: Py; 
v_x = t4530;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?)?, &py::mul(&py::mul(&v_h.clone(), &v_h.clone())?, &py::minmax(&[{ let mut out = vec![]; for t4531 in py::iter(&(v_f.clone()))? { let mut v_x: Py; 
v_x = t4531;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?)?)?], true)?;
    return Ok(py::minmax(&[py::truediv(&py::minmax(&[{ let mut out = vec![]; for t4532 in py::iter(&(v_res.clone()))? { let mut v_x: Py; 
v_x = t4532;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?, &v_sc.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("h"))?, &v_h.clone())?)?, &v_h.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_poisson2d(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t4596 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_o.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("h"))?])).items()?;
    if t4596.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F = t4596[0].clone();
    v_U = t4596[1].clone();
    v_h = t4596[2].clone();
    let t4597 = (Py::tuple(vec![py::len(&v_F.clone())?, py::len(&py::getitem(&v_F.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4597[0].clone();
    v_m = t4597[1].clone();
    if ({ let t4598 = f__shape(v_U.clone())?; Py::Bool({ let t4599 = Py::tuple(vec![v_n.clone(), v_m.clone()]); if py::ne(&t4598, &t4599) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_g = { let v_F = v_F.clone(); let v_U = v_U.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_m = v_m.clone(); let v_n = v_n.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_a: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_b: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t4607 = { let t4601 = Py::Int(0i64); Py::Bool({ let t4602 = v_a.clone(); if py::le(&t4601, &t4602)? { { let t4603 = v_n.clone(); if py::lt(&t4602, &t4603)? { true } else { false } } } else { false } }) }; if !t4607.truthy() { t4607 } else { { let t4604 = Py::Int(0i64); Py::Bool({ let t4605 = v_b.clone(); if py::le(&t4604, &t4605)? { { let t4606 = v_m.clone(); if py::lt(&t4605, &t4606)? { true } else { false } } } else { false } }) } } }).truthy() { py::getitem(&py::getitem(&v_U.clone(), &v_a.clone())?, &v_b.clone())? } else { Py::Float(0.0) }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_res = py::minmax(&[{ let mut out = vec![]; for t4608 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_a: Py; 
v_a = t4608;
for t4609 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_b: Py; 
v_b = t4609;
out.push(py::abs(&py::sub(&py::sub(&py::sub(&py::sub(&py::sub(&py::mul(&Py::Int(4i64), &py::getitem(&py::getitem(&v_U.clone(), &v_a.clone())?, &v_b.clone())?)?, &py::call(&v_g.clone(), &[py::sub(&v_a.clone(), &Py::Int(1i64))?, v_b.clone()])?)?, &py::call(&v_g.clone(), &[py::add(&v_a.clone(), &Py::Int(1i64))?, v_b.clone()])?)?, &py::call(&v_g.clone(), &[v_a.clone(), py::sub(&v_b.clone(), &Py::Int(1i64))?])?)?, &py::call(&v_g.clone(), &[v_a.clone(), py::add(&v_b.clone(), &Py::Int(1i64))?])?)?, &py::mul(&py::mul(&v_h.clone(), &v_h.clone())?, &py::getitem(&py::getitem(&v_F.clone(), &v_a.clone())?, &v_b.clone())?)?)?)?); }} Py::list(out) }], true)?;
    v_sc = py::minmax(&[Py::Float(1e-300), py::add(&py::mul(&Py::Int(8i64), &py::minmax(&[{ let mut out = vec![]; for t4610 in py::iter(&(v_U.clone()))? { let mut v_r: Py; 
v_r = t4610;
for t4611 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4611;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?)?, &py::mul(&py::mul(&v_h.clone(), &v_h.clone())?, &py::minmax(&[{ let mut out = vec![]; for t4612 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t4612;
for t4613 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t4613;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?)?)?], true)?;
    return Ok(py::truediv(&v_res.clone(), &v_sc.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_polar(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t4713 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("U"))?, py::getitem(&v_o.clone(), &Py::str("P"))?])).items()?;
    if t4713.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4713[0].clone();
    v_U = t4713[1].clone();
    v_P = t4713[2].clone();
    v_n = py::len(&v_A.clone())?;
    if ({ let t4724 = { let t4714 = f__shape(v_U.clone())?; Py::Bool({ let t4715 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t4714, &t4715) { true } else { false } }) }; if t4724.truthy() { t4724 } else { { let t4723 = { let t4717 = f__shape(v_P.clone())?; Py::Bool({ let t4718 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t4717, &t4718) { true } else { false } }) }; if t4723.truthy() { t4723 } else { Py::Bool(({ let mut out = vec![]; for t4719 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t4719;
for t4720 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_c: Py; 
v_c = t4720;
out.push({ let t4721 = py::getitem(&py::getitem(&v_P.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t4722 = py::getitem(&py::getitem(&v_P.clone(), &v_c.clone())?, &v_r.clone())?; if py::ne(&t4721, &t4722) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_L = { let mut out = vec![]; for t4725 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t4725;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t4726 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t4726;
        v_s = py::sub(&py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t4727 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t4727;
out.push(py::pow(&py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t4728 = v_s.clone(); Py::Bool({ let t4729 = Py::Int(0i64); if py::le(&t4728, &t4729)? { true } else { false } }) }).truthy() {
            return Ok(c_BAD()?);
        }
        py::setitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &v_s.clone())?)?;
        for t4730 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t4730;
            py::setitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_j.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_P.clone(), &v_r.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t4731 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t4731;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?)?)?;
        }
    }
    v_e1 = py::minmax(&[{ let mut out = vec![]; for t4741 in py::iter(&(py::zip(&[f__mm(f__T(v_U.clone())?, v_U.clone())?, f__eye(v_n.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t4742 = (t4741).items()?;
if t4742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t4742[0].clone();
v_t = t4742[1].clone();
for t4743 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4744 = (t4743).items()?;
if t4744.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4744[0].clone();
v_b = t4744[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    v_e2 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t4745 in py::iter(&(py::zip(&[f__mm(v_U.clone(), v_P.clone())?, v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t4746 = (t4745).items()?;
if t4746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t4746[0].clone();
v_t = t4746[1].clone();
for t4747 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4748 = (t4747).items()?;
if t4748.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4748[0].clone();
v_b = t4748[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_polyfit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2511 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("y"))?, py::getitem(&v_o.clone(), &Py::str("coefficients"))?])).items()?;
    if t2511.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2511[0].clone();
    v_y = t2511[1].clone();
    v_c = t2511[2].clone();
    v_d = py::int(&py::getitem(&v_i.clone(), &Py::str("degree"))?)?;
    if ({ let t2512 = py::len(&v_c.clone())?; Py::Bool({ let t2513 = py::add(&v_d.clone(), &Py::Int(1i64))?; if py::ne(&t2512, &t2513) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_V = { let mut out = vec![]; for t2514 in py::iter(&(v_x.clone()))? { let mut v_t: Py; 
v_t = t2514;
out.push({ let mut out = vec![]; for t2515 in py::range_iter(&[(py::add(&v_d.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2515;
out.push(py::pow(&v_t.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
    v_r = { let mut out = vec![]; for t2516 in py::iter(&(py::zip(&[v_V.clone(), v_y.clone()])?))? { let mut v_b: Py; let mut v_row: Py; 
let t2517 = (t2516).items()?;
if t2517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t2517[0].clone();
v_b = t2517[1].clone();
out.push(py::sub(&v_b.clone(), &py::fsum(&{ let mut out = vec![]; for t2518 in py::iter(&(py::zip(&[v_row.clone(), v_c.clone()])?))? { let mut v_cc: Py; let mut v_v: Py; 
let t2519 = (t2518).items()?;
if t2519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_v = t2519[0].clone();
v_cc = t2519[1].clone();
out.push(py::mul(&v_v.clone(), &v_cc.clone())?); } Py::list(out) })?)?); } Py::list(out) };
    v_g = { let mut out = vec![]; for t2520 in py::range_iter(&[(py::add(&v_d.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2520;
out.push(py::fsum(&{ let mut out = vec![]; for t2521 in py::iter(&(py::zip(&[v_V.clone(), v_r.clone()])?))? { let mut v_e: Py; let mut v_row: Py; 
let t2522 = (t2521).items()?;
if t2522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t2522[0].clone();
v_e = t2522[1].clone();
out.push(py::mul(&py::getitem(&v_row.clone(), &v_k.clone())?, &v_e.clone())?); } Py::list(out) })?); } Py::list(out) };
    v_nv = f__fro(v_V.clone())?;
    return Ok(py::truediv(&f__norm(v_g.clone())?, &py::minmax(&[Py::Float(1e-300), py::mul(&v_nv.clone(), &py::add(&f__norm(v_y.clone())?, &py::mul(&v_nv.clone(), &f__norm(v_c.clone())?)?)?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_power(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t3522 = (Py::tuple(vec![f__FM(py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::int(&py::getitem(&v_i.clone(), &Py::str("k"))?)?])).items()?;
    if t3522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3522[0].clone();
    v_k = t3522[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_R = { let mut out = vec![]; for t3523 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3523;
out.push({ let mut out = vec![]; for t3524 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t3524;
out.push(py::fraction(&[py::int(&{ let t3525 = v_r.clone(); Py::Bool({ let t3526 = v_c.clone(); if py::eq(&t3525, &t3526) { true } else { false } }) })?])?); } Py::list(out) }); } Py::list(out) };
    for t3527 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t3527;
        v_R = f__mm_F(v_R.clone(), v_A.clone())?;
    }
    return Ok(f__diff(py::getitem(&v_o.clone(), &Py::str("power"))?, f__fl(v_R.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_projection(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_nv: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t6671 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?, py::getitem(&v_o.clone(), &Py::str("projection"))?])).items()?;
    if t6671.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6671[0].clone();
    v_v = t6671[1].clone();
    v_p = t6671[2].clone();
    if ({ let t6672 = py::len(&v_p.clone())?; Py::Bool({ let t6673 = py::len(&v_v.clone())?; if py::ne(&t6672, &t6673) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_nv = f__norm(v_v.clone())?;
    v_t = py::truediv(&f__dot(v_p.clone(), v_v.clone())?, &py::mul(&v_nv.clone(), &v_nv.clone())?)?;
    v_s = py::minmax(&[Py::Float(1.0), f__norm(v_u.clone())?], true)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&f__dot({ let mut out = vec![]; for t6677 in py::iter(&(py::zip(&[v_u.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6678 = (t6677).items()?;
if t6678.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6678[0].clone();
v_b = t6678[1].clone();
out.push(py::sub(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, v_v.clone())?)?, &py::mul(&v_s.clone(), &v_nv.clone())?)?, py::truediv(&py::minmax(&[{ let mut out = vec![]; for t6679 in py::iter(&(py::zip(&[v_p.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6680 = (t6679).items()?;
if t6680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6680[0].clone();
v_b = t6680[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &py::mul(&v_t.clone(), &v_b.clone())?)?)?); } Py::list(out) }], true)?, &v_s.clone())?, py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("scalar_projection"))?, &py::truediv(&f__dot(v_p.clone(), v_v.clone())?, &v_nv.clone())?)?)?, &v_s.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_projmat(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_PA: Py = Py::None;
    let mut v_PP: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_e3: Py = Py::None;
    let mut v_e4: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t5141 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("projection"))?])).items()?;
    if t5141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5141[0].clone();
    v_P = t5141[1].clone();
    v_m = py::len(&v_A.clone())?;
    if ({ let t5142 = f__shape(v_P.clone())?; Py::Bool({ let t5143 = Py::tuple(vec![v_m.clone(), v_m.clone()]); if py::ne(&t5142, &t5143) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    let t5149 = (Py::tuple(vec![f__mm(v_P.clone(), v_P.clone())?, f__mm(v_P.clone(), v_A.clone())?])).items()?;
    if t5149.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_PP = t5149[0].clone();
    v_PA = t5149[1].clone();
    v_e1 = py::minmax(&[{ let mut out = vec![]; for t5150 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_r: Py; 
v_r = t5150;
for t5151 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_c: Py; 
v_c = t5151;
out.push(py::abs(&py::sub(&py::getitem(&py::getitem(&v_P.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_c.clone())?, &v_r.clone())?)?)?); }} Py::list(out) }], true)?;
    v_e2 = py::minmax(&[{ let mut out = vec![]; for t5152 in py::iter(&(py::zip(&[v_PP.clone(), v_P.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5153 = (t5152).items()?;
if t5153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5153[0].clone();
v_t = t5153[1].clone();
for t5154 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5155 = (t5154).items()?;
if t5155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5155[0].clone();
v_b = t5155[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    v_e3 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t5156 in py::iter(&(py::zip(&[v_PA.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5157 = (t5156).items()?;
if t5157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5157[0].clone();
v_t = t5157[1].clone();
for t5158 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5159 = (t5158).items()?;
if t5159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5159[0].clone();
v_b = t5159[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?;
    v_e4 = py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t5162 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_k: Py; 
v_k = t5162;
out.push(py::getitem(&py::getitem(&v_P.clone(), &v_k.clone())?, &v_k.clone())?); } Py::list(out) })?, &py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?)?)?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone(), v_e3.clone(), v_e4.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_qr(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_QR: Py = Py::None;
    let mut v_QtQ: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t5362 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("Q"))?, py::getitem(&v_o.clone(), &Py::str("R"))?])).items()?;
    if t5362.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5362[0].clone();
    v_Q = t5362[1].clone();
    v_R = t5362[2].clone();
    let t5363 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t5363.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5363[0].clone();
    v_n = t5363[1].clone();
    if ({ let t5369 = { let t5364 = f__shape(v_Q.clone())?; Py::Bool({ let t5365 = Py::tuple(vec![v_m.clone(), v_n.clone()]); if py::ne(&t5364, &t5365) { true } else { false } }) }; if t5369.truthy() { t5369 } else { { let t5367 = f__shape(v_R.clone())?; Py::Bool({ let t5368 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t5367, &t5368) { true } else { false } }) } } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t5377 = Py::Bool(({ let mut out = vec![]; for t5370 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5370;
for t5371 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_c: Py; 
v_c = t5371;
out.push({ let t5372 = py::getitem(&py::getitem(&v_R.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t5373 = Py::Int(0i64); if py::ne(&t5372, &t5373) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t5377.truthy() { t5377 } else { Py::Bool(({ let mut out = vec![]; for t5374 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t5374;
out.push({ let t5375 = py::getitem(&py::getitem(&v_R.clone(), &v_r.clone())?, &v_r.clone())?; Py::Bool({ let t5376 = Py::Int(0i64); if py::le(&t5375, &t5376)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_QtQ = f__mm(f__T(v_Q.clone())?, v_Q.clone())?;
    v_e1 = py::minmax(&[{ let mut out = vec![]; for t5387 in py::iter(&(py::zip(&[v_QtQ.clone(), f__eye(v_n.clone())?])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5388 = (t5387).items()?;
if t5388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5388[0].clone();
v_t = t5388[1].clone();
for t5389 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5390 = (t5389).items()?;
if t5390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5390[0].clone();
v_b = t5390[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?;
    v_QR = f__mm(v_Q.clone(), v_R.clone())?;
    v_e2 = py::truediv(&py::minmax(&[{ let mut out = vec![]; for t5391 in py::iter(&(py::zip(&[v_QR.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t5392 = (t5391).items()?;
if t5392.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5392[0].clone();
v_t = t5392[1].clone();
for t5393 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5394 = (t5393).items()?;
if t5394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5394[0].clone();
v_b = t5394[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?;
    return Ok(py::minmax(&[v_e1.clone(), v_e2.clone()], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_rotation(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t5421 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("v"))?, py::getitem(&v_o.clone(), &Py::str("rotated"))?])).items()?;
    if t5421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t5421[0].clone();
    v_w = t5421[1].clone();
    if ({ let t5422 = py::len(&v_w.clone())?; Py::Bool({ let t5423 = Py::Int(2i64); if py::ne(&t5422, &t5423) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    let t5424 = (f__cos_sin_deg(py::getitem(&v_i.clone(), &Py::str("angle"))?)?).items()?;
    if t5424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5424[0].clone();
    v_s = t5424[1].clone();
    v_n2 = py::minmax(&[Py::Float(1.0), f__dot(v_v.clone(), v_v.clone())?], true)?;
    return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&f__dot(v_w.clone(), v_w.clone())?, &f__dot(v_v.clone(), v_v.clone())?)?)?, &v_n2.clone())?, py::truediv(&py::abs(&py::sub(&f__dot(v_v.clone(), v_w.clone())?, &py::mul(&f__dot(v_v.clone(), v_v.clone())?, &v_c.clone())?)?)?, &v_n2.clone())?, py::truediv(&py::abs(&py::sub(&py::sub(&py::mul(&py::getitem(&v_v.clone(), &Py::Int(0i64))?, &py::getitem(&v_w.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_v.clone(), &Py::Int(1i64))?, &py::getitem(&v_w.clone(), &Py::Int(0i64))?)?)?, &py::mul(&f__dot(v_v.clone(), v_v.clone())?, &v_s.clone())?)?)?, &v_n2.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_singular(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let t5793 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("singular_values"))?])).items()?;
    if t5793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5793[0].clone();
    v_s = t5793[1].clone();
    v_n = py::minmax(&[py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?], false)?;
    if ({ let t5803 = { let t5794 = py::len(&v_s.clone())?; Py::Bool({ let t5795 = v_n.clone(); if py::ne(&t5794, &t5795) { true } else { false } }) }; if t5803.truthy() { t5803 } else { { let t5802 = Py::Bool(({ let mut out = vec![]; for t5796 in py::iter(&(v_s.clone()))? { let mut v_x: Py; 
v_x = t5796;
out.push({ let t5797 = v_x.clone(); Py::Bool({ let t5798 = Py::Int(0i64); if py::lt(&t5797, &t5798)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t5802.truthy() { t5802 } else { Py::Bool(({ let mut out = vec![]; for t5799 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t5799;
out.push({ let t5800 = py::getitem(&v_s.clone(), &v_k.clone())?; Py::Bool({ let t5801 = py::getitem(&v_s.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::lt(&t5800, &t5801)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_p = f__gram_power_sums(v_A.clone(), v_n.clone())?;
    v_sc = py::pow(&py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?, &Py::Int(2i64))?;
    return Ok(py::minmax(&[{ let mut out = vec![]; for t5818 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t5818;
out.push(py::truediv(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t5819 in py::iter(&(v_s.clone()))? { let mut v_x: Py; 
v_x = t5819;
out.push(py::pow(&v_x.clone(), &py::mul(&Py::Int(2i64), &v_k.clone())?)?); } Py::list(out) })?, &py::getitem(&v_p.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?, &py::pow(&v_sc.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_solution(mut v_i: Py, mut v_o: Py, mut v_key: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t6478 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_o.clone(), &v_key.clone())?])).items()?;
    if t6478.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t6478[0].clone();
    v_b = t6478[1].clone();
    v_x = t6478[2].clone();
    if ({ let t6479 = py::len(&v_x.clone())?; Py::Bool({ let t6480 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::ne(&t6479, &t6480) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_r = { let mut out = vec![]; for t6484 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_x.clone())?, v_b.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t6485 = (t6484).items()?;
if t6485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6485[0].clone();
v_c = t6485[1].clone();
out.push(py::sub(&v_a.clone(), &v_c.clone())?); } Py::list(out) };
    return Ok(py::truediv(&f__norm(v_r.clone())?, &py::minmax(&[Py::Float(1e-300), py::add(&py::mul(&f__fro(v_A.clone())?, &f__norm(v_x.clone())?)?, &f__norm(v_b.clone())?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_spectrum(mut v_A: Py, mut v_vals: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_sc: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t6367 = py::len(&v_vals.clone())?; Py::Bool({ let t6368 = v_n.clone(); if py::ne(&t6367, &t6368) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_p = f__power_sums_exact(v_A.clone(), v_n.clone())?;
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    return Ok(py::minmax(&[{ let mut out = vec![]; for t6379 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6379;
out.push(py::truediv(&py::abs(&py::sub(&py::sum(&{ let mut out = vec![]; for t6380 in py::iter(&(v_vals.clone()))? { let mut v_x: Py; 
v_x = t6380;
out.push(py::pow(&py::complex(&[v_x.clone()])?, &v_k.clone())?); } Py::list(out) }, None)?, &py::getitem(&v_p.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?, &py::pow(&v_sc.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_sqrtm(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_XX: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3711 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("sqrt"))?])).items()?;
    if t3711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3711[0].clone();
    v_X = t3711[1].clone();
    v_n = py::len(&v_A.clone())?;
    if ({ let t3719 = { let t3712 = f__shape(v_X.clone())?; Py::Bool({ let t3713 = Py::tuple(vec![v_n.clone(), v_n.clone()]); if py::ne(&t3712, &t3713) { true } else { false } }) }; if t3719.truthy() { t3719 } else { Py::Bool(({ let mut out = vec![]; for t3715 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t3715;
for t3716 in py::range_iter(&[(v_r.clone()).i()?]) { let mut v_c: Py; 
v_c = t3716;
out.push({ let t3717 = py::getitem(&py::getitem(&v_X.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t3718 = py::getitem(&py::getitem(&v_X.clone(), &v_c.clone())?, &v_r.clone())?; if py::ne(&t3717, &t3718) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_L = { let mut out = vec![]; for t3720 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3720;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t3721 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t3721;
        v_s = py::sub(&py::getitem(&py::getitem(&v_X.clone(), &v_j.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t3722 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t3722;
out.push(py::pow(&py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        if ({ let t3723 = v_s.clone(); Py::Bool({ let t3724 = Py::Int(0i64); if py::le(&t3723, &t3724)? { true } else { false } }) }).truthy() {
            return Ok(c_BAD()?);
        }
        py::setitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &v_s.clone())?)?;
        for t3725 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t3725;
            py::setitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_j.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_X.clone(), &v_r.clone())?, &v_j.clone())?, &py::fsum(&{ let mut out = vec![]; for t3726 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t3726;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?)?)?;
        }
    }
    v_XX = f__mm(v_X.clone(), v_X.clone())?;
    return Ok(py::truediv(&py::minmax(&[{ let mut out = vec![]; for t3731 in py::iter(&(py::zip(&[v_XX.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t3732 = (t3731).items()?;
if t3732.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3732[0].clone();
v_t = t3732[1].clone();
for t3733 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3734 = (t3733).items()?;
if t3734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3734[0].clone();
v_b = t3734[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_tridiag(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t6467 = (Py::tuple(vec![py::len(&py::getitem(&v_i.clone(), &Py::str("diag"))?)?, py::getitem(&v_o.clone(), &Py::str("solution"))?])).items()?;
    if t6467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t6467[0].clone();
    v_x = t6467[1].clone();
    if ({ let t6468 = py::len(&v_x.clone())?; Py::Bool({ let t6469 = v_n.clone(); if py::ne(&t6468, &t6469) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_A = { let mut out = vec![]; for t6470 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_r: Py; 
v_r = t6470;
out.push({ let mut out = vec![]; for t6471 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t6471;
out.push((if ({ let t6472 = v_c.clone(); Py::Bool({ let t6473 = v_r.clone(); if py::eq(&t6472, &t6473) { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_i.clone(), &Py::str("diag"))?, &v_r.clone())? } else { (if ({ let t6474 = v_c.clone(); Py::Bool({ let t6475 = py::sub(&v_r.clone(), &Py::Int(1i64))?; if py::eq(&t6474, &t6475) { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_i.clone(), &Py::str("sub"))?, &v_c.clone())? } else { (if ({ let t6476 = v_c.clone(); Py::Bool({ let t6477 = py::add(&v_r.clone(), &Py::Int(1i64))?; if py::eq(&t6476, &t6477) { true } else { false } }) }).truthy() { py::getitem(&py::getitem(&v_i.clone(), &Py::str("sup"))?, &v_r.clone())? } else { Py::Float(0.0) }) }) })); } Py::list(out) }); } Py::list(out) };
    return Ok(f__verify_solution(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("b"), py::getitem(&v_i.clone(), &Py::str("d"))?)]), v_o.clone(), Py::str("solution"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_unit(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t6571 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("unit"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])).items()?;
    if t6571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6571[0].clone();
    v_v = t6571[1].clone();
    if ({ let t6572 = py::len(&v_u.clone())?; Py::Bool({ let t6573 = py::len(&v_v.clone())?; if py::ne(&t6572, &t6573) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    v_n = f__norm(v_v.clone())?;
    return Ok(py::minmax(&[py::abs(&py::sub(&f__norm(v_u.clone())?, &Py::Float(1.0))?)?, py::minmax(&[{ let mut out = vec![]; for t6575 in py::iter(&(py::zip(&[v_u.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6576 = (t6575).items()?;
if t6576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6576[0].clone();
v_b = t6576[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&py::mul(&v_a.clone(), &v_n.clone())?, &v_b.clone())?)?, &py::minmax(&[Py::Float(1.0), v_n.clone()], true)?)?); } Py::list(out) }], true)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__well_conditioned_root(mut v_c: Py, mut v_x: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_delta: Py = Py::None;
    v_d = py::abs(&f__peval(f__pder(v_c.clone())?, v_x.clone())?)?;
    v_delta = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?;
    return Ok({ let t5633 = py::mul(&v_d.clone(), &v_delta.clone())?; Py::Bool({ let t5634 = py::mul(&py::mul(&Py::Float(1000.0), &Py::Float(2.2e-16))?, &py::minmax(&[f__pscale(v_c.clone(), v_x.clone())?, Py::Float(1e-300)], true)?)?; if py::gt(&t5633, &t5634)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_adjugate(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, Py::Int(7i64))?;
    let t34 = (Py::tuple(vec![f__FM(v_A.clone())?, py::len(&v_A.clone())?])).items()?;
    if t34.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t34[0].clone();
    v_n = t34[1].clone();
    if ({ let t35 = v_n.clone(); Py::Bool({ let t36 = Py::Int(1i64); if py::eq(&t35, &t36) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("adjugate"), Py::list(vec![Py::list(vec![Py::Float(1.0)])]))]));
    }
    return Ok(Py::dict(vec![(Py::str("adjugate"), { let mut out = vec![]; for t37 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t37;
out.push({ let mut out = vec![]; for t38 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t38;
out.push(py::float(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &py::add(&v_i.clone(), &v_j.clone())?)?, &f__minor_F(v_F.clone(), v_j.clone(), v_i.clone())?)?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_angle_between_vectors(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_nu: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let t92 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t92.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t92[0].clone();
    v_v = t92[1].clone();
    if ({ let t93 = py::len(&v_u.clone())?; Py::Bool({ let t94 = py::len(&v_v.clone())?; if py::ne(&t93, &t94) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries"))));
    }
    let t96 = (Py::tuple(vec![f__norm(v_u.clone())?, f__norm(v_v.clone())?])).items()?;
    if t96.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nu = t96[0].clone();
    v_nv = t96[1].clone();
    if ({ let t101 = { let t97 = v_nu.clone(); Py::Bool({ let t98 = Py::Int(0i64); if py::eq(&t97, &t98) { true } else { false } }) }; if t101.truthy() { t101 } else { { let t99 = v_nv.clone(); Py::Bool({ let t100 = Py::Int(0i64); if py::eq(&t99, &t100) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the angle with the zero vector is undefined"))));
    }
    let t104 = (Py::tuple(vec![{ let mut out = vec![]; for t102 in py::iter(&(v_u.clone()))? { let mut v_x: Py; 
v_x = t102;
out.push(py::truediv(&v_x.clone(), &v_nu.clone())?); } Py::list(out) }, { let mut out = vec![]; for t103 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t103;
out.push(py::truediv(&v_x.clone(), &v_nv.clone())?); } Py::list(out) }])).items()?;
    if t104.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t104[0].clone();
    v_b = t104[1].clone();
    v_rad = py::mul(&Py::Int(2i64), &py::atan2(&f__norm({ let mut out = vec![]; for t105 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t106 = (t105).items()?;
if t106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t106[0].clone();
v_y = t106[1].clone();
out.push(py::sub(&v_x.clone(), &v_y.clone())?); } Py::list(out) })?, &f__norm({ let mut out = vec![]; for t107 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t108 = (t107).items()?;
if t108.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t108[0].clone();
v_y = t108[1].clone();
out.push(py::add(&v_x.clone(), &v_y.clone())?); } Py::list(out) })?)?)?;
    return Ok(Py::dict(vec![(Py::str("angle_degrees"), py::math1("degrees", &v_rad.clone())?), (Py::str("angle_radians"), v_rad.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bisection_root(mut v_coefficients: Py, mut v_a: Py, mut v_b: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    let _ = Py::tuple(vec![f__finite(v_a.clone(), Py::str("a"))?, f__finite(v_b.clone(), Py::str("b"))?]);
    if (py::not(&{ let t146 = v_a.clone(); Py::Bool({ let t147 = v_b.clone(); if py::lt(&t146, &t147)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval must have a < b"))));
    }
    if ({ let t148 = py::len(&v_c.clone())?; Py::Bool({ let t149 = Py::Int(2i64); if py::lt(&t148, &t149)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a constant polynomial has no isolated root"))));
    }
    let t151 = (Py::tuple(vec![f__peval(v_c.clone(), v_a.clone())?, f__peval(v_c.clone(), v_b.clone())?])).items()?;
    if t151.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t151[0].clone();
    v_fb = t151[1].clone();
    if ({ let t156 = { let t152 = v_fa.clone(); Py::Bool({ let t153 = Py::Int(0i64); if py::eq(&t152, &t153) { true } else { false } }) }; if t156.truthy() { t156 } else { { let t154 = v_fb.clone(); Py::Bool({ let t155 = Py::Int(0i64); if py::eq(&t154, &t155) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("an end of the interval is itself a root: give an interval with the root strictly inside"))));
    }
    if ({ let t157 = { let t159 = v_fa.clone(); Py::Bool({ let t160 = Py::Int(0i64); if py::gt(&t159, &t160)? { true } else { false } }) }; Py::Bool({ let t158 = { let t161 = v_fb.clone(); Py::Bool({ let t162 = Py::Int(0i64); if py::gt(&t161, &t162)? { true } else { false } }) }; if py::eq(&t157, &t158) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("p(a) and p(b) must have opposite signs"))));
    }
    if ({ let t163 = f__sturm_count(v_c.clone(), v_a.clone(), v_b.clone())?; Py::Bool({ let t164 = Py::Int(1i64); if py::ne(&t163, &t164) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval must contain exactly one root"))));
    }
    let t194 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
    if t194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t194[0].clone();
    v_hi = t194[1].clone();
    for t195 in py::range_iter(&[(Py::Int(3000i64)).i()?]) {
        v__ = t195;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t200 = { let t196 = v_mid.clone(); Py::Bool({ let t197 = v_lo.clone(); if py::le(&t196, &t197)? { true } else { false } }) }; if t200.truthy() { t200 } else { { let t198 = v_mid.clone(); Py::Bool({ let t199 = v_hi.clone(); if py::ge(&t198, &t199)? { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = f__peval(v_c.clone(), v_mid.clone())?;
        if ({ let t201 = v_fm.clone(); Py::Bool({ let t202 = Py::Int(0i64); if py::eq(&t201, &t202) { true } else { false } }) }).truthy() {
            let t203 = v_mid.clone();
            v_lo = t203.clone();
            v_hi = t203.clone();
            break;
        }
        if ({ let t204 = { let t206 = v_fm.clone(); Py::Bool({ let t207 = Py::Int(0i64); if py::gt(&t206, &t207)? { true } else { false } }) }; Py::Bool({ let t205 = { let t208 = v_fa.clone(); Py::Bool({ let t209 = Py::Int(0i64); if py::gt(&t208, &t209)? { true } else { false } }) }; if py::eq(&t204, &t205) { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    v_x = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    if (py::not(&f__well_conditioned_root(v_c.clone(), v_x.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the root is too ill-conditioned (a multiple or nearly multiple root)"))));
    }
    return Ok(Py::dict(vec![(Py::str("root"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_change_of_basis(mut v_basis: Py, mut v_v: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_c: Py = Py::None;
    v_B = f__need_mat(v_basis.clone(), Py::str("basis"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_v = f__need_vec(v_v.clone(), Py::str("v"), py::len(&v_B.clone())?, Py::Int(1i64), Py::None)?;
    let t312: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_c = f__solve_F(f__T(f__FM(v_B.clone())?)?, f__FV(v_v.clone())?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t312 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Err(py::value_error(py::to_str(&Py::str("the basis vectors are linearly dependent, so they are not a basis"))));
        }
        Err(e) => return Err(e),
    }
    return Ok(Py::dict(vec![(Py::str("coordinates"), f__fl(v_c.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_characteristic_polynomial(mut v_A: Py) -> R {
    let mut v_AM: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    let t362 = (Py::tuple(vec![f__FM(v_A.clone())?, py::len(&v_A.clone())?])).items()?;
    if t362.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t362[0].clone();
    v_n = t362[1].clone();
    v_M = { let mut out = vec![]; for t363 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t363;
out.push(py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?); } Py::list(out) };
    v_c = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    v_I = { let mut out = vec![]; for t364 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t364;
out.push({ let mut out = vec![]; for t365 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t365;
out.push(py::fraction(&[py::int(&{ let t366 = v_i.clone(); Py::Bool({ let t367 = v_j.clone(); if py::eq(&t366, &t367) { true } else { false } }) })?])?); } Py::list(out) }); } Py::list(out) };
    for t368 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t368;
        v_M = (if ({ let t369 = v_k.clone(); Py::Bool({ let t370 = Py::Int(1i64); if py::gt(&t369, &t370)? { true } else { false } }) }).truthy() { { let mut out = vec![]; for t375 in py::iter(&(py::zip(&[f__mm_F(v_F.clone(), v_M.clone())?, v_I.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t376 = (t375).items()?;
if t376.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t376[0].clone();
v_s = t376[1].clone();
out.push({ let mut out = vec![]; for t377 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t378 = (t377).items()?;
if t378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t378[0].clone();
v_b = t378[1].clone();
out.push(py::add(&v_a.clone(), &py::mul(&py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?, &v_b.clone())?)?); } Py::list(out) }); } Py::list(out) } } else { { let mut out = vec![]; for t379 in py::iter(&(v_I.clone()))? { let mut v_r: Py; 
v_r = t379;
out.push(py::slice(&v_r.clone(), None, None, None)?); } Py::list(out) } });
        v_AM = f__mm_F(v_F.clone(), v_M.clone())?;
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t380 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t380;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?, &v_k.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("coefficients"), { let mut out = vec![]; for t381 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t381;
out.push(py::float(&v_x.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cholesky(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_L: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_A = f__need_symmetric(v_A.clone(), Py::str("A"))?;
    v_n = py::len(&v_A.clone())?;
    v_F = f__FM(v_A.clone())?;
    for t445 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t445;
        if ({ let t446 = f__det_F({ let mut out = vec![]; for t460 in py::iter(&(py::slice(&v_F.clone(), None, Some((v_k.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t460;
out.push(py::slice(&v_r.clone(), None, Some((v_k.clone()).i()?), None)?); } Py::list(out) })?; Py::Bool({ let t447 = Py::Int(0i64); if py::le(&t446, &t447)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("A is not positive definite (a leading principal minor is not positive)"))));
        }
    }
    v_L = { let mut out = vec![]; for t461 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t461;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t462 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_j = t462;
        v_s = py::sub(&py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_j.clone())?, &v_j.clone())?)?, &py::fsum(&{ let mut out = vec![]; for t463 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t463;
out.push(py::pow(&py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?;
        py::setitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone(), py::math1("sqrt", &v_s.clone())?)?;
        for t464 in py::range_iter(&[(py::add(&v_j.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t464;
            py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_j.clone(), py::truediv(&py::sub(&py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::fsum(&{ let mut out = vec![]; for t465 in py::range_iter(&[(v_j.clone()).i()?]) { let mut v_k: Py; 
v_k = t465;
out.push(py::mul(&py::getitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_L.clone(), &v_j.clone())?, &v_j.clone())?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("L"), v_L.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_clamped_cubic_spline(mut v_x: Py, mut v_y: Py, mut v_slope_start: Py, mut v_slope_end: Py, mut v_at: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t523 = (f__table(v_x.clone(), v_y.clone(), Py::Int(2i64), Py::Bool(true), Py::Bool(true))?).items()?;
    if t523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t523[0].clone();
    v_Y = t523[1].clone();
    v_t = f__F(v_at.clone(), Py::str("at"))?;
    if (py::not(&{ let t524 = py::getitem(&v_X.clone(), &Py::Int(0i64))?; Py::Bool({ let t525 = v_t.clone(); if py::le(&t524, &t525)? { { let t526 = py::getitem(&v_X.clone(), &py::neg(&Py::Int(1i64))?)?; if py::le(&t525, &t526)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point must lie within the range of the x values (no extrapolation)"))));
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&f__spline_value(v_X.clone(), v_Y.clone(), v_t.clone(), f__F(v_slope_start.clone(), Py::str("slope_start"))?, f__F(v_slope_end.clone(), Py::str("slope_end"))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_column_space(mut v_A: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_piv: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    let t640 = (f__rref_F(f__FM(v_A.clone())?)?).items()?;
    if t640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t640[0].clone();
    v_piv = t640[1].clone();
    return Ok(Py::dict(vec![(Py::str("dimension"), py::float(&py::len(&v_piv.clone())?)?), (Py::str("basis"), { let mut out = vec![]; for t641 in py::iter(&(v_piv.clone()))? { let mut v_c: Py; 
v_c = t641;
out.push({ let mut out = vec![]; for t642 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_r: Py; 
v_r = t642;
out.push(py::float(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_condition_number(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_inv: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    if ({ let t699 = f__det_F(v_F.clone())?; Py::Bool({ let t700 = Py::Int(0i64); if py::eq(&t699, &t700) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the matrix is singular: its condition number is infinite"))));
    }
    v_s = py::getitem(&f__svd_jacobi(v_A.clone())?, &Py::Int(1i64))?;
    v_inv = f__inv_F(v_F.clone())?;
    v_n1 = { let v_A = v_A.clone(); let v_F = v_F.clone(); let v_inv = v_inv.clone(); let v_s = v_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_M: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[{ let mut out = vec![]; for t779 in py::range_iter(&[(py::len(&v_M.clone())?).i()?]) { let mut v_j: Py; 
v_j = t779;
out.push(py::sum(&{ let mut out = vec![]; for t780 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t780;
out.push(py::abs(&py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("cond2"), py::truediv(&py::getitem(&v_s.clone(), &Py::Int(0i64))?, &py::getitem(&v_s.clone(), &py::neg(&Py::Int(1i64))?)?)?), (Py::str("cond1"), py::float(&py::mul(&py::call(&v_n1.clone(), &[v_F.clone()])?, &py::call(&v_n1.clone(), &[v_inv.clone()])?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_conjugate_gradient(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_Ap: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let mut v_rr2: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    if (py::not(&f__spd(v_A.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be symmetric positive definite"))));
    }
    v_n = py::len(&v_A.clone())?;
    v_A = { let mut out = vec![]; for t859 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t859;
out.push({ let mut out = vec![]; for t860 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t860;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    v_nb = f__norm(v_b.clone())?;
    if ({ let t862 = v_nb.clone(); Py::Bool({ let t863 = Py::Int(0i64); if py::eq(&t862, &t863) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("solution"), v_x.clone())]));
    }
    for t864 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
        v__ = t864;
        v_r = { let mut out = vec![]; for t868 in py::iter(&(py::zip(&[v_b.clone(), f__mv(v_A.clone(), v_x.clone())?])?))? { let mut v_bi: Py; let mut v_v: Py; 
let t869 = (t868).items()?;
if t869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bi = t869[0].clone();
v_v = t869[1].clone();
out.push(py::sub(&v_bi.clone(), &v_v.clone())?); } Py::list(out) };
        v_p = Py::list((v_r.clone()).items()?);
        v_rr = f__dot(v_r.clone(), v_r.clone())?;
        for t872 in py::range_iter(&[(py::mul(&Py::Int(4i64), &v_n.clone())?).i()?]) {
            v__ = t872;
            if ({ let t873 = py::math1("sqrt", &v_rr.clone())?; Py::Bool({ let t874 = py::mul(&Py::Float(1e-17), &v_nb.clone())?; if py::le(&t873, &t874)? { true } else { false } }) }).truthy() {
                break;
            }
            v_Ap = f__mv(v_A.clone(), v_p.clone())?;
            v_alpha = py::truediv(&v_rr.clone(), &f__dot(v_p.clone(), v_Ap.clone())?)?;
            v_x = { let mut out = vec![]; for t875 in py::iter(&(py::zip(&[v_x.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t876 = (t875).items()?;
if t876.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t876[0].clone();
v_c = t876[1].clone();
out.push(py::add(&v_a.clone(), &py::mul(&v_alpha.clone(), &v_c.clone())?)?); } Py::list(out) };
            v_r = { let mut out = vec![]; for t877 in py::iter(&(py::zip(&[v_r.clone(), v_Ap.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t878 = (t877).items()?;
if t878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t878[0].clone();
v_c = t878[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_alpha.clone(), &v_c.clone())?)?); } Py::list(out) };
            v_rr2 = f__dot(v_r.clone(), v_r.clone())?;
            v_p = { let mut out = vec![]; for t879 in py::iter(&(py::zip(&[v_r.clone(), v_p.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t880 = (t879).items()?;
if t880.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t880[0].clone();
v_c = t880[1].clone();
out.push(py::add(&v_a.clone(), &py::mul(&py::truediv(&v_rr2.clone(), &v_rr.clone())?, &v_c.clone())?)?); } Py::list(out) };
            v_rr = v_rr2.clone();
        }
    }
    v_res = f__norm({ let mut out = vec![]; for t881 in py::iter(&(py::zip(&[v_b.clone(), f__mv(v_A.clone(), v_x.clone())?])?))? { let mut v_bi: Py; let mut v_v: Py; 
let t882 = (t881).items()?;
if t882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_bi = t882[0].clone();
v_v = t882[1].clone();
out.push(py::sub(&v_bi.clone(), &v_v.clone())?); } Py::list(out) })?;
    if ({ let t883 = v_res.clone(); Py::Bool({ let t884 = py::mul(&Py::Float(1e-12), &py::add(&py::mul(&f__fro(v_A.clone())?, &f__norm(v_x.clone())?)?, &v_nb.clone())?)?; if py::gt(&t883, &t884)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("conjugate gradients did not converge"))));
    }
    return Ok(Py::dict(vec![(Py::str("solution"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cross_product(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let t913 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::Int(3i64), Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::Int(3i64), Py::Int(1i64), Py::None)?])).items()?;
    if t913.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t913[0].clone();
    v_v = t913[1].clone();
    let t915 = (Py::tuple(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?])).items()?;
    if t915.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t915[0].clone();
    v_b = t915[1].clone();
    return Ok(Py::dict(vec![(Py::str("cross"), Py::list(vec![py::float(&py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?)?)?, py::float(&py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(2i64))?)?)?)?, py::float(&py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?)?)?]))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_csc_format(mut v_A: Py) -> R {
    return Ok(f__compress(v_A.clone(), Py::Bool(false))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_csr_format(mut v_A: Py) -> R {
    return Ok(f__compress(v_A.clone(), Py::Bool(true))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_csr_to_dense(mut v_data: Py, mut v_indices: Py, mut v_indptr: Py, mut v_ncols: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t6101 = (Py::tuple(vec![f__need_vec(v_data.clone(), Py::str("data"), Py::None, Py::Int(0i64), Py::None)?, f__need_vec(v_indices.clone(), Py::str("indices"), Py::None, Py::Int(0i64), Py::None)?, f__need_vec(v_indptr.clone(), Py::str("indptr"), Py::None, Py::Int(2i64), Py::None)?])).items()?;
    if t6101.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_data = t6101[0].clone();
    v_indices = t6101[1].clone();
    v_indptr = t6101[2].clone();
    if ({ let t6108 = Py::Bool(matches!(v_ncols.clone(), Py::Bool(_))); if t6108.truthy() { t6108 } else { { let t6107 = { let t6102 = v_ncols.clone(); Py::Bool({ let t6103 = py::int(&v_ncols.clone())?; if py::ne(&t6102, &t6103) { true } else { false } }) }; if t6107.truthy() { t6107 } else { py::not(&{ let t6104 = Py::Int(1i64); Py::Bool({ let t6105 = v_ncols.clone(); if py::le(&t6104, &t6105)? { { let t6106 = Py::Int(50i64); if py::le(&t6105, &t6106)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("ncols must be a whole number from 1 to 50"))));
    }
    if ({ let t6120 = { let t6109 = py::len(&v_indices.clone())?; Py::Bool({ let t6110 = py::len(&v_data.clone())?; if py::ne(&t6109, &t6110) { true } else { false } }) }; if t6120.truthy() { t6120 } else { { let t6119 = { let t6111 = py::getitem(&v_indptr.clone(), &Py::Int(0i64))?; Py::Bool({ let t6112 = Py::Int(0i64); if py::ne(&t6111, &t6112) { true } else { false } }) }; if t6119.truthy() { t6119 } else { { let t6118 = { let t6113 = py::getitem(&v_indptr.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t6114 = py::len(&v_data.clone())?; if py::ne(&t6113, &t6114) { true } else { false } }) }; if t6118.truthy() { t6118 } else { Py::Bool(({ let mut out = vec![]; for t6115 in py::range_iter(&[(py::sub(&py::len(&v_indptr.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6115;
out.push({ let t6116 = py::getitem(&v_indptr.clone(), &v_k.clone())?; Py::Bool({ let t6117 = py::getitem(&v_indptr.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t6116, &t6117)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("inconsistent CSR arrays: indptr must start at 0, never decrease and end at len(data) = len(indices)"))));
    }
    if ({ let t6131 = Py::Bool(({ let mut out = vec![]; for t6121 in py::iter(&(py::add(&Py::list((v_indices.clone()).items()?), &Py::list((v_indptr.clone()).items()?))?))? { let mut v_x: Py; 
v_x = t6121;
out.push({ let t6122 = v_x.clone(); Py::Bool({ let t6123 = py::int(&v_x.clone())?; if py::ne(&t6122, &t6123) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t6131.truthy() { t6131 } else { { let t6130 = Py::Bool(({ let mut out = vec![]; for t6124 in py::iter(&(v_indices.clone()))? { let mut v_j: Py; 
v_j = t6124;
out.push(py::not(&{ let t6125 = Py::Int(0i64); Py::Bool({ let t6126 = v_j.clone(); if py::le(&t6125, &t6126)? { { let t6127 = v_ncols.clone(); if py::lt(&t6126, &t6127)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t6130.truthy() { t6130 } else { { let t6128 = py::sub(&py::len(&v_indptr.clone())?, &Py::Int(1i64))?; Py::Bool({ let t6129 = Py::Int(50i64); if py::gt(&t6128, &t6129)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("every index must be a whole number from 0 to ncols - 1 (at most 50 rows)"))));
    }
    v_D = { let mut out = vec![]; for t6132 in py::iter(&(v_data.clone()))? { let mut v_x: Py; 
v_x = t6132;
out.push(f__F(v_x.clone(), Py::str("input"))?); } Py::list(out) };
    v_M = { let mut out = vec![]; for t6133 in py::range_iter(&[(py::sub(&py::len(&v_indptr.clone())?, &Py::Int(1i64))?).i()?]) { let mut v__: Py; 
v__ = t6133;
out.push(py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::int(&v_ncols.clone())?)?); } Py::list(out) };
    for t6134 in py::range_iter(&[(py::sub(&py::len(&v_indptr.clone())?, &Py::Int(1i64))?).i()?]) {
        v_r = t6134;
        for t6135 in py::range_iter(&[(py::int(&py::getitem(&v_indptr.clone(), &v_r.clone())?)?).i()?, (py::int(&py::getitem(&v_indptr.clone(), &py::add(&v_r.clone(), &Py::Int(1i64))?)?)?).i()?]) {
            v_k = t6135;
            let t6136 = py::getitem(&v_M.clone(), &v_r.clone())?; let t6137 = py::int(&py::getitem(&v_indices.clone(), &v_k.clone())?)?;
            py::setitem(&t6136, &t6137, py::iadd(&py::getitem(&t6136, &t6137)?, &py::getitem(&v_D.clone(), &v_k.clone())?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("dense"), f__fl(v_M.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_determinant(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    return Ok(Py::dict(vec![(Py::str("det"), py::float(&f__det_F(f__FM(v_A.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_determinant_2x2(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::Int(2i64), Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("det"), py::float(&py::sub(&py::mul(&py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_determinant_3x3(mut v_A: Py) -> R {
    let mut v_a: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::Int(3i64), Py::None, c_MAX_DIM()?)?;
    v_a = f__FM(v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("det"), py::float(&py::sub(&py::sub(&py::sub(&py::add(&py::add(&py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(2i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(0i64))?)?)?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?)?)?, &py::mul(&py::mul(&py::getitem(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?, &py::getitem(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &Py::Int(2i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_distance_between_points(mut v_p: Py, mut v_r: Py) -> R {
    let mut v_s: Py = Py::None;
    let t1338 = (Py::tuple(vec![f__need_vec(v_p.clone(), Py::str("p"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_r.clone(), Py::str("r"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t1338.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1338[0].clone();
    v_r = t1338[1].clone();
    if ({ let t1339 = py::len(&v_p.clone())?; Py::Bool({ let t1340 = py::len(&v_r.clone())?; if py::ne(&t1339, &t1340) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two points must have the same number of coordinates"))));
    }
    v_s = py::sum(&{ let mut out = vec![]; for t1342 in py::iter(&(py::zip(&[f__FV(v_p.clone())?, f__FV(v_r.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1343 = (t1342).items()?;
if t1343.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1343[0].clone();
v_b = t1343[1].clone();
out.push(py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?;
    return Ok(Py::dict(vec![(Py::str("distance"), py::math1("sqrt", &py::float(&v_s.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_divided_differences(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_col: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let t4207 = (f__table(v_x.clone(), v_y.clone(), Py::Int(1i64), Py::Bool(true), Py::Bool(false))?).items()?;
    if t4207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t4207[0].clone();
    v_Y = t4207[1].clone();
    let t4208 = (Py::tuple(vec![Py::list((v_Y.clone()).items()?), Py::list(vec![py::getitem(&v_Y.clone(), &Py::Int(0i64))?])])).items()?;
    if t4208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_col = t4208[0].clone();
    v_out = t4208[1].clone();
    for t4209 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_X.clone())?).i()?]) {
        v_k = t4209;
        v_col = { let mut out = vec![]; for t4210 in py::range_iter(&[(py::sub(&py::len(&v_col.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t4210;
out.push(py::truediv(&py::sub(&py::getitem(&v_col.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_col.clone(), &v_i.clone())?)?, &py::sub(&py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &v_k.clone())?)?, &py::getitem(&v_X.clone(), &v_i.clone())?)?)?); } Py::list(out) };
        let _ = py::method(&v_out.clone(), "append", &[py::getitem(&v_col.clone(), &Py::Int(0i64))?])?;
    }
    return Ok(Py::dict(vec![(Py::str("coefficients"), f__fl(v_out.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dot_product(mut v_u: Py, mut v_v: Py) -> R {
    let t1370 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t1370.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t1370[0].clone();
    v_v = t1370[1].clone();
    if ({ let t1371 = py::len(&v_u.clone())?; Py::Bool({ let t1372 = py::len(&v_v.clone())?; if py::ne(&t1371, &t1372) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries"))));
    }
    return Ok(Py::dict(vec![(Py::str("dot"), py::float(&py::sum(&{ let mut out = vec![]; for t1374 in py::iter(&(py::zip(&[f__FV(v_u.clone())?, f__FV(v_v.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1375 = (t1374).items()?;
if t1375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1375[0].clone();
v_b = t1375[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eigenvalues_2x2(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_qq: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tr: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::Int(2i64), Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    let t1414 = (Py::tuple(vec![py::add(&py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_F.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_F.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?])).items()?;
    if t1414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t1414[0].clone();
    v_det = t1414[1].clone();
    v_disc = py::sub(&py::mul(&v_tr.clone(), &v_tr.clone())?, &py::mul(&Py::Int(4i64), &v_det.clone())?)?;
    if ({ let t1415 = v_disc.clone(); Py::Bool({ let t1416 = Py::Int(0i64); if py::ge(&t1415, &t1416)? { true } else { false } }) }).truthy() {
        v_s = py::math1("sqrt", &py::float(&v_disc.clone())?)?;
        if ({ let t1417 = v_disc.clone(); Py::Bool({ let t1418 = Py::Int(0i64); if py::eq(&t1417, &t1418) { true } else { false } }) }).truthy() {
            let t1419 = py::float(&py::truediv(&v_tr.clone(), &Py::Int(2i64))?)?;
            v_l1 = t1419.clone();
            v_l2 = t1419.clone();
        } else {
            v_qq = py::truediv(&py::neg(&py::add(&py::neg(&py::float(&v_tr.clone())?)?, &py::copysign(&v_s.clone(), &py::neg(&py::float(&v_tr.clone())?)?)?)?)?, &Py::Int(2i64))?;
            let t1422 = ((if ({ let t1420 = v_qq.clone(); Py::Bool({ let t1421 = Py::Int(0i64); if py::ne(&t1420, &t1421) { true } else { false } }) }).truthy() { Py::tuple(vec![v_qq.clone(), py::truediv(&py::float(&v_det.clone())?, &v_qq.clone())?]) } else { Py::tuple(vec![py::truediv(&v_s.clone(), &Py::Int(2i64))?, py::truediv(&py::neg(&v_s.clone())?, &Py::Int(2i64))?]) })).items()?;
            if t1422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_l1 = t1422[0].clone();
            v_l2 = t1422[1].clone();
        }
        let t1423 = (py::sorted(&Py::list(vec![v_l1.clone(), v_l2.clone()]), (Py::Bool(false)).truthy())?).items()?;
        if t1423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_lo = t1423[0].clone();
        v_hi = t1423[1].clone();
        return Ok(Py::dict(vec![(Py::str("lambda1"), v_lo.clone()), (Py::str("lambda2"), v_hi.clone())]));
    }
    let t1424 = (Py::tuple(vec![py::add(&py::truediv(&py::float(&v_tr.clone())?, &Py::Int(2i64))?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::float(&py::neg(&v_disc.clone())?)?)?, &Py::Int(2i64))?])).items()?;
    if t1424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t1424[0].clone();
    v_im = t1424[1].clone();
    return Ok(Py::dict(vec![(Py::str("lambda1"), py::complex(&[v_re_.clone(), v_im.clone()])?), (Py::str("lambda2"), py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gauss_legendre_integral(mut v_coefficients: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_half: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let mut v_xs: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    let _ = Py::tuple(vec![f__finite(v_a.clone(), Py::str("a"))?, f__finite(v_b.clone(), Py::str("b"))?]);
    if (py::not(&{ let t1467 = v_a.clone(); Py::Bool({ let t1468 = v_b.clone(); if py::lt(&t1467, &t1468)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval must have a < b"))));
    }
    let t1500 = (py::method(&f_gauss_legendre_rule(v_n.clone())?, "values", &[])?).items()?;
    if t1500.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t1500[0].clone();
    v_ws = t1500[1].clone();
    let t1501 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, py::truediv(&py::sub(&v_b.clone(), &v_a.clone())?, &Py::Int(2i64))?])).items()?;
    if t1501.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mid = t1501[0].clone();
    v_half = t1501[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), py::mul(&v_half.clone(), &py::fsum(&{ let mut out = vec![]; for t1502 in py::iter(&(py::zip(&[v_xs.clone(), v_ws.clone()])?))? { let mut v_w: Py; let mut v_x: Py; 
let t1503 = (t1502).items()?;
if t1503.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t1503[0].clone();
v_w = t1503[1].clone();
out.push(py::mul(&v_w.clone(), &f__peval(v_c.clone(), py::add(&v_mid.clone(), &py::mul(&v_half.clone(), &v_x.clone())?)?)?)?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gauss_legendre_rule(mut v_n: Py) -> R {
    let mut v_ws: Py = Py::None;
    let mut v_xs: Py = Py::None;
    if ({ let t1527 = Py::Bool(matches!(v_n.clone(), Py::Bool(_))); if t1527.truthy() { t1527 } else { { let t1526 = { let t1521 = v_n.clone(); Py::Bool({ let t1522 = py::int(&v_n.clone())?; if py::ne(&t1521, &t1522) { true } else { false } }) }; if t1526.truthy() { t1526 } else { py::not(&{ let t1523 = Py::Int(1i64); Py::Bool({ let t1524 = v_n.clone(); if py::le(&t1523, &t1524)? { { let t1525 = Py::Int(40i64); if py::le(&t1524, &t1525)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 1 to 40"))));
    }
    let t1551 = (f__legendre_nodes(py::int(&v_n.clone())?)?).items()?;
    if t1551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_xs = t1551[0].clone();
    v_ws = t1551[1].clone();
    return Ok(Py::dict(vec![(Py::str("nodes"), v_xs.clone()), (Py::str("weights"), v_ws.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gauss_seidel(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_step: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    if (py::not(&{ let t1643 = f__sdd(v_A.clone())?; if t1643.truthy() { t1643 } else { f__spd(v_A.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be strictly diagonally dominant or symmetric positive definite (then Gauss-Seidel converges)"))));
    }
    v_n = py::len(&v_A.clone())?;
    v_A = { let mut out = vec![]; for t1644 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1644;
out.push({ let mut out = vec![]; for t1645 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t1645;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_step = { let v_A = v_A.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); let v_step = v_step.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_i: Py = Py::None;
            v_x = Py::list((v_x.clone()).items()?);
            for t1646 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_i = t1646;
                py::setitem(&v_x.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t1647 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1647;
if ({ let t1648 = v_j.clone(); Py::Bool({ let t1649 = v_i.clone(); if py::ne(&t1648, &t1649) { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); }} Py::list(out) })?)?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?)?;
            }
            return Ok(v_x.clone());
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("solution"), f__fixed_point(v_step.clone(), py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?, v_A.clone(), v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_general_eigenvalues(mut v_A: Py) -> R {
    let mut v_ev: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_ev = f__eig_qr(v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("eigenvalues"), { let mut out = vec![]; for t1792 in py::iter(&(v_ev.clone()))? { let mut v_z: Py; 
v_z = t1792;
out.push((if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { py::complex(&[v_z.clone()])? } else { py::float(&v_z.clone())? })); } Py::list(out) }), (Py::str("spectral_radius"), py::minmax(&[{ let mut out = vec![]; for t1793 in py::iter(&(v_ev.clone()))? { let mut v_z: Py; 
v_z = t1793;
out.push(py::abs(&py::complex(&[v_z.clone()])?)?); } Py::list(out) }], true)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gram_schmidt(mut v_vectors: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nw: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_V = f__need_mat(v_vectors.clone(), Py::str("vectors"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    let t1852 = (Py::tuple(vec![py::len(&v_V.clone())?, py::len(&py::getitem(&v_V.clone(), &Py::Int(0i64))?)?])).items()?;
    if t1852.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t1852[0].clone();
    v_n = t1852[1].clone();
    if ({ let t1877 = { let t1853 = v_k.clone(); Py::Bool({ let t1854 = v_n.clone(); if py::gt(&t1853, &t1854)? { true } else { false } }) }; if t1877.truthy() { t1877 } else { { let t1855 = f__rank_F(f__FM(v_V.clone())?)?; Py::Bool({ let t1856 = v_k.clone(); if py::lt(&t1855, &t1856)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the vectors must be linearly independent"))));
    }
    v_E = Py::list(vec![]);
    for t1878 in py::iter(&(v_V.clone()))? {
        v_v = t1878;
        v_w = { let mut out = vec![]; for t1879 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t1879;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
        for t1880 in py::range_iter(&[(Py::Int(2i64)).i()?]) {
            v__ = t1880;
            for t1881 in py::iter(&(v_E.clone()))? {
                v_e = t1881;
                v_d = f__dot(v_e.clone(), v_w.clone())?;
                v_w = { let mut out = vec![]; for t1884 in py::iter(&(py::zip(&[v_w.clone(), v_e.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1885 = (t1884).items()?;
if t1885.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1885[0].clone();
v_b = t1885[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_d.clone(), &v_b.clone())?)?); } Py::list(out) };
            }
        }
        v_nw = f__norm(v_w.clone())?;
        let _ = py::method(&v_E.clone(), "append", &[{ let mut out = vec![]; for t1887 in py::iter(&(v_w.clone()))? { let mut v_x: Py; 
v_x = t1887;
out.push(py::truediv(&v_x.clone(), &v_nw.clone())?); } Py::list(out) }])?;
    }
    return Ok(Py::dict(vec![(Py::str("orthonormal"), v_E.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_interpolating_polynomial(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let t1949 = (f__table(v_x.clone(), v_y.clone(), Py::Int(1i64), Py::Bool(true), Py::Bool(false))?).items()?;
    if t1949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t1949[0].clone();
    v_Y = t1949[1].clone();
    if ({ let t1950 = py::len(&v_X.clone())?; Py::Bool({ let t1951 = Py::Int(15i64); if py::gt(&t1950, &t1951)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("at most 15 points (the coefficients of a higher-degree interpolant are ill conditioned)"))));
    }
    v_V = { let mut out = vec![]; for t1952 in py::iter(&(v_X.clone()))? { let mut v_t: Py; 
v_t = t1952;
out.push({ let mut out = vec![]; for t1953 in py::range_iter(&[(py::len(&v_X.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1953;
out.push(py::pow(&v_t.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("coefficients"), f__fl(f__solve_F(v_V.clone(), v_Y.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverse_iteration(mut v_A: Py) -> R {
    let mut v_solve: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    if ({ let t2023 = f__det_F(f__FM(v_A.clone())?)?; Py::Bool({ let t2024 = Py::Int(0i64); if py::eq(&t2023, &t2024) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A is singular: its smallest eigenvalue is 0 and A cannot be inverted"))));
    }
    v_solve = f__lu_solver(v_A.clone())?;
    return Ok(f__iterate(v_A.clone(), v_solve.clone(), Py::str("min"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jacobi_iteration(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_step: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    if (py::not(&f__sdd(v_A.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be strictly diagonally dominant by rows (then the Jacobi iteration converges)"))));
    }
    v_n = py::len(&v_A.clone())?;
    v_A = { let mut out = vec![]; for t2255 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t2255;
out.push({ let mut out = vec![]; for t2256 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2256;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_step = { let v_A = v_A.clone(); let v_b = v_b.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut out = vec![]; for t2257 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2257;
out.push(py::truediv(&py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &py::fsum(&{ let mut out = vec![]; for t2258 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t2258;
if ({ let t2259 = v_j.clone(); Py::Bool({ let t2260 = v_i.clone(); if py::ne(&t2259, &t2260) { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); }} Py::list(out) })?)?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("solution"), f__fixed_point(v_step.clone(), py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?, v_A.clone(), v_b.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kronecker_product(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_FA: Py = Py::None;
    let mut v_FB: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t2324 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(6i64))?, f__need_mat(v_B.clone(), Py::str("B"), Py::Bool(false), Py::None, Py::None, Py::Int(6i64))?])).items()?;
    if t2324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2324[0].clone();
    v_B = t2324[1].clone();
    let t2327 = (Py::tuple(vec![f__FM(v_A.clone())?, f__FM(v_B.clone())?])).items()?;
    if t2327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FA = t2327[0].clone();
    v_FB = t2327[1].clone();
    let t2328 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?, py::len(&v_B.clone())?, py::len(&py::getitem(&v_B.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2328.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2328[0].clone();
    v_n = t2328[1].clone();
    v_p = t2328[2].clone();
    v_r = t2328[3].clone();
    return Ok(Py::dict(vec![(Py::str("kron"), { let mut out = vec![]; for t2329 in py::range_iter(&[(py::mul(&v_m.clone(), &v_p.clone())?).i()?]) { let mut v_i: Py; 
v_i = t2329;
out.push({ let mut out = vec![]; for t2330 in py::range_iter(&[(py::mul(&v_n.clone(), &v_r.clone())?).i()?]) { let mut v_j: Py; 
v_j = t2330;
out.push(py::float(&py::mul(&py::getitem(&py::getitem(&v_FA.clone(), &py::floordiv(&v_i.clone(), &v_p.clone())?)?, &py::floordiv(&v_j.clone(), &v_r.clone())?)?, &py::getitem(&py::getitem(&v_FB.clone(), &py::modulo(&v_i.clone(), &v_p.clone())?)?, &py::modulo(&v_j.clone(), &v_r.clone())?)?)?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lagrange_interpolation(mut v_x: Py, mut v_y: Py, mut v_at: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xi: Py = Py::None;
    let mut v_xj: Py = Py::None;
    let mut v_yi: Py = Py::None;
    let t2383 = (f__table(v_x.clone(), v_y.clone(), Py::Int(1i64), Py::Bool(true), Py::Bool(false))?).items()?;
    if t2383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2383[0].clone();
    v_Y = t2383[1].clone();
    v_t = f__F(v_at.clone(), Py::str("at"))?;
    v_v = py::fraction(&[Py::Int(0i64)])?;
    for t2384 in py::iter(&(py::enumerate(&py::zip(&[v_X.clone(), v_Y.clone()])?, 0)?))? {
        let t2389 = (t2384).items()?;
        if t2389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t2389[0].clone();
        let t2390 = (t2389[1].clone()).items()?;
        if t2390.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_xi = t2390[0].clone();
        v_yi = t2390[1].clone();
        v_L = py::fraction(&[Py::Int(1i64)])?;
        for t2385 in py::iter(&(py::enumerate(&v_X.clone(), 0)?))? {
            let t2388 = (t2385).items()?;
            if t2388.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t2388[0].clone();
            v_xj = t2388[1].clone();
            if ({ let t2386 = v_j.clone(); Py::Bool({ let t2387 = v_i.clone(); if py::ne(&t2386, &t2387) { true } else { false } }) }).truthy() {
                v_L = py::mul(&v_L, &py::truediv(&py::sub(&v_t.clone(), &v_xj.clone())?, &py::sub(&v_xi.clone(), &v_xj.clone())?)?)?;
            }
        }
        v_v = py::iadd(&v_v, &py::mul(&v_yi.clone(), &v_L.clone())?)?;
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_least_squares_line(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let mut v_syy: Py = Py::None;
    let t2416 = (Py::tuple(vec![f__need_vec(v_x.clone(), Py::str("x"), Py::None, Py::Int(2i64), Py::Int(1000i64))?, f__need_vec(v_y.clone(), Py::str("y"), Py::None, Py::Int(2i64), Py::Int(1000i64))?])).items()?;
    if t2416.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2416[0].clone();
    v_y = t2416[1].clone();
    if ({ let t2417 = py::len(&v_x.clone())?; Py::Bool({ let t2418 = py::len(&v_y.clone())?; if py::ne(&t2417, &t2418) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y must have the same number of values"))));
    }
    let t2420 = (Py::tuple(vec![f__FV(v_x.clone())?, f__FV(v_y.clone())?])).items()?;
    if t2420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2420[0].clone();
    v_Y = t2420[1].clone();
    v_n = py::len(&v_X.clone())?;
    let t2421 = (Py::tuple(vec![py::truediv(&py::sum(&v_X.clone(), Some(py::fraction(&[Py::Int(0i64)])?))?, &v_n.clone())?, py::truediv(&py::sum(&v_Y.clone(), Some(py::fraction(&[Py::Int(0i64)])?))?, &v_n.clone())?])).items()?;
    if t2421.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mx = t2421[0].clone();
    v_my = t2421[1].clone();
    v_sxx = py::sum(&{ let mut out = vec![]; for t2422 in py::iter(&(v_X.clone()))? { let mut v_a: Py; 
v_a = t2422;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mx.clone())?, &Py::Int(2i64))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?;
    v_syy = py::sum(&{ let mut out = vec![]; for t2423 in py::iter(&(v_Y.clone()))? { let mut v_b: Py; 
v_b = t2423;
out.push(py::pow(&py::sub(&v_b.clone(), &v_my.clone())?, &Py::Int(2i64))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?;
    v_sxy = py::sum(&{ let mut out = vec![]; for t2424 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2425 = (t2424).items()?;
if t2425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2425[0].clone();
v_b = t2425[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mx.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?;
    if ({ let t2426 = v_sxx.clone(); Py::Bool({ let t2427 = Py::Int(0i64); if py::eq(&t2426, &t2427) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all x values are equal: the line is vertical and has no slope"))));
    }
    if ({ let t2428 = v_syy.clone(); Py::Bool({ let t2429 = Py::Int(0i64); if py::eq(&t2428, &t2429) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all y values are equal: the fitted line is y = constant and R^2 is undefined"))));
    }
    v_slope = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::dict(vec![(Py::str("slope"), py::float(&v_slope.clone())?), (Py::str("intercept"), py::float(&py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mx.clone())?)?)?), (Py::str("r_squared"), py::float(&py::truediv(&py::mul(&v_sxy.clone(), &v_sxy.clone())?, &py::mul(&v_sxx.clone(), &v_syy.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_least_squares_polynomial(mut v_x: Py, mut v_y: Py, mut v_degree: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t2461 = (Py::tuple(vec![f__need_vec(v_x.clone(), Py::str("x"), Py::None, Py::Int(2i64), Py::Int(200i64))?, f__need_vec(v_y.clone(), Py::str("y"), Py::None, Py::Int(2i64), Py::Int(200i64))?])).items()?;
    if t2461.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2461[0].clone();
    v_y = t2461[1].clone();
    if ({ let t2462 = py::len(&v_x.clone())?; Py::Bool({ let t2463 = py::len(&v_y.clone())?; if py::ne(&t2462, &t2463) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x and y must have the same number of values"))));
    }
    if ({ let t2470 = Py::Bool(matches!(v_degree.clone(), Py::Bool(_))); if t2470.truthy() { t2470 } else { { let t2469 = { let t2464 = v_degree.clone(); Py::Bool({ let t2465 = py::int(&v_degree.clone())?; if py::ne(&t2464, &t2465) { true } else { false } }) }; if t2469.truthy() { t2469 } else { py::not(&{ let t2466 = Py::Int(0i64); Py::Bool({ let t2467 = v_degree.clone(); if py::le(&t2466, &t2467)? { { let t2468 = Py::Int(8i64); if py::le(&t2467, &t2468)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the degree must be a whole number from 0 to 8"))));
    }
    v_d = py::int(&v_degree.clone())?;
    v_X = f__FV(v_x.clone())?;
    if ({ let t2472 = py::len(&py::set(&v_X.clone())?)?; Py::Bool({ let t2473 = v_d.clone(); if py::le(&t2472, &t2473)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["a degree-".to_string(), py::format(&v_d.clone(), "")?, " fit needs at least ".to_string(), py::format(&py::add(&v_d.clone(), &Py::Int(1i64))?, "")?, " distinct x values".to_string()].concat()))));
    }
    v_V = { let mut out = vec![]; for t2474 in py::iter(&(v_X.clone()))? { let mut v_t: Py; 
v_t = t2474;
out.push({ let mut out = vec![]; for t2475 in py::range_iter(&[(py::add(&v_d.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2475;
out.push(py::pow(&v_t.clone(), &v_k.clone())?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("coefficients"), f__fl(f__normal_solve(v_V.clone(), f__FV(v_y.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_least_squares_solution(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_FA: Py = Py::None;
    let mut v_Fb: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    if ({ let t2575 = py::len(&v_A.clone())?; Py::Bool({ let t2576 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::lt(&t2575, &t2576)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("fewer equations than unknowns: the least-squares solution is not unique"))));
    }
    let t2580 = (Py::tuple(vec![f__FM(v_A.clone())?, f__FV(v_b.clone())?])).items()?;
    if t2580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_FA = t2580[0].clone();
    v_Fb = t2580[1].clone();
    v_x = f__normal_solve(v_FA.clone(), v_Fb.clone())?;
    v_r = { let mut out = vec![]; for t2614 in py::iter(&(py::zip(&[v_FA.clone(), v_Fb.clone()])?))? { let mut v_bi: Py; let mut v_row: Py; 
let t2615 = (t2614).items()?;
if t2615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_row = t2615[0].clone();
v_bi = t2615[1].clone();
out.push(py::sub(&v_bi.clone(), &py::sum(&{ let mut out = vec![]; for t2616 in py::iter(&(py::zip(&[v_row.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_c: Py; 
let t2617 = (t2616).items()?;
if t2617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2617[0].clone();
v_c = t2617[1].clone();
out.push(py::mul(&v_a.clone(), &v_c.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("solution"), f__fl(v_x.clone())?), (Py::str("residual_norm"), py::math1("sqrt", &py::float(&py::sum(&{ let mut out = vec![]; for t2619 in py::iter(&(v_r.clone()))? { let mut v_e: Py; 
v_e = t2619;
out.push(py::mul(&v_e.clone(), &v_e.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_independence(mut v_vectors: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_V = f__need_mat(v_vectors.clone(), Py::str("vectors"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_r = f__rank_F(f__FM(v_V.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("independent"), { let t2686 = v_r.clone(); Py::Bool({ let t2687 = py::len(&v_V.clone())?; if py::eq(&t2686, &t2687) { true } else { false } }) }), (Py::str("rank"), py::float(&v_r.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_interpolation(mut v_x: Py, mut v_y: Py, mut v_at: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2740 = (f__table(v_x.clone(), v_y.clone(), Py::Int(2i64), Py::Bool(true), Py::Bool(true))?).items()?;
    if t2740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t2740[0].clone();
    v_Y = t2740[1].clone();
    v_t = f__F(v_at.clone(), Py::str("at"))?;
    if (py::not(&{ let t2741 = py::getitem(&v_X.clone(), &Py::Int(0i64))?; Py::Bool({ let t2742 = v_t.clone(); if py::le(&t2741, &t2742)? { { let t2743 = py::getitem(&v_X.clone(), &py::neg(&Py::Int(1i64))?)?; if py::le(&t2742, &t2743)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point must lie within the range of the x values (no extrapolation)"))));
    }
    v_k = (if ({ let t2744 = v_t.clone(); Py::Bool({ let t2745 = py::getitem(&v_X.clone(), &py::neg(&Py::Int(1i64))?)?; if py::lt(&t2744, &t2745)? { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t2746 in py::range_iter(&[(py::sub(&py::len(&v_X.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t2746;
if ({ let t2747 = py::getitem(&v_X.clone(), &v_j.clone())?; Py::Bool({ let t2748 = v_t.clone(); if py::le(&t2747, &t2748)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) }], true)? } else { py::sub(&py::len(&v_X.clone())?, &Py::Int(2i64))? });
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&py::add(&py::getitem(&v_Y.clone(), &v_k.clone())?, &py::truediv(&py::mul(&py::sub(&py::getitem(&v_Y.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_Y.clone(), &v_k.clone())?)?, &py::sub(&v_t.clone(), &py::getitem(&v_X.clone(), &v_k.clone())?)?)?, &py::sub(&py::getitem(&v_X.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_X.clone(), &v_k.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_system_classification(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_FA: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_ra: Py = Py::None;
    let mut v_rb: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    v_FA = f__FM(v_A.clone())?;
    v_ra = f__rank_F(v_FA.clone())?;
    v_rb = f__rank_F({ let mut out = vec![]; for t2839 in py::iter(&(py::zip(&[v_FA.clone(), f__FV(v_b.clone())?])?))? { let mut v_r: Py; let mut v_x: Py; 
let t2840 = (t2839).items()?;
if t2840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t2840[0].clone();
v_x = t2840[1].clone();
out.push(py::add(&v_r.clone(), &Py::list(vec![v_x.clone()]))?); } Py::list(out) })?;
    v_kind = (if ({ let t2841 = v_rb.clone(); Py::Bool({ let t2842 = v_ra.clone(); if py::gt(&t2841, &t2842)? { true } else { false } }) }).truthy() { Py::str("none") } else { (if ({ let t2843 = v_ra.clone(); Py::Bool({ let t2844 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::eq(&t2843, &t2844) { true } else { false } }) }).truthy() { Py::str("unique") } else { Py::str("infinitely many") }) });
    return Ok(Py::dict(vec![(Py::str("solutions"), v_kind.clone()), (Py::str("rank_A"), py::float(&v_ra.clone())?), (Py::str("rank_augmented"), py::float(&v_rb.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_low_rank_approximation(mut v_A: Py, mut v_k: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tall: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    let t2899 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t2899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2899[0].clone();
    v_n = t2899[1].clone();
    v_r = py::minmax(&[v_m.clone(), v_n.clone()], false)?;
    if ({ let t2906 = Py::Bool(matches!(v_k.clone(), Py::Bool(_))); if t2906.truthy() { t2906 } else { { let t2905 = { let t2900 = v_k.clone(); Py::Bool({ let t2901 = py::int(&v_k.clone())?; if py::ne(&t2900, &t2901) { true } else { false } }) }; if t2905.truthy() { t2905 } else { py::not(&{ let t2902 = Py::Int(1i64); Py::Bool({ let t2903 = v_k.clone(); if py::le(&t2902, &t2903)? { { let t2904 = v_r.clone(); if py::le(&t2903, &t2904)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["k must be a whole number from 1 to ".to_string(), py::format(&v_r.clone(), "")?].concat()))));
    }
    v_k = py::int(&v_k.clone())?;
    v_tall = { let t2907 = v_m.clone(); Py::Bool({ let t2908 = v_n.clone(); if py::ge(&t2907, &t2908)? { true } else { false } }) };
    let t2949 = (f__svd_jacobi((if (v_tall.clone()).truthy() { v_A.clone() } else { f__T(v_A.clone())? }))?).items()?;
    if t2949.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t2949[0].clone();
    v_s = t2949[1].clone();
    v_V = t2949[2].clone();
    if ({ let t2954 = { let t2950 = v_k.clone(); Py::Bool({ let t2951 = v_r.clone(); if py::lt(&t2950, &t2951)? { true } else { false } }) }; if !t2954.truthy() { t2954 } else { { let t2952 = py::sub(&py::getitem(&v_s.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_s.clone(), &v_k.clone())?)?; Py::Bool({ let t2953 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::getitem(&v_s.clone(), &Py::Int(0i64))?], true)?)?; if py::le(&t2952, &t2953)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("sigma_k = sigma_(k+1): the best rank-k approximation is not unique"))));
    }
    v_B = { let mut out = vec![]; for t2955 in py::range_iter(&[(py::len(&py::getitem(&v_U.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_i: Py; 
v_i = t2955;
out.push({ let mut out = vec![]; for t2956 in py::range_iter(&[(py::len(&py::getitem(&v_V.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t2956;
out.push(py::fsum(&{ let mut out = vec![]; for t2957 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_t: Py; 
v_t = t2957;
out.push(py::mul(&py::mul(&py::getitem(&v_s.clone(), &v_t.clone())?, &py::getitem(&py::getitem(&v_U.clone(), &v_t.clone())?, &v_i.clone())?)?, &py::getitem(&py::getitem(&v_V.clone(), &v_t.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    if (py::not(&v_tall.clone())).truthy() {
        v_B = f__T(v_B.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("approximation"), v_B.clone()), (Py::str("error2"), (if ({ let t2958 = v_k.clone(); Py::Bool({ let t2959 = v_r.clone(); if py::lt(&t2958, &t2959)? { true } else { false } }) }).truthy() { py::getitem(&v_s.clone(), &v_k.clone())? } else { Py::Float(0.0) })), (Py::str("errorF"), py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t2960 in py::iter(&(py::slice(&v_s.clone(), Some((v_k.clone()).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t2960;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); } Py::list(out) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lu_decomposition(mut v_A: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_perm: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    if ({ let t3008 = f__det_F(f__FM(v_A.clone())?)?; Py::Bool({ let t3009 = Py::Int(0i64); if py::eq(&t3008, &t3009) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A is singular: this capability factors nonsingular matrices"))));
    }
    v_n = py::len(&v_A.clone())?;
    v_U = { let mut out = vec![]; for t3024 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3024;
out.push({ let mut out = vec![]; for t3025 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3025;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) };
    v_L = { let mut out = vec![]; for t3026 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3026;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    v_perm = Py::list((Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    for t3027 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3027;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_k.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_L = v_L.clone(); let v_U = v_U.clone(); let v_k = v_k.clone(); let v_n = v_n.clone(); let v_perm = v_perm.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        if ({ let t3028 = v_p.clone(); Py::Bool({ let t3029 = v_k.clone(); if py::ne(&t3028, &t3029) { true } else { false } }) }).truthy() {
            let t3030 = (Py::tuple(vec![py::getitem(&v_U.clone(), &v_p.clone())?, py::getitem(&v_U.clone(), &v_k.clone())?])).items()?;
            if t3030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_U.clone(), &v_k.clone(), t3030[0].clone())?;
            py::setitem(&v_U.clone(), &v_p.clone(), t3030[1].clone())?;
            let t3031 = (Py::tuple(vec![py::getitem(&v_L.clone(), &v_p.clone())?, py::getitem(&v_L.clone(), &v_k.clone())?])).items()?;
            if t3031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_L.clone(), &v_k.clone(), t3031[0].clone())?;
            py::setitem(&v_L.clone(), &v_p.clone(), t3031[1].clone())?;
            let t3032 = (Py::tuple(vec![py::getitem(&v_perm.clone(), &v_p.clone())?, py::getitem(&v_perm.clone(), &v_k.clone())?])).items()?;
            if t3032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_perm.clone(), &v_k.clone(), t3032[0].clone())?;
            py::setitem(&v_perm.clone(), &v_p.clone(), t3032[1].clone())?;
        }
        for t3033 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_i = t3033;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_U.clone(), &v_k.clone())?, &v_k.clone())?)?;
            py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_k.clone(), v_f.clone())?;
            py::setitem(&v_U.clone(), &v_i.clone(), { let mut out = vec![]; for t3034 in py::iter(&(py::enumerate(&py::zip(&[py::getitem(&v_U.clone(), &v_i.clone())?, py::getitem(&v_U.clone(), &v_k.clone())?])?, 0)?))? { let mut v_a: Py; let mut v_b: Py; let mut v_j: Py; 
let t3035 = (t3034).items()?;
if t3035.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t3035[0].clone();
let t3036 = (t3035[1].clone()).items()?;
if t3036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3036[0].clone();
v_b = t3036[1].clone();
out.push((if ({ let t3037 = v_j.clone(); Py::Bool({ let t3038 = v_k.clone(); if py::ge(&t3037, &t3038)? { true } else { false } }) }).truthy() { py::sub(&v_a.clone(), &py::mul(&v_f.clone(), &v_b.clone())?)? } else { v_a.clone() })); } Py::list(out) })?;
            py::setitem(&py::getitem(&v_U.clone(), &v_i.clone())?, &v_k.clone(), Py::Float(0.0))?;
        }
    }
    for t3039 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3039;
        py::setitem(&py::getitem(&v_L.clone(), &v_i.clone())?, &v_i.clone(), Py::Float(1.0))?;
    }
    v_P = { let mut out = vec![]; for t3040 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t3040;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    for t3041 in py::iter(&(py::enumerate(&v_perm.clone(), 0)?))? {
        let t3042 = (t3041).items()?;
        if t3042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t3042[0].clone();
        v_r = t3042[1].clone();
        py::setitem(&py::getitem(&v_P.clone(), &v_r.clone())?, &v_i.clone(), Py::Float(1.0))?;
    }
    return Ok(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("L"), v_L.clone()), (Py::str("U"), v_U.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_addition(mut v_A: Py, mut v_B: Py) -> R {
    let t3122 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?, f__need_mat(v_B.clone(), Py::str("B"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?])).items()?;
    if t3122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3122[0].clone();
    v_B = t3122[1].clone();
    if ({ let t3123 = f__shape(v_A.clone())?; Py::Bool({ let t3124 = f__shape(v_B.clone())?; if py::ne(&t3123, &t3124) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A and B must have the same shape"))));
    }
    return Ok(Py::dict(vec![(Py::str("sum"), { let mut out = vec![]; for t3126 in py::iter(&(py::zip(&[v_A.clone(), v_B.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t3127 = (t3126).items()?;
if t3127.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3127[0].clone();
v_s = t3127[1].clone();
out.push({ let mut out = vec![]; for t3128 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3129 = (t3128).items()?;
if t3129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3129[0].clone();
v_b = t3129[1].clone();
out.push(py::float(&py::add(&f__F(v_a.clone(), Py::str("input"))?, &f__F(v_b.clone(), Py::str("input"))?)?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_exponential(mut v_A: Py) -> R {
    let mut v_norm1: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_norm1 = py::minmax(&[{ let mut out = vec![]; for t3174 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_j: Py; 
v_j = t3174;
out.push(py::fsum(&{ let mut out = vec![]; for t3175 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3175;
out.push(py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }], true)?;
    if ({ let t3176 = v_norm1.clone(); Py::Bool({ let t3177 = Py::Int(50i64); if py::gt(&t3176, &t3177)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("this version computes exp(A) for |A|_1 <= 50 (larger norms lose accuracy)"))));
    }
    return Ok(Py::dict(vec![(Py::str("exponential"), f__expm(v_A.clone(), Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_inverse(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    return Ok(Py::dict(vec![(Py::str("inverse"), f__fl(f__inv_F(f__FM(v_A.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_multiplication(mut v_A: Py, mut v_B: Py) -> R {
    let t3346 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?, f__need_mat(v_B.clone(), Py::str("B"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?])).items()?;
    if t3346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3346[0].clone();
    v_B = t3346[1].clone();
    if ({ let t3347 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3348 = py::len(&v_B.clone())?; if py::ne(&t3347, &t3348) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of columns of A must equal the number of rows of B"))));
    }
    return Ok(Py::dict(vec![(Py::str("product"), f__fl(f__mm_F(f__FM(v_A.clone())?, f__FM(v_B.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_norms(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_fro: Py = Py::None;
    let mut v_inf: Py = Py::None;
    let mut v_one: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    v_fro = py::math1("sqrt", &py::float(&py::sum(&{ let mut out = vec![]; for t3409 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t3409;
for t3410 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3410;
out.push(py::mul(&v_x.clone(), &v_x.clone())?); }} Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?;
    v_one = py::float(&py::minmax(&[{ let mut out = vec![]; for t3411 in py::range_iter(&[(py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t3411;
out.push(py::sum(&{ let mut out = vec![]; for t3412 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t3412;
out.push(py::abs(&py::getitem(&v_r.clone(), &v_j.clone())?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }], true)?)?;
    v_inf = py::float(&py::minmax(&[{ let mut out = vec![]; for t3413 in py::iter(&(v_F.clone()))? { let mut v_r: Py; 
v_r = t3413;
out.push(py::sum(&{ let mut out = vec![]; for t3414 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3414;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }], true)?)?;
    return Ok(Py::dict(vec![(Py::str("frobenius"), v_fro.clone()), (Py::str("norm1"), v_one.clone()), (Py::str("norm_inf"), v_inf.clone()), (Py::str("norm2"), py::getitem(&py::getitem(&f_singular_values(v_A.clone())?, &Py::str("singular_values"))?, &Py::Int(0i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_power(mut v_A: Py, mut v_k: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, Py::Int(6i64))?;
    if ({ let t3505 = Py::Bool(matches!(v_k.clone(), Py::Bool(_))); if t3505.truthy() { t3505 } else { { let t3504 = py::not(&Py::Bool(matches!(v_k.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3504.truthy() { t3504 } else { { let t3503 = { let t3496 = v_k.clone(); Py::Bool({ let t3497 = py::int(&v_k.clone())?; if py::ne(&t3496, &t3497) { true } else { false } }) }; if t3503.truthy() { t3503 } else { { let t3502 = { let t3498 = v_k.clone(); Py::Bool({ let t3499 = Py::Int(0i64); if py::lt(&t3498, &t3499)? { true } else { false } }) }; if t3502.truthy() { t3502 } else { { let t3500 = v_k.clone(); Py::Bool({ let t3501 = Py::Int(64i64); if py::gt(&t3500, &t3501)? { true } else { false } }) } } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must be a whole number from 0 to 64"))));
    }
    let t3506 = (Py::tuple(vec![py::int(&v_k.clone())?, py::len(&v_A.clone())?])).items()?;
    if t3506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_k = t3506[0].clone();
    v_n = t3506[1].clone();
    v_R = { let mut out = vec![]; for t3507 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3507;
out.push({ let mut out = vec![]; for t3508 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3508;
out.push(py::fraction(&[py::int(&{ let t3509 = v_i.clone(); Py::Bool({ let t3510 = v_j.clone(); if py::eq(&t3509, &t3510) { true } else { false } }) })?])?); } Py::list(out) }); } Py::list(out) };
    v_P = f__FM(v_A.clone())?;
    while (v_k.clone()).truthy() {
        if (py::bitand(&v_k.clone(), &Py::Int(1i64))?).truthy() {
            v_R = f__mm_F(v_R.clone(), v_P.clone())?;
        }
        v_k = py::rshift(&v_k, &Py::Int(1i64))?;
        if (v_k.clone()).truthy() {
            v_P = f__mm_F(v_P.clone(), v_P.clone())?;
        }
    }
    v_out = f__fl(v_R.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t3518 in py::iter(&(v_out.clone()))? { let mut v_r: Py; 
v_r = t3518;
for t3519 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t3519;
out.push(py::not(&py::math1("isfinite", &v_x.clone())?)); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the entries of A^k are too large for floating-point numbers"))));
    }
    return Ok(Py::dict(vec![(Py::str("power"), v_out.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_rank(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    return Ok(Py::dict(vec![(Py::str("rank"), py::float(&f__rank_F(f__FM(v_A.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_square_root(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_A = f__need_symmetric(v_A.clone(), Py::str("A"))?;
    v_F = f__FM(v_A.clone())?;
    for t3657 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_A.clone())?, &Py::Int(1i64))?).i()?]) {
        v_k = t3657;
        if ({ let t3658 = f__det_F({ let mut out = vec![]; for t3672 in py::iter(&(py::slice(&v_F.clone(), None, Some((v_k.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t3672;
out.push(py::slice(&v_r.clone(), None, Some((v_k.clone()).i()?), None)?); } Py::list(out) })?; Py::Bool({ let t3659 = Py::Int(0i64); if py::le(&t3658, &t3659)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("A must be positive definite (a leading principal minor is not positive)"))));
        }
    }
    let t3704 = (f__jacobi_eig(v_A.clone())?).items()?;
    if t3704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_vals = t3704[0].clone();
    v_V = t3704[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_X = { let mut out = vec![]; for t3705 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3705;
out.push({ let mut out = vec![]; for t3706 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3706;
out.push(py::fsum(&{ let mut out = vec![]; for t3707 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3707;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_V.clone(), &v_i.clone())?, &v_k.clone())?, &py::math1("sqrt", &py::getitem(&v_vals.clone(), &v_k.clone())?)?)?, &py::getitem(&py::getitem(&v_V.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_X = { let mut out = vec![]; for t3708 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3708;
out.push({ let mut out = vec![]; for t3709 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3709;
out.push(py::truediv(&py::add(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_j.clone())?, &v_i.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("sqrt"), v_X.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_subtraction(mut v_A: Py, mut v_B: Py) -> R {
    let t3769 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?, f__need_mat(v_B.clone(), Py::str("B"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?])).items()?;
    if t3769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3769[0].clone();
    v_B = t3769[1].clone();
    if ({ let t3770 = f__shape(v_A.clone())?; Py::Bool({ let t3771 = f__shape(v_B.clone())?; if py::ne(&t3770, &t3771) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A and B must have the same shape"))));
    }
    return Ok(Py::dict(vec![(Py::str("difference"), { let mut out = vec![]; for t3773 in py::iter(&(py::zip(&[v_A.clone(), v_B.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t3774 = (t3773).items()?;
if t3774.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3774[0].clone();
v_s = t3774[1].clone();
out.push({ let mut out = vec![]; for t3775 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3776 = (t3775).items()?;
if t3776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3776[0].clone();
v_b = t3776[1].clone();
out.push(py::float(&py::sub(&f__F(v_a.clone(), Py::str("input"))?, &f__F(v_b.clone(), Py::str("input"))?)?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_trace(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, Py::Int(0i64))?;
    return Ok(Py::dict(vec![(Py::str("trace"), py::float(&py::sum(&{ let mut out = vec![]; for t3821 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_i: Py; 
v_i = t3821;
out.push(f__F(py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?, Py::str("input"))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_transpose(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?;
    return Ok(Py::dict(vec![(Py::str("transpose"), { let mut out = vec![]; for t3858 in py::iter(&(py::zip(&{ let mut v: Vec<Py> = vec![]; v.extend((v_A.clone()).items()?); v })?))? { let mut v_c: Py; 
v_c = t3858;
out.push({ let mut out = vec![]; for t3859 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t3859;
out.push(py::float(&v_x.clone())?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_matrix_vector_product(mut v_A: Py, mut v_x: Py) -> R {
    let mut v_X: Py = Py::None;
    let t3916 = (Py::tuple(vec![f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?, f__need_vec(v_x.clone(), Py::str("x"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t3916.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3916[0].clone();
    v_x = t3916[1].clone();
    if ({ let t3917 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t3918 = py::len(&v_x.clone())?; if py::ne(&t3917, &t3918) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of columns of A must equal the length of x"))));
    }
    v_X = f__FV(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("image"), { let mut out = vec![]; for t3920 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t3920;
out.push(py::float(&py::sum(&{ let mut out = vec![]; for t3921 in py::iter(&(py::zip(&[v_r.clone(), v_X.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3922 = (t3921).items()?;
if t3922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3922[0].clone();
v_b = t3922[1].clone();
out.push(py::mul(&f__F(v_a.clone(), Py::str("input"))?, &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_midpoint_rule(mut v_coefficients: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_pv: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    let _ = Py::tuple(vec![f__finite(v_a.clone(), Py::str("a"))?, f__finite(v_b.clone(), Py::str("b"))?]);
    if (py::not(&{ let t3957 = v_a.clone(); Py::Bool({ let t3958 = v_b.clone(); if py::lt(&t3957, &t3958)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval must have a < b"))));
    }
    if ({ let t3965 = Py::Bool(matches!(v_n.clone(), Py::Bool(_))); if t3965.truthy() { t3965 } else { { let t3964 = { let t3959 = v_n.clone(); Py::Bool({ let t3960 = py::int(&v_n.clone())?; if py::ne(&t3959, &t3960) { true } else { false } }) }; if t3964.truthy() { t3964 } else { py::not(&{ let t3961 = Py::Int(1i64); Py::Bool({ let t3962 = v_n.clone(); if py::le(&t3961, &t3962)? { { let t3963 = Py::Int(1000i64); if py::le(&t3962, &t3963)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be a whole number from 1 to 1000"))));
    }
    let t3966 = (Py::tuple(vec![f__F(v_a.clone(), Py::str("input"))?, f__F(v_b.clone(), Py::str("input"))?, py::int(&v_n.clone())?])).items()?;
    if t3966.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3966[0].clone();
    v_B = t3966[1].clone();
    v_N = t3966[2].clone();
    v_h = py::truediv(&py::sub(&v_B.clone(), &v_A.clone())?, &v_N.clone())?;
    v_C = f__FV(v_c.clone())?;
    v_pv = { let v_A = v_A.clone(); let v_B = v_B.clone(); let v_C = v_C.clone(); let v_N = v_N.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_coefficients = v_coefficients.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t3968 in py::iter(&(py::enumerate(&v_C.clone(), 0)?))? { let mut v_cc: Py; let mut v_k: Py; 
let t3969 = (t3968).items()?;
if t3969.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3969[0].clone();
v_cc = t3969[1].clone();
out.push(py::mul(&v_cc.clone(), &py::pow(&v_t.clone(), &py::sub(&py::sub(&py::len(&v_C.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?)?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("integral"), py::float(&py::mul(&v_h.clone(), &py::sum(&{ let mut out = vec![]; for t3970 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_k: Py; 
v_k = t3970;
out.push(py::call(&v_pv.clone(), &[py::add(&v_A.clone(), &py::mul(&py::add(&v_k.clone(), &py::fraction(&[Py::Int(1i64), Py::Int(2i64)])?)?, &v_h.clone())?)?])?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_minor_cofactor(mut v_A: Py, mut v_i: Py, mut v_j: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_name: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_n = py::len(&v_A.clone())?;
    if ({ let t4025 = py::len(&v_A.clone())?; Py::Bool({ let t4026 = Py::Int(2i64); if py::lt(&t4025, &t4026)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a minor needs a matrix of at least 2 x 2"))));
    }
    for t4027 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("i"), v_i.clone()]), Py::tuple(vec![Py::str("j"), v_j.clone()])])))? {
        let t4035 = (t4027).items()?;
        if t4035.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_name = t4035[0].clone();
        v_v = t4035[1].clone();
        if ({ let t4034 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t4034.truthy() { t4034 } else { { let t4033 = { let t4028 = v_v.clone(); Py::Bool({ let t4029 = py::int(&v_v.clone())?; if py::ne(&t4028, &t4029) { true } else { false } }) }; if t4033.truthy() { t4033 } else { py::not(&{ let t4030 = Py::Int(1i64); Py::Bool({ let t4031 = v_v.clone(); if py::le(&t4030, &t4031)? { { let t4032 = v_n.clone(); if py::le(&t4031, &t4032)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number from 1 to ".to_string(), py::format(&v_n.clone(), "")?].concat()))));
        }
    }
    v_m = f__minor_F(f__FM(v_A.clone())?, py::sub(&py::int(&v_i.clone())?, &Py::Int(1i64))?, py::sub(&py::int(&v_j.clone())?, &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("minor"), py::float(&v_m.clone())?), (Py::str("cofactor"), py::float(&(if ({ let t4058 = py::modulo(&py::add(&py::int(&v_i.clone())?, &py::int(&v_j.clone())?)?, &Py::Int(2i64))?; Py::Bool({ let t4059 = Py::Int(0i64); if py::eq(&t4058, &t4059) { true } else { false } }) }).truthy() { v_m.clone() } else { py::neg(&v_m.clone())? }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_natural_cubic_spline(mut v_x: Py, mut v_y: Py, mut v_at: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t4112 = (f__table(v_x.clone(), v_y.clone(), Py::Int(3i64), Py::Bool(true), Py::Bool(true))?).items()?;
    if t4112.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t4112[0].clone();
    v_Y = t4112[1].clone();
    v_t = f__F(v_at.clone(), Py::str("at"))?;
    if (py::not(&{ let t4113 = py::getitem(&v_X.clone(), &Py::Int(0i64))?; Py::Bool({ let t4114 = v_t.clone(); if py::le(&t4113, &t4114)? { { let t4115 = py::getitem(&v_X.clone(), &py::neg(&Py::Int(1i64))?)?; if py::le(&t4114, &t4115)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the point must lie within the range of the x values (no extrapolation)"))));
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&f__spline_value(v_X.clone(), v_Y.clone(), v_t.clone(), Py::None, Py::None)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_newton_root(mut v_coefficients: Py, mut v_x0: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_dc: Py = Py::None;
    let mut v_step: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    let _ = f__finite(v_x0.clone(), Py::str("x0"))?;
    if ({ let t4257 = py::len(&v_c.clone())?; Py::Bool({ let t4258 = Py::Int(2i64); if py::lt(&t4257, &t4258)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a constant polynomial has no isolated root"))));
    }
    v_dc = f__pder(v_c.clone())?;
    v_step = { let v_c = v_c.clone(); let v_coefficients = v_coefficients.clone(); let v_dc = v_dc.clone(); let v_step = v_step.clone(); let v_x0 = v_x0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v__: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_d: Py = Py::None;
            v_d = f__peval(v_dc.clone(), v_x.clone())?;
            if ({ let t4263 = v_d.clone(); Py::Bool({ let t4264 = Py::Int(0i64); if py::eq(&t4263, &t4264) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("p'(x) = 0 at an iterate: Newton's method breaks down"))));
            }
            return Ok(py::sub(&v_x.clone(), &py::truediv(&f__peval(v_c.clone(), v_x.clone())?, &v_d.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("root"), f__newton_like(v_c.clone(), v_step.clone(), py::float(&v_x0.clone())?, Py::None)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_null_space(mut v_A: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_basis: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_piv: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    let t4362 = (f__rref_F(f__FM(v_A.clone())?)?).items()?;
    if t4362.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t4362[0].clone();
    v_piv = t4362[1].clone();
    v_n = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?;
    v_basis = Py::list(vec![]);
    for t4366 in py::iter(&({ let mut out = vec![]; for t4363 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_c: Py; 
v_c = t4363;
if ({ let t4364 = v_c.clone(); Py::Bool({ let t4365 = v_piv.clone(); if !py::contains(&t4365, &t4364)? { true } else { false } }) }).truthy() { out.push(v_c.clone()); }} Py::list(out) }))? {
        v_f = t4366;
        v_v = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
        py::setitem(&v_v.clone(), &v_f.clone(), py::fraction(&[Py::Int(1i64)])?)?;
        for t4367 in py::iter(&(py::enumerate(&v_piv.clone(), 0)?))? {
            let t4368 = (t4367).items()?;
            if t4368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_row = t4368[0].clone();
            v_p = t4368[1].clone();
            py::setitem(&v_v.clone(), &v_p.clone(), py::neg(&py::getitem(&py::getitem(&v_R.clone(), &v_row.clone())?, &v_f.clone())?)?)?;
        }
        let _ = py::method(&v_basis.clone(), "append", &[f__fl(v_v.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("nullity"), py::float(&py::len(&v_basis.clone())?)?), (Py::str("basis"), v_basis.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_numerical_derivative(mut v_y: Py, mut v_h: Py) -> R {
    let mut v_H: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_y = f__need_vec(v_y.clone(), Py::str("y"), Py::None, Py::Int(2i64), Py::Int(200i64))?;
    let _ = f__finite(v_h.clone(), Py::str("h"))?;
    if ({ let t4403 = v_h.clone(); Py::Bool({ let t4404 = Py::Int(0i64); if py::le(&t4403, &t4404)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the spacing h must be positive"))));
    }
    let t4406 = (Py::tuple(vec![f__FV(v_y.clone())?, f__F(v_h.clone(), Py::str("input"))?])).items()?;
    if t4406.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Y = t4406[0].clone();
    v_H = t4406[1].clone();
    v_n = py::len(&v_Y.clone())?;
    v_d = py::add(&py::add(&Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &Py::Int(1i64))?, &py::getitem(&v_Y.clone(), &Py::Int(0i64))?)?, &v_H.clone())?]), &{ let mut out = vec![]; for t4407 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t4407;
out.push(py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_Y.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&Py::Int(2i64), &v_H.clone())?)?); } Py::list(out) })?, &Py::list(vec![py::truediv(&py::sub(&py::getitem(&v_Y.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_Y.clone(), &py::neg(&Py::Int(2i64))?)?)?, &v_H.clone())?]))?;
    return Ok(Py::dict(vec![(Py::str("derivative"), f__fl(v_d.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_outer_product(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let t4449 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), c_MAX_DIM()?)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), c_MAX_DIM()?)?])).items()?;
    if t4449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t4449[0].clone();
    v_v = t4449[1].clone();
    let t4451 = (Py::tuple(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?])).items()?;
    if t4451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4451[0].clone();
    v_V = t4451[1].clone();
    return Ok(Py::dict(vec![(Py::str("outer"), { let mut out = vec![]; for t4452 in py::iter(&(v_U.clone()))? { let mut v_a: Py; 
v_a = t4452;
out.push({ let mut out = vec![]; for t4453 in py::iter(&(v_V.clone()))? { let mut v_b: Py; 
v_b = t4453;
out.push(py::float(&py::mul(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallelogram_area(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_uu: Py = Py::None;
    let mut v_uv: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let t4481 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(2i64), Py::Int(3i64))?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(2i64), Py::Int(3i64))?])).items()?;
    if t4481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t4481[0].clone();
    v_v = t4481[1].clone();
    if ({ let t4482 = py::len(&v_u.clone())?; Py::Bool({ let t4483 = py::len(&v_v.clone())?; if py::ne(&t4482, &t4483) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries (2 or 3)"))));
    }
    let t4485 = (Py::tuple(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?])).items()?;
    if t4485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4485[0].clone();
    v_V = t4485[1].clone();
    let t4490 = ({ let mut out = vec![]; for t4486 in py::iter(&(Py::tuple(vec![Py::tuple(vec![v_U.clone(), v_U.clone()]), Py::tuple(vec![v_V.clone(), v_V.clone()]), Py::tuple(vec![v_U.clone(), v_V.clone()])])))? { let mut v_X: Py; let mut v_Y: Py; 
let t4487 = (t4486).items()?;
if t4487.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_X = t4487[0].clone();
v_Y = t4487[1].clone();
out.push(py::sum(&{ let mut out = vec![]; for t4488 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4489 = (t4488).items()?;
if t4489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4489[0].clone();
v_b = t4489[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?); } Py::list(out) }).items()?;
    if t4490.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_uu = t4490[0].clone();
    v_vv = t4490[1].clone();
    v_uv = t4490[2].clone();
    v_area = py::math1("sqrt", &py::float(&py::sub(&py::mul(&v_uu.clone(), &v_vv.clone())?, &py::mul(&v_uv.clone(), &v_uv.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("area"), v_area.clone()), (Py::str("triangle_area"), py::truediv(&v_area.clone(), &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poisson_1d(mut v_f: Py, mut v_L: Py) -> R {
    let mut v_cp: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_f = f__need_vec(v_f.clone(), Py::str("f"), Py::None, Py::Int(1i64), Py::Int(200i64))?;
    let _ = f__finite(v_L.clone(), Py::str("L"))?;
    if ({ let t4515 = v_L.clone(); Py::Bool({ let t4516 = Py::Int(0i64); if py::le(&t4515, &t4516)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the interval length L must be positive"))));
    }
    v_n = py::len(&v_f.clone())?;
    v_h = py::truediv(&f__F(v_L.clone(), Py::str("input"))?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_rhs = { let mut out = vec![]; for t4518 in py::iter(&(f__FV(v_f.clone())?))? { let mut v_x: Py; 
v_x = t4518;
out.push(py::mul(&py::mul(&v_h.clone(), &v_h.clone())?, &v_x.clone())?); } Py::list(out) };
    let t4519 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?, py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?])).items()?;
    if t4519.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cp = t4519[0].clone();
    v_dp = t4519[1].clone();
    for t4520 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t4520;
        v_den = py::add(&Py::Int(2i64), &(if (v_i.clone()).truthy() { py::getitem(&v_cp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)? } else { Py::Int(0i64) }))?;
        py::setitem(&v_cp.clone(), &v_i.clone(), py::truediv(&py::fraction(&[py::neg(&Py::Int(1i64))?])?, &v_den.clone())?)?;
        py::setitem(&v_dp.clone(), &v_i.clone(), py::truediv(&py::add(&py::getitem(&v_rhs.clone(), &v_i.clone())?, &(if (v_i.clone()).truthy() { py::getitem(&v_dp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)? } else { Py::Int(0i64) }))?, &v_den.clone())?)?;
    }
    v_u = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t4521 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t4521;
        py::setitem(&v_u.clone(), &v_i.clone(), py::sub(&py::getitem(&v_dp.clone(), &v_i.clone())?, &(if ({ let t4522 = v_i.clone(); Py::Bool({ let t4523 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::lt(&t4522, &t4523)? { true } else { false } }) }).truthy() { py::mul(&py::getitem(&v_cp.clone(), &v_i.clone())?, &py::getitem(&v_u.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)? } else { Py::Int(0i64) }))?)?;
    }
    return Ok(Py::dict(vec![(Py::str("u"), f__fl(v_u.clone())?), (Py::str("h"), py::float(&v_h.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poisson_2d(mut v_f: Py, mut v_h: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_fct: Py = Py::None;
    let mut v_hh: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_F = f__need_mat(v_f.clone(), Py::str("f"), Py::Bool(false), Py::None, Py::None, Py::Int(12i64))?;
    let _ = f__finite(v_h.clone(), Py::str("h"))?;
    if ({ let t4567 = v_h.clone(); Py::Bool({ let t4568 = Py::Int(0i64); if py::le(&t4567, &t4568)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the grid spacing h must be positive"))));
    }
    let t4569 = (Py::tuple(vec![py::len(&v_F.clone())?, py::len(&py::getitem(&v_F.clone(), &Py::Int(0i64))?)?])).items()?;
    if t4569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t4569[0].clone();
    v_m = t4569[1].clone();
    v_N = py::mul(&v_n.clone(), &v_m.clone())?;
    v_w = v_m.clone();
    v_hh = py::mul(&py::float(&v_h.clone())?, &py::float(&v_h.clone())?)?;
    v_A = { let mut out = vec![]; for t4570 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v__: Py; 
v__ = t4570;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?); } Py::list(out) };
    v_rhs = { let mut out = vec![]; for t4571 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4571;
for t4572 in py::range_iter(&[(v_m.clone()).i()?]) { let mut v_j: Py; 
v_j = t4572;
out.push(py::mul(&v_hh.clone(), &py::float(&py::getitem(&py::getitem(&v_F.clone(), &v_i.clone())?, &v_j.clone())?)?)?); }} Py::list(out) };
    for t4573 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t4573;
        for t4574 in py::range_iter(&[(v_m.clone()).i()?]) {
            v_j = t4574;
            v_k = py::add(&py::mul(&v_i.clone(), &v_m.clone())?, &v_j.clone())?;
            py::setitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_k.clone(), Py::Float(4.0))?;
            if ({ let t4575 = v_j.clone(); Py::Bool({ let t4576 = Py::Int(0i64); if py::gt(&t4575, &t4576)? { true } else { false } }) }).truthy() {
                py::setitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &py::sub(&v_k.clone(), &Py::Int(1i64))?, py::neg(&Py::Float(1.0))?)?;
            }
            if ({ let t4577 = v_j.clone(); Py::Bool({ let t4578 = py::sub(&v_m.clone(), &Py::Int(1i64))?; if py::lt(&t4577, &t4578)? { true } else { false } }) }).truthy() {
                py::setitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?, py::neg(&Py::Float(1.0))?)?;
            }
            if ({ let t4579 = v_i.clone(); Py::Bool({ let t4580 = Py::Int(0i64); if py::gt(&t4579, &t4580)? { true } else { false } }) }).truthy() {
                py::setitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &py::sub(&v_k.clone(), &v_m.clone())?, py::neg(&Py::Float(1.0))?)?;
            }
            if ({ let t4581 = v_i.clone(); Py::Bool({ let t4582 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::lt(&t4581, &t4582)? { true } else { false } }) }).truthy() {
                py::setitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &py::add(&v_k.clone(), &v_m.clone())?, py::neg(&Py::Float(1.0))?)?;
            }
        }
    }
    for t4583 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_c = t4583;
        for t4584 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (py::minmax(&[v_N.clone(), py::add(&py::add(&v_c.clone(), &v_w.clone())?, &Py::Int(1i64))?], false)?).i()?]) {
            v_r = t4584;
            if ({ let t4585 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t4586 = Py::Int(0i64); if py::ne(&t4585, &t4586) { true } else { false } }) }).truthy() {
                v_fct = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_c.clone())?, &v_c.clone())?)?;
                for t4587 in py::range_iter(&[(v_c.clone()).i()?, (py::minmax(&[v_N.clone(), py::add(&py::add(&v_c.clone(), &v_w.clone())?, &Py::Int(1i64))?], false)?).i()?]) {
                    v_k = t4587;
                    let t4588 = py::getitem(&v_A.clone(), &v_r.clone())?; let t4589 = v_k.clone();
                    py::setitem(&t4588, &t4589, py::sub(&py::getitem(&t4588, &t4589)?, &py::mul(&v_fct.clone(), &py::getitem(&py::getitem(&v_A.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
                let t4590 = v_rhs.clone(); let t4591 = v_r.clone();
                py::setitem(&t4590, &t4591, py::sub(&py::getitem(&t4590, &t4591)?, &py::mul(&v_fct.clone(), &py::getitem(&v_rhs.clone(), &v_c.clone())?)?)?)?;
            }
        }
    }
    v_u = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?;
    for t4592 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_N.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t4592;
        py::setitem(&v_u.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&v_rhs.clone(), &v_r.clone())?, &py::fsum(&{ let mut out = vec![]; for t4593 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (py::minmax(&[v_N.clone(), py::add(&py::add(&v_r.clone(), &v_w.clone())?, &Py::Int(1i64))?], false)?).i()?]) { let mut v_k: Py; 
v_k = t4593;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_u.clone(), &v_k.clone())?)?); } Py::list(out) })?)?, &py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(Py::dict(vec![(Py::str("u"), { let mut out = vec![]; for t4594 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4594;
out.push(py::slice(&v_u.clone(), Some((py::mul(&v_i.clone(), &v_m.clone())?).i()?), Some((py::mul(&py::add(&v_i.clone(), &Py::Int(1i64))?, &v_m.clone())?).i()?), None)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polar_decomposition(mut v_A: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    if ({ let t4648 = f__det_F(f__FM(v_A.clone())?)?; Py::Bool({ let t4649 = Py::Int(0i64); if py::eq(&t4648, &t4649) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A is singular: its polar factor is not unique"))));
    }
    let t4703 = (f__svd_jacobi(v_A.clone())?).items()?;
    if t4703.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_U = t4703[0].clone();
    v_s = t4703[1].clone();
    v_V = t4703[2].clone();
    v_n = py::len(&v_A.clone())?;
    v_W = { let mut out = vec![]; for t4704 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4704;
out.push({ let mut out = vec![]; for t4705 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4705;
out.push(py::fsum(&{ let mut out = vec![]; for t4706 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4706;
out.push(py::mul(&py::getitem(&py::getitem(&v_U.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_P = { let mut out = vec![]; for t4707 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4707;
out.push({ let mut out = vec![]; for t4708 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4708;
out.push(py::fsum(&{ let mut out = vec![]; for t4709 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t4709;
out.push(py::mul(&py::mul(&py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&v_s.clone(), &v_k.clone())?)?, &py::getitem(&py::getitem(&v_V.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_P = { let mut out = vec![]; for t4710 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4710;
out.push({ let mut out = vec![]; for t4711 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4711;
out.push(py::truediv(&py::add(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_i.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("U"), v_W.clone()), (Py::str("P"), v_P.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_polynomial_evaluation(mut v_coefficients: Py, mut v_x: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    v_X = f__F(v_x.clone(), Py::str("x"))?;
    let t4776 = (Py::tuple(vec![py::fraction(&[Py::Int(0i64)])?, py::fraction(&[Py::Int(0i64)])?])).items()?;
    if t4776.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t4776[0].clone();
    v_dv = t4776[1].clone();
    for t4778 in py::iter(&(f__FV(v_c.clone())?))? {
        v_a = t4778;
        v_dv = py::add(&py::mul(&v_dv.clone(), &v_X.clone())?, &v_v.clone())?;
        v_v = py::add(&py::mul(&v_v.clone(), &v_X.clone())?, &v_a.clone())?;
    }
    return Ok(Py::dict(vec![(Py::str("value"), py::float(&v_v.clone())?), (Py::str("derivative"), py::float(&v_dv.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_positive_definiteness(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_minors: Py = Py::None;
    v_A = f__need_symmetric(v_A.clone(), Py::str("A"))?;
    v_F = f__FM(v_A.clone())?;
    v_minors = { let mut out = vec![]; for t4839 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&py::len(&v_A.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t4839;
out.push(f__det_F({ let mut out = vec![]; for t4852 in py::iter(&(py::slice(&v_F.clone(), None, Some((v_k.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t4852;
out.push(py::slice(&v_r.clone(), None, Some((v_k.clone()).i()?), None)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("positive_definite"), Py::Bool(({ let mut out = vec![]; for t4853 in py::iter(&(v_minors.clone()))? { let mut v_m: Py; 
v_m = t4853;
out.push({ let t4854 = v_m.clone(); Py::Bool({ let t4855 = Py::Int(0i64); if py::gt(&t4854, &t4855)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))), (Py::str("leading_minors"), { let mut out = vec![]; for t4856 in py::iter(&(v_minors.clone()))? { let mut v_m: Py; 
v_m = t4856;
out.push(py::float(&v_m.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_iteration(mut v_A: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    return Ok(f__iterate(v_A.clone(), { let v_A = v_A.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(f__mv(v_A.clone(), v_x.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::str("max"))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_projection_matrix(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Ft: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    if ({ let t5105 = f__rank_F(v_F.clone())?; Py::Bool({ let t5106 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::ne(&t5105, &t5106) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the columns of A must be linearly independent"))));
    }
    v_Ft = f__T(v_F.clone())?;
    return Ok(Py::dict(vec![(Py::str("projection"), f__fl(f__mm_F(f__mm_F(v_F.clone(), f__inv_F(f__mm_F(v_Ft.clone(), v_F.clone())?)?)?, v_Ft.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pseudoinverse(mut v_A: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Ft: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_F = f__FM(v_A.clone())?;
    let t5199 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t5199.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5199[0].clone();
    v_n = t5199[1].clone();
    v_r = f__rank_F(v_F.clone())?;
    v_Ft = f__T(v_F.clone())?;
    if ({ let t5219 = v_r.clone(); Py::Bool({ let t5220 = v_n.clone(); if py::eq(&t5219, &t5220) { true } else { false } }) }).truthy() {
        v_P = f__mm_F(f__inv_F(f__mm_F(v_Ft.clone(), v_F.clone())?)?, v_Ft.clone())?;
    } else {
        if ({ let t5234 = v_r.clone(); Py::Bool({ let t5235 = v_m.clone(); if py::eq(&t5234, &t5235) { true } else { false } }) }).truthy() {
            v_P = f__mm_F(v_Ft.clone(), f__inv_F(f__mm_F(v_F.clone(), v_Ft.clone())?)?)?;
        } else {
            return Err(py::value_error(py::to_str(&Py::str("this version computes the pseudoinverse of a matrix of full rank (independent rows or columns) only"))));
        }
    }
    return Ok(Py::dict(vec![(Py::str("pseudoinverse"), f__fl(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qr_decomposition(mut v_A: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_R: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    if ({ let t5299 = py::len(&v_A.clone())?; Py::Bool({ let t5300 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::lt(&t5299, &t5300)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A needs at least as many rows as columns"))));
    }
    if ({ let t5301 = f__rank_F(f__FM(v_A.clone())?)?; Py::Bool({ let t5302 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::lt(&t5301, &t5302)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the columns of A are linearly dependent"))));
    }
    let t5360 = (f__qr_house(v_A.clone())?).items()?;
    if t5360.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t5360[0].clone();
    v_R = t5360[1].clone();
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("R"), v_R.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rotate_vector_2d(mut v_v: Py, mut v_angle: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_v = f__need_vec(v_v.clone(), Py::str("v"), Py::Int(2i64), Py::Int(1i64), Py::None)?;
    let _ = f__finite(v_angle.clone(), Py::str("angle"))?;
    let t5419 = (f__cos_sin_deg(v_angle.clone())?).items()?;
    if t5419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5419[0].clone();
    v_s = t5419[1].clone();
    return Ok(Py::dict(vec![(Py::str("rotated"), Py::list(vec![py::sub(&py::mul(&v_c.clone(), &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&v_s.clone(), &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?, py::add(&py::mul(&v_s.clone(), &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?, &py::mul(&v_c.clone(), &py::getitem(&v_v.clone(), &Py::Int(1i64))?)?)?]))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rref(mut v_A: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_piv: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    let t5481 = (f__rref_F(f__FM(v_A.clone())?)?).items()?;
    if t5481.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_R = t5481[0].clone();
    v_piv = t5481[1].clone();
    return Ok(Py::dict(vec![(Py::str("rref"), f__fl(v_R.clone())?), (Py::str("pivot_columns"), { let mut out = vec![]; for t5483 in py::iter(&(v_piv.clone()))? { let mut v_p: Py; 
v_p = t5483;
out.push(py::float(&py::add(&v_p.clone(), &Py::Int(1i64))?)?); } Py::list(out) }), (Py::str("rank"), py::float(&py::len(&v_piv.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scalar_matrix_multiplication(mut v_k: Py, mut v_A: Py) -> R {
    let mut v_kk: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, Py::Int(0i64))?;
    v_kk = f__F(v_k.clone(), Py::str("k"))?;
    return Ok(Py::dict(vec![(Py::str("product"), { let mut out = vec![]; for t5538 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t5538;
out.push({ let mut out = vec![]; for t5539 in py::iter(&(v_r.clone()))? { let mut v_a: Py; 
v_a = t5539;
out.push(py::float(&py::mul(&v_kk.clone(), &f__F(v_a.clone(), Py::str("input"))?)?)?); } Py::list(out) }); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scalar_vector_multiplication(mut v_k: Py, mut v_v: Py) -> R {
    let mut v_kk: Py = Py::None;
    v_v = f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?;
    v_kk = f__F(v_k.clone(), Py::str("k"))?;
    return Ok(Py::dict(vec![(Py::str("product"), { let mut out = vec![]; for t5570 in py::iter(&(f__FV(v_v.clone())?))? { let mut v_a: Py; 
v_a = t5570;
out.push(py::float(&py::mul(&v_kk.clone(), &v_a.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_secant_root(mut v_coefficients: Py, mut v_x0: Py, mut v_x1: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_step: Py = Py::None;
    v_c = f__need_poly(v_coefficients.clone(), Py::str("coefficients"))?;
    let _ = Py::tuple(vec![f__finite(v_x0.clone(), Py::str("x0"))?, f__finite(v_x1.clone(), Py::str("x1"))?]);
    if ({ let t5602 = py::len(&v_c.clone())?; Py::Bool({ let t5603 = Py::Int(2i64); if py::lt(&t5602, &t5603)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a constant polynomial has no isolated root"))));
    }
    if ({ let t5604 = v_x0.clone(); Py::Bool({ let t5605 = v_x1.clone(); if py::eq(&t5604, &t5605) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the two starting points must differ"))));
    }
    v_step = { let v_c = v_c.clone(); let v_coefficients = v_coefficients.clone(); let v_step = v_step.clone(); let v_x0 = v_x0.clone(); let v_x1 = v_x1.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_xp: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_fp: Py = Py::None;
            let mut v_fx: Py = Py::None;
            let t5607 = (Py::tuple(vec![f__peval(v_c.clone(), v_x.clone())?, f__peval(v_c.clone(), v_xp.clone())?])).items()?;
            if t5607.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_fx = t5607[0].clone();
            v_fp = t5607[1].clone();
            if ({ let t5608 = v_fx.clone(); Py::Bool({ let t5609 = v_fp.clone(); if py::eq(&t5608, &t5609) { true } else { false } }) }).truthy() {
                return Ok((if ({ let t5610 = v_fx.clone(); Py::Bool({ let t5611 = Py::Int(0i64); if py::eq(&t5610, &t5611) { true } else { false } }) }).truthy() { Py::None } else { f__raise(Py::str("p(x0) = p(x1): the secant line is horizontal"))? }));
            }
            return Ok(py::sub(&v_x.clone(), &py::truediv(&py::mul(&v_fx.clone(), &py::sub(&v_x.clone(), &v_xp.clone())?)?, &py::sub(&v_fx.clone(), &v_fp.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    return Ok(Py::dict(vec![(Py::str("root"), f__newton_like(v_c.clone(), v_step.clone(), py::float(&v_x1.clone())?, py::float(&v_x0.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_simpson_rule(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t5690 = (f__table(v_x.clone(), v_y.clone(), Py::Int(3i64), Py::Bool(true), Py::Bool(true))?).items()?;
    if t5690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t5690[0].clone();
    v_Y = t5690[1].clone();
    if ({ let t5691 = py::modulo(&py::len(&v_X.clone())?, &Py::Int(2i64))?; Py::Bool({ let t5692 = Py::Int(0i64); if py::eq(&t5691, &t5692) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule needs an odd number of points (an even number of intervals)"))));
    }
    v_h = py::sub(&py::getitem(&v_X.clone(), &Py::Int(1i64))?, &py::getitem(&v_X.clone(), &Py::Int(0i64))?)?;
    if (Py::Bool(({ let mut out = vec![]; for t5693 in py::range_iter(&[(py::sub(&py::len(&v_X.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t5693;
out.push({ let t5694 = py::sub(&py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_X.clone(), &v_i.clone())?)?; Py::Bool({ let t5695 = v_h.clone(); if py::ne(&t5694, &t5695) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Simpson's rule here needs equally spaced x values"))));
    }
    v_s = py::add(&py::add(&py::add(&py::getitem(&v_Y.clone(), &Py::Int(0i64))?, &py::getitem(&v_Y.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::mul(&Py::Int(4i64), &py::sum(&py::slice(&v_Y.clone(), Some((Py::Int(1i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?, &py::mul(&Py::Int(2i64), &py::sum(&py::slice(&v_Y.clone(), Some((Py::Int(2i64)).i()?), Some((py::neg(&Py::Int(1i64))?).i()?), Some((Py::Int(2i64)).i()?))?, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?;
    return Ok(Py::dict(vec![(Py::str("integral"), py::float(&py::truediv(&py::mul(&v_h.clone(), &v_s.clone())?, &Py::Int(3i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_singular_values(mut v_A: Py) -> R {
    let mut v_M: Py = Py::None;
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(false), Py::None, Py::None, c_MAX_DIM()?)?;
    v_M = (if ({ let t5750 = py::len(&v_A.clone())?; Py::Bool({ let t5751 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; if py::ge(&t5750, &t5751)? { true } else { false } }) }).truthy() { v_A.clone() } else { f__T(v_A.clone())? });
    return Ok(Py::dict(vec![(Py::str("singular_values"), py::getitem(&f__svd_jacobi(v_M.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solve_2x2_system(mut v_A: Py, mut v_b: Py) -> R {
    return Ok(f__cramer(v_A.clone(), v_b.clone(), Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solve_3x3_system(mut v_A: Py, mut v_b: Py) -> R {
    return Ok(f__cramer(v_A.clone(), v_b.clone(), Py::Int(3i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_solve_linear_system(mut v_A: Py, mut v_b: Py) -> R {
    v_A = f__need_mat(v_A.clone(), Py::str("A"), Py::Bool(true), Py::None, Py::None, c_MAX_DIM()?)?;
    v_b = f__need_vec(v_b.clone(), Py::str("b"), py::len(&v_A.clone())?, Py::Int(1i64), Py::None)?;
    return Ok(Py::dict(vec![(Py::str("solution"), f__fl(f__solve_F(f__FM(v_A.clone())?, f__FV(v_b.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sparse_matvec(mut v_data: Py, mut v_indices: Py, mut v_indptr: Py, mut v_x: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_X: Py = Py::None;
    v_x = f__need_vec(v_x.clone(), Py::str("x"), Py::None, Py::Int(1i64), Py::Int(50i64))?;
    v_M = py::getitem(&f_csr_to_dense(v_data.clone(), v_indices.clone(), v_indptr.clone(), py::len(&v_x.clone())?)?, &Py::str("dense"))?;
    v_X = f__FV(v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("product"), { let mut out = vec![]; for t6140 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t6140;
out.push(py::float(&py::sum(&{ let mut out = vec![]; for t6141 in py::iter(&(py::zip(&[v_r.clone(), v_X.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6142 = (t6141).items()?;
if t6142.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6142[0].clone();
v_b = t6142[1].clone();
out.push(py::mul(&f__F(v_a.clone(), Py::str("input"))?, &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_symmetric_eigenvalues(mut v_A: Py) -> R {
    v_A = f__need_symmetric(v_A.clone(), Py::str("A"))?;
    return Ok(Py::dict(vec![(Py::str("eigenvalues"), py::sorted(&py::getitem(&f__jacobi_eig(v_A.clone())?, &Py::Int(0i64))?, (Py::Bool(false)).truthy())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_symmetric_eigenvectors(mut v_A: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vecs: Py = Py::None;
    let c_vals = py::cell_new();
    v_A = f__need_symmetric(v_A.clone(), Py::str("A"))?;
    let t6319 = (f__jacobi_eig(v_A.clone())?).items()?;
    if t6319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    py::cell_set(&c_vals, t6319[0].clone());
    v_V = t6319[1].clone();
    v_order = py::sorted_key(&Py::list((py::range_iter(&[(py::len(&py::cell_get(&c_vals)?)?).i()?])).into_iter().collect::<Vec<Py>>()), &{ let v_A = v_A.clone(); let v_V = v_V.clone(); let c_vals = c_vals.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::getitem(&py::cell_get(&c_vals)?, &v_k.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    py::cell_set(&c_vals, { let mut out = vec![]; for t6320 in py::iter(&(v_order.clone()))? { let mut v_k: Py; 
v_k = t6320;
out.push(py::getitem(&py::cell_get(&c_vals)?, &v_k.clone())?); } Py::list(out) });
    v_sc = py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t6321 in py::iter(&(py::cell_get(&c_vals)?))? { let mut v_x: Py; 
v_x = t6321;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?], true)?;
    if (Py::Bool(({ let mut out = vec![]; for t6322 in py::range_iter(&[(py::sub(&py::len(&py::cell_get(&c_vals)?)?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6322;
out.push({ let t6323 = py::sub(&py::getitem(&py::cell_get(&c_vals)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&py::cell_get(&c_vals)?, &v_k.clone())?)?; Py::Bool({ let t6324 = py::mul(&Py::Float(1e-08), &v_sc.clone())?; if py::le(&t6323, &t6324)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A has a repeated eigenvalue, so its eigenvectors are not determined up to sign"))));
    }
    v_vecs = Py::list(vec![]);
    for t6325 in py::iter(&(v_order.clone()))? {
        v_k = t6325;
        v_v = { let mut out = vec![]; for t6326 in py::range_iter(&[(py::len(&v_A.clone())?).i()?]) { let mut v_r: Py; 
v_r = t6326;
out.push(py::getitem(&py::getitem(&v_V.clone(), &v_r.clone())?, &v_k.clone())?); } Py::list(out) };
        v_nv = f__norm(v_v.clone())?;
        let _ = py::method(&v_vecs.clone(), "append", &[f__sign_fix({ let mut out = vec![]; for t6335 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6335;
out.push(py::truediv(&v_x.clone(), &v_nv.clone())?); } Py::list(out) }, Py::Float(1e-09))?])?;
    }
    return Ok(Py::dict(vec![(Py::str("eigenvalues"), py::cell_get(&c_vals)?), (Py::str("eigenvectors"), v_vecs.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_trapezoid_rule(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_Y: Py = Py::None;
    let t6413 = (f__table(v_x.clone(), v_y.clone(), Py::Int(2i64), Py::Bool(true), Py::Bool(true))?).items()?;
    if t6413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_X = t6413[0].clone();
    v_Y = t6413[1].clone();
    return Ok(Py::dict(vec![(Py::str("integral"), py::float(&py::sum(&{ let mut out = vec![]; for t6414 in py::range_iter(&[(py::sub(&py::len(&v_X.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t6414;
out.push(py::truediv(&py::mul(&py::sub(&py::getitem(&v_X.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_X.clone(), &v_i.clone())?)?, &py::add(&py::getitem(&v_Y.clone(), &v_i.clone())?, &py::getitem(&v_Y.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &Py::Int(2i64))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tridiagonal_solve(mut v_sub: Py, mut v_diag: Py, mut v_sup: Py, mut v_d: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_diag = f__need_vec(v_diag.clone(), Py::str("diag"), Py::None, Py::Int(2i64), Py::Int(200i64))?;
    v_n = py::len(&v_diag.clone())?;
    let t6455 = (Py::tuple(vec![f__need_vec(v_sub.clone(), Py::str("sub"), py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(1i64), Py::None)?, f__need_vec(v_sup.clone(), Py::str("sup"), py::sub(&v_n.clone(), &Py::Int(1i64))?, Py::Int(1i64), Py::None)?, f__need_vec(v_d.clone(), Py::str("d"), v_n.clone(), Py::Int(1i64), Py::None)?])).items()?;
    if t6455.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_sub = t6455[0].clone();
    v_sup = t6455[1].clone();
    v_d = t6455[2].clone();
    let t6457 = (Py::tuple(vec![py::add(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &f__FV(v_sub.clone())?)?, f__FV(v_diag.clone())?, py::add(&f__FV(v_sup.clone())?, &Py::list(vec![py::fraction(&[Py::Int(0i64)])?]))?, f__FV(v_d.clone())?])).items()?;
    if t6457.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t6457[0].clone();
    v_b = t6457[1].clone();
    v_c = t6457[2].clone();
    v_r = t6457[3].clone();
    let t6458 = (Py::tuple(vec![py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?, py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?])).items()?;
    if t6458.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cp = t6458[0].clone();
    v_dp = t6458[1].clone();
    for t6459 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t6459;
        v_den = py::sub(&py::getitem(&v_b.clone(), &v_i.clone())?, &(if (v_i.clone()).truthy() { py::mul(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_cp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)? } else { Py::Int(0i64) }))?;
        if ({ let t6460 = v_den.clone(); Py::Bool({ let t6461 = Py::Int(0i64); if py::eq(&t6460, &t6461) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a zero pivot arises: the Thomas algorithm needs nonzero pivots (e.g. a diagonally dominant matrix)"))));
        }
        py::setitem(&v_cp.clone(), &v_i.clone(), py::truediv(&py::getitem(&v_c.clone(), &v_i.clone())?, &v_den.clone())?)?;
        py::setitem(&v_dp.clone(), &v_i.clone(), py::truediv(&py::sub(&py::getitem(&v_r.clone(), &v_i.clone())?, &(if (v_i.clone()).truthy() { py::mul(&py::getitem(&v_a.clone(), &v_i.clone())?, &py::getitem(&v_dp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)? } else { Py::Int(0i64) }))?, &v_den.clone())?)?;
    }
    v_x = py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?;
    for t6462 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_i = t6462;
        py::setitem(&v_x.clone(), &v_i.clone(), py::sub(&py::getitem(&v_dp.clone(), &v_i.clone())?, &(if ({ let t6463 = v_i.clone(); Py::Bool({ let t6464 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::lt(&t6463, &t6464)? { true } else { false } }) }).truthy() { py::mul(&py::getitem(&v_cp.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)? } else { Py::Int(0i64) }))?)?;
    }
    return Ok(Py::dict(vec![(Py::str("solution"), f__fl(v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_triple_scalar_product(mut v_u: Py, mut v_v: Py, mut v_w: Py) -> R {
    let mut v_d: Py = Py::None;
    let t6509 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::Int(3i64), Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::Int(3i64), Py::Int(1i64), Py::None)?, f__need_vec(v_w.clone(), Py::str("w"), Py::Int(3i64), Py::Int(1i64), Py::None)?])).items()?;
    if t6509.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6509[0].clone();
    v_v = t6509[1].clone();
    v_w = t6509[2].clone();
    v_d = f__det_F(Py::list(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?, f__FV(v_w.clone())?]))?;
    return Ok(Py::dict(vec![(Py::str("triple"), py::float(&v_d.clone())?), (Py::str("volume"), py::float(&py::abs(&v_d.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_unit_vector(mut v_v: Py) -> R {
    let mut v_n: Py = Py::None;
    v_v = f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?;
    v_n = py::getitem(&f_vector_norm(v_v.clone())?, &Py::str("norm"))?;
    if ({ let t6567 = v_n.clone(); Py::Bool({ let t6568 = Py::Int(0i64); if py::eq(&t6567, &t6568) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the zero vector has no direction, so it has no unit vector"))));
    }
    return Ok(Py::dict(vec![(Py::str("unit"), { let mut out = vec![]; for t6569 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t6569;
out.push(py::truediv(&v_x.clone(), &v_n.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_addition(mut v_u: Py, mut v_v: Py) -> R {
    let t6597 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t6597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6597[0].clone();
    v_v = t6597[1].clone();
    if ({ let t6598 = py::len(&v_u.clone())?; Py::Bool({ let t6599 = py::len(&v_v.clone())?; if py::ne(&t6598, &t6599) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries"))));
    }
    return Ok(Py::dict(vec![(Py::str("sum"), { let mut out = vec![]; for t6601 in py::iter(&(py::zip(&[f__FV(v_u.clone())?, f__FV(v_v.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6602 = (t6601).items()?;
if t6602.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6602[0].clone();
v_b = t6602[1].clone();
out.push(py::float(&py::add(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_norm(mut v_v: Py) -> R {
    let mut v_F: Py = Py::None;
    v_v = f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?;
    v_F = f__FV(v_v.clone())?;
    return Ok(Py::dict(vec![(Py::str("norm"), py::math1("sqrt", &py::float(&py::sum(&{ let mut out = vec![]; for t6630 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6630;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?)?), (Py::str("norm1"), py::float(&py::sum(&{ let mut out = vec![]; for t6631 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6631;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }, None)?)?), (Py::str("norm_inf"), py::float(&py::minmax(&[{ let mut out = vec![]; for t6632 in py::iter(&(v_F.clone()))? { let mut v_a: Py; 
v_a = t6632;
out.push(py::abs(&v_a.clone())?); } Py::list(out) }], true)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_projection(mut v_u: Py, mut v_v: Py) -> R {
    let mut v_U: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let t6659 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t6659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6659[0].clone();
    v_v = t6659[1].clone();
    if ({ let t6660 = py::len(&v_u.clone())?; Py::Bool({ let t6661 = py::len(&v_v.clone())?; if py::ne(&t6660, &t6661) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries"))));
    }
    let t6663 = (Py::tuple(vec![f__FV(v_u.clone())?, f__FV(v_v.clone())?])).items()?;
    if t6663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_U = t6663[0].clone();
    v_V = t6663[1].clone();
    v_vv = py::sum(&{ let mut out = vec![]; for t6664 in py::iter(&(v_V.clone()))? { let mut v_a: Py; 
v_a = t6664;
out.push(py::mul(&v_a.clone(), &v_a.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?;
    if ({ let t6665 = v_vv.clone(); Py::Bool({ let t6666 = Py::Int(0i64); if py::eq(&t6665, &t6666) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("cannot project onto the zero vector"))));
    }
    v_t = py::truediv(&py::sum(&{ let mut out = vec![]; for t6667 in py::iter(&(py::zip(&[v_U.clone(), v_V.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6668 = (t6667).items()?;
if t6668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6668[0].clone();
v_b = t6668[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?, &v_vv.clone())?;
    return Ok(Py::dict(vec![(Py::str("projection"), { let mut out = vec![]; for t6669 in py::iter(&(v_V.clone()))? { let mut v_b: Py; 
v_b = t6669;
out.push(py::float(&py::mul(&v_t.clone(), &v_b.clone())?)?); } Py::list(out) }), (Py::str("scalar_projection"), py::mul(&py::float(&v_t.clone())?, &py::math1("sqrt", &py::float(&v_vv.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vector_subtraction(mut v_u: Py, mut v_v: Py) -> R {
    let t6701 = (Py::tuple(vec![f__need_vec(v_u.clone(), Py::str("u"), Py::None, Py::Int(1i64), Py::None)?, f__need_vec(v_v.clone(), Py::str("v"), Py::None, Py::Int(1i64), Py::None)?])).items()?;
    if t6701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t6701[0].clone();
    v_v = t6701[1].clone();
    if ({ let t6702 = py::len(&v_u.clone())?; Py::Bool({ let t6703 = py::len(&v_v.clone())?; if py::ne(&t6702, &t6703) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("u and v must have the same number of entries"))));
    }
    return Ok(Py::dict(vec![(Py::str("difference"), { let mut out = vec![]; for t6705 in py::iter(&(py::zip(&[f__FV(v_u.clone())?, f__FV(v_v.clone())?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t6706 = (t6705).items()?;
if t6706.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t6706[0].clone();
v_b = t6706[1].clone();
out.push(py::float(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1049(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1095: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1050(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1093 = v_r.clone(); Py::Bool({ let t1094 = v_r.clone(); if py::eq(&t1093, &t1094) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1095 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1050(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_data: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_outer: Py = Py::None;
    let mut v_ptr: Py = Py::None;
    v_A = py::getitem(&v_i.clone(), &Py::str("A"))?;
    let t1051 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t1051.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t1051[0].clone();
    v_n = t1051[1].clone();
    let t1052 = ((if (Py::Bool(true)).truthy() { Py::tuple(vec![v_m.clone(), v_n.clone()]) } else { Py::tuple(vec![v_n.clone(), v_m.clone()]) })).items()?;
    if t1052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_outer = t1052[0].clone();
    v_inner = t1052[1].clone();
    let t1053 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("data"))?, py::getitem(&v_o.clone(), &Py::str("indices"))?, py::getitem(&v_o.clone(), &Py::str("indptr"))?])).items()?;
    if t1053.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_data = t1053[0].clone();
    v_idx = t1053[1].clone();
    v_ptr = t1053[2].clone();
    if ({ let t1068 = { let t1054 = py::len(&v_ptr.clone())?; Py::Bool({ let t1055 = py::add(&v_outer.clone(), &Py::Int(1i64))?; if py::ne(&t1054, &t1055) { true } else { false } }) }; if t1068.truthy() { t1068 } else { { let t1067 = { let t1056 = py::getitem(&v_ptr.clone(), &Py::Int(0i64))?; Py::Bool({ let t1057 = Py::Int(0i64); if py::ne(&t1056, &t1057) { true } else { false } }) }; if t1067.truthy() { t1067 } else { { let t1066 = { let t1058 = py::getitem(&v_ptr.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1059 = py::len(&v_data.clone())?; if py::ne(&t1058, &t1059) { true } else { false } }) }; if t1066.truthy() { t1066 } else { { let t1065 = { let t1060 = py::len(&v_idx.clone())?; Py::Bool({ let t1061 = py::len(&v_data.clone())?; if py::ne(&t1060, &t1061) { true } else { false } }) }; if t1065.truthy() { t1065 } else { Py::Bool(({ let mut out = vec![]; for t1062 in py::iter(&(v_data.clone()))? { let mut v_x: Py; 
v_x = t1062;
out.push({ let t1063 = v_x.clone(); Py::Bool({ let t1064 = Py::Int(0i64); if py::eq(&t1063, &t1064) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t1076 = Py::Bool(({ let mut out = vec![]; for t1069 in py::range_iter(&[(v_outer.clone()).i()?]) { let mut v_k: Py; 
v_k = t1069;
out.push({ let t1070 = py::getitem(&v_ptr.clone(), &v_k.clone())?; Py::Bool({ let t1071 = py::getitem(&v_ptr.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t1070, &t1071)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t1076.truthy() { t1076 } else { Py::Bool(({ let mut out = vec![]; for t1072 in py::iter(&(v_idx.clone()))? { let mut v_j: Py; 
v_j = t1072;
out.push(py::not(&{ let t1073 = Py::Int(0i64); Py::Bool({ let t1074 = v_j.clone(); if py::le(&t1073, &t1074)? { { let t1075 = v_inner.clone(); if py::lt(&t1074, &t1075)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    if (Py::Bool(({ let mut out = vec![]; for t1077 in py::range_iter(&[(v_outer.clone()).i()?]) { let mut v_o2: Py; 
v_o2 = t1077;
for t1078 in py::range_iter(&[(py::int(&py::getitem(&v_ptr.clone(), &v_o2.clone())?)?).i()?, (py::sub(&py::int(&py::getitem(&v_ptr.clone(), &py::add(&v_o2.clone(), &Py::Int(1i64))?)?)?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1078;
out.push({ let t1079 = py::getitem(&v_idx.clone(), &v_k.clone())?; Py::Bool({ let t1080 = py::getitem(&v_idx.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::ge(&t1079, &t1080)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(c_BAD()?);
    }
    v_M = f__expand(v_data.clone(), v_idx.clone(), v_ptr.clone(), v_outer.clone(), v_inner.clone(), Py::Bool(true))?;
    return Ok(py::minmax(&[py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1087 in py::iter(&(py::zip(&[v_M.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t1088 = (t1087).items()?;
if t1088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1088[0].clone();
v_t = t1088[1].clone();
for t1089 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1090 = (t1089).items()?;
if t1090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1090[0].clone();
v_b = t1090[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1091 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1091;
for t1092 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t1092;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?], true)?)?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("nnz"))?, &py::len(&v_data.clone())?)?)?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("density"))?, &py::truediv(&py::len(&v_data.clone())?, &py::mul(&v_m.clone(), &v_n.clone())?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_109(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t123: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_angle(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t121 = v_r.clone(); Py::Bool({ let t122 = v_r.clone(); if py::eq(&t121, &t122) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t123 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1151(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1173: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1152(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1171 = v_r.clone(); Py::Bool({ let t1172 = v_r.clone(); if py::eq(&t1171, &t1172) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1173 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1152(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1153(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1168 = v_r.clone(); Py::Bool({ let t1169 = v_r.clone(); if py::eq(&t1168, &t1169) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1170 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1153(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t1154 = v_inputs.clone(); f_csr_to_dense(py::getitem(&t1154, &Py::str("data"))?, py::getitem(&t1154, &Py::str("indices"))?, py::getitem(&t1154, &Py::str("indptr"))?, py::getitem(&t1154, &Py::str("ncols"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t1155 in py::iter(&(Py::list(vec![Py::str("dense")])))? { let mut v_k: Py; 
v_k = t1155;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1219(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1222: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_det_value(v_inputs.clone(), v_outputs.clone(), Py::str("det"))?)?;
            return Ok(Some((if ({ let t1220 = v_r.clone(); Py::Bool({ let t1221 = v_r.clone(); if py::eq(&t1220, &t1221) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1222 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1256(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1271: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_det_value(v_inputs.clone(), v_outputs.clone(), Py::str("det"))?)?;
            return Ok(Some((if ({ let t1269 = v_r.clone(); Py::Bool({ let t1270 = v_r.clone(); if py::eq(&t1269, &t1270) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1271 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1305(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1320: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_det_value(v_inputs.clone(), v_outputs.clone(), Py::str("det"))?)?;
            return Ok(Some((if ({ let t1318 = v_r.clone(); Py::Bool({ let t1319 = v_r.clone(); if py::eq(&t1318, &t1319) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1320 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1344(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1352: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1345(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1350 = v_r.clone(); Py::Bool({ let t1351 = v_r.clone(); if py::eq(&t1350, &t1351) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1352 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1345(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("distance"))?, &Py::Int(2i64))?, &v_s.clone())?)?, &py::minmax(&[Py::Float(1.0), v_s.clone()], true)?)?, &(if ({ let t1346 = py::getitem(&v_o.clone(), &Py::str("distance"))?; Py::Bool({ let t1347 = Py::Int(0i64); if py::ge(&t1346, &t1347)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? }))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::float(&py::sum(&{ let mut out = vec![]; for t1348 in py::iter(&(py::zip(&[f__FV(py::getitem(&v_i.clone(), &Py::str("p"))?)?, f__FV(py::getitem(&v_i.clone(), &Py::str("r"))?)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1349 = (t1348).items()?;
if t1349.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1349[0].clone();
v_b = t1349[1].clone();
out.push(py::pow(&py::sub(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?); } Py::list(out) }, Some(py::fraction(&[Py::Int(0i64)])?))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1376(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1380: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1377(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1378 = v_r.clone(); Py::Bool({ let t1379 = v_r.clone(); if py::eq(&t1378, &t1379) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1380 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1377(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dot"))?, &py::getitem(&f_dot_product(py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?)?, &Py::str("dot"))?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&f_dot_product(py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?)?, &Py::str("dot"))?)?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1425(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1444: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_eig2(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1442 = v_r.clone(); Py::Bool({ let t1443 = v_r.clone(); if py::eq(&t1442, &t1443) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1444 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1520: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1506(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1518 = v_r.clone(); Py::Bool({ let t1519 = v_r.clone(); if py::eq(&t1518, &t1519) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1520 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1506(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("integral"))?, &v_w.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_w.clone())?, py::mul(&py::mul(&py::sub(&py::getitem(&v_i.clone(), &Py::str("b"))?, &py::getitem(&v_i.clone(), &Py::str("a"))?)?, &py::minmax(&[{ let mut out = vec![]; for t1507 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("coefficients"))?))? { let mut v_c: Py; 
v_c = t1507;
out.push(py::abs(&v_c.clone())?); } Py::list(out) }], true)?)?, &py::pow(&py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_i.clone(), &Py::str("a"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("b"))?)?], true)?, &py::sub(&py::len(&py::getitem(&v_i.clone(), &Py::str("coefficients"))?)?, &Py::Int(1i64))?)?)?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__gl_check(py::getitem(&v_i.clone(), &Py::str("coefficients"))?, py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::int(&py::getitem(&v_i.clone(), &Py::str("n"))?)?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1552(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_gl(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1565 = v_r.clone(); Py::Bool({ let t1566 = v_r.clone(); if py::eq(&t1565, &t1566) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1567 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1667(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1681: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_solution(v_inputs.clone(), v_outputs.clone(), Py::str("solution"))?)?;
            return Ok(Some((if ({ let t1679 = v_r.clone(); Py::Bool({ let t1680 = v_r.clone(); if py::eq(&t1679, &t1680) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1681 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1794(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1820: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_general_eig(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1818 = v_r.clone(); Py::Bool({ let t1819 = v_r.clone(); if py::eq(&t1818, &t1819) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1820 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1888(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1919: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_gs(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1917 = v_r.clone(); Py::Bool({ let t1918 = v_r.clone(); if py::eq(&t1917, &t1918) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1919 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1979(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1991: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_1980(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1989 = v_r.clone(); Py::Bool({ let t1990 = v_r.clone(); if py::eq(&t1989, &t1990) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1991 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1980(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t1981 = py::len(&py::getitem(&v_o.clone(), &Py::str("coefficients"))?)?; Py::Bool({ let t1982 = py::len(&py::getitem(&v_i.clone(), &Py::str("x"))?)?; if py::eq(&t1981, &t1982) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t1983 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("y"))?])?))? { let mut v_t: Py; let mut v_yy: Py; 
let t1984 = (t1983).items()?;
if t1984.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t1984[0].clone();
v_yy = t1984[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&py::fsum(&{ let mut out = vec![]; for t1985 in py::iter(&(py::enumerate(&py::getitem(&v_o.clone(), &Py::str("coefficients"))?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t1986 = (t1985).items()?;
if t1986.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1986[0].clone();
v_c = t1986[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_t.clone(), &v_k.clone())?)?); } Py::list(out) })?, &v_yy.clone())?)?, &py::minmax(&[Py::Float(1.0), py::fsum(&{ let mut out = vec![]; for t1987 in py::iter(&(py::enumerate(&py::getitem(&v_o.clone(), &Py::str("coefficients"))?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t1988 = (t1987).items()?;
if t1988.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1988[0].clone();
v_c = t1988[1].clone();
out.push(py::abs(&py::mul(&v_c.clone(), &py::pow(&v_t.clone(), &v_k.clone())?)?)?); } Py::list(out) })?], true)?)?); } Py::list(out) }], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_217(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t237: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_218(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t235 = v_r.clone(); Py::Bool({ let t236 = v_r.clone(); if py::eq(&t235, &t236) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t237 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_218(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__root_residual(v_i.clone(), v_o.clone(), Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2181(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2199: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2182(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2197 = v_r.clone(); Py::Bool({ let t2198 = v_r.clone(); if py::eq(&t2197, &t2198) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2199 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2182(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_edge: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mods: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2183 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("eigenvalue"))?, py::getitem(&v_o.clone(), &Py::str("eigenvector"))?])).items()?;
    if t2183.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2183[0].clone();
    v_lam = t2183[1].clone();
    v_x = t2183[2].clone();
    if ({ let t2191 = { let t2184 = py::len(&v_x.clone())?; Py::Bool({ let t2185 = py::len(&v_A.clone())?; if py::ne(&t2184, &t2185) { true } else { false } }) }; if t2191.truthy() { t2191 } else { { let t2190 = py::not(&f__sign_ok(v_x.clone(), Py::Float(1e-09))?); if t2190.truthy() { t2190 } else { { let t2188 = py::abs(&py::sub(&f__norm(v_x.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t2189 = Py::Float(1e-12); if py::gt(&t2188, &t2189)? { true } else { false } }) } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    v_res = py::truediv(&f__norm({ let mut out = vec![]; for t2192 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_x.clone())?, v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2193 = (t2192).items()?;
if t2193.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2193[0].clone();
v_b = t2193[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_lam.clone(), &v_b.clone())?)?); } Py::list(out) })?, &v_sc.clone())?;
    v_mods = { let mut out = vec![]; for t2194 in py::iter(&(f__eig_qr(v_A.clone())?))? { let mut v_z: Py; 
v_z = t2194;
out.push(py::abs(&py::complex(&[v_z.clone()])?)?); } Py::list(out) };
    v_edge = (if ({ let t2195 = Py::str("min"); Py::Bool({ let t2196 = Py::str("max"); if py::eq(&t2195, &t2196) { true } else { false } }) }).truthy() { py::minmax(&[v_mods.clone()], true)? } else { py::minmax(&[v_mods.clone()], false)? });
    return Ok(py::minmax(&[v_res.clone(), py::truediv(&py::abs(&py::sub(&py::abs(&v_lam.clone())?, &v_edge.clone())?)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2278(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2292: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_solution(v_inputs.clone(), v_outputs.clone(), Py::str("solution"))?)?;
            return Ok(Some((if ({ let t2290 = v_r.clone(); Py::Bool({ let t2291 = v_r.clone(); if py::eq(&t2290, &t2291) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2292 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2353: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2332(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2351 = v_r.clone(); Py::Bool({ let t2352 = v_r.clone(); if py::eq(&t2351, &t2352) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2353 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2332(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2350: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2333(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2348 = v_r.clone(); Py::Bool({ let t2349 = v_r.clone(); if py::eq(&t2348, &t2349) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2350 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t2334 = v_inputs.clone(); f_kronecker_product(py::getitem(&t2334, &Py::str("A"))?, py::getitem(&t2334, &Py::str("B"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t2335 in py::iter(&(Py::list(vec![Py::str("kron")])))? { let mut v_k: Py; 
v_k = t2335;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2391(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2398: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2392(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2396 = v_r.clone(); Py::Bool({ let t2397 = v_r.clone(); if py::eq(&t2396, &t2397) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2398 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2392(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("value"))?, &v_w.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_w.clone())?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f__neville(py::getitem(&v_i.clone(), &Py::str("x"))?, py::getitem(&v_i.clone(), &Py::str("y"))?, f__F(py::getitem(&v_i.clone(), &Py::str("at"))?, Py::str("input"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2430(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2443: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_line(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2441 = v_r.clone(); Py::Bool({ let t2442 = v_r.clone(); if py::eq(&t2441, &t2442) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2443 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2510(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2528: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_polyfit(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2526 = v_r.clone(); Py::Bool({ let t2527 = v_r.clone(); if py::eq(&t2526, &t2527) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2528 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2620(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2634: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_lstsq(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2632 = v_r.clone(); Py::Bool({ let t2633 = v_r.clone(); if py::eq(&t2632, &t2633) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2634 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2688(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2710: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2689(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2708 = v_r.clone(); Py::Bool({ let t2709 = v_r.clone(); if py::eq(&t2708, &t2709) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2710 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2707: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2690(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2705 = v_r.clone(); Py::Bool({ let t2706 = v_r.clone(); if py::eq(&t2705, &t2706) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2707 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2690(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t2691 = v_inputs.clone(); f_linear_independence(py::getitem(&t2691, &Py::str("vectors"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t2692 in py::iter(&(Py::list(vec![Py::str("independent"), Py::str("rank")])))? { let mut v_k: Py; 
v_k = t2692;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2749(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2771: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2750(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2769 = v_r.clone(); Py::Bool({ let t2770 = v_r.clone(); if py::eq(&t2769, &t2770) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2771 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2750(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2768: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2751(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2766 = v_r.clone(); Py::Bool({ let t2767 = v_r.clone(); if py::eq(&t2766, &t2767) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2768 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t2752 = v_inputs.clone(); f_linear_interpolation(py::getitem(&t2752, &Py::str("x"))?, py::getitem(&t2752, &Py::str("y"))?, py::getitem(&t2752, &Py::str("at"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t2753 in py::iter(&(Py::list(vec![Py::str("value")])))? { let mut v_k: Py; 
v_k = t2753;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2845(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2846(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2865 = v_r.clone(); Py::Bool({ let t2866 = v_r.clone(); if py::eq(&t2865, &t2866) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2867 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2846(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2864: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_2847(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2862 = v_r.clone(); Py::Bool({ let t2863 = v_r.clone(); if py::eq(&t2862, &t2863) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2864 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2847(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t2848 = v_inputs.clone(); f_linear_system_classification(py::getitem(&t2848, &Py::str("A"))?, py::getitem(&t2848, &Py::str("b"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t2849 in py::iter(&(Py::list(vec![Py::str("solutions"), Py::str("rank_A"), Py::str("rank_augmented")])))? { let mut v_k: Py; 
v_k = t2849;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2961(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t2976: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_lowrank(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t2974 = v_r.clone(); Py::Bool({ let t2975 = v_r.clone(); if py::eq(&t2974, &t2975) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2976 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3090: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_lu(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3088 = v_r.clone(); Py::Bool({ let t3089 = v_r.clone(); if py::eq(&t3088, &t3089) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3090 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3130(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3131(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3140 = v_r.clone(); Py::Bool({ let t3141 = v_r.clone(); if py::eq(&t3140, &t3141) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3142 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3131(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let t3133 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, py::getitem(&v_o.clone(), &Py::str("sum"))?])).items()?;
    if t3133.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3133[0].clone();
    v_B = t3133[1].clone();
    v_C = t3133[2].clone();
    if ({ let t3134 = f__shape(v_C.clone())?; Py::Bool({ let t3135 = f__shape(v_A.clone())?; if py::ne(&t3134, &t3135) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3136 in py::iter(&(py::zip(&[v_A.clone(), v_B.clone(), v_C.clone()])?))? { let mut v_r: Py; let mut v_s: Py; let mut v_t: Py; 
let t3137 = (t3136).items()?;
if t3137.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r = t3137[0].clone();
v_s = t3137[1].clone();
v_t = t3137[2].clone();
for t3138 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t3139 = (t3138).items()?;
if t3139.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t3139[0].clone();
v_b = t3139[1].clone();
v_c = t3139[2].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_c.clone(), &s_3132(v_a.clone(), v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?], true)?)?); }} Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3132(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::add(&v_a.clone(), &v_b.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t328: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_315(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t326 = v_r.clone(); Py::Bool({ let t327 = v_r.clone(); if py::eq(&t326, &t327) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t328 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_315(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_B: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_c: Py = a.get(1).cloned().unwrap_or(Py::None);
            let mut v_v: Py = a.get(2).cloned().unwrap_or(Py::None);
            return Ok((if ({ let t316 = py::len(&v_c.clone())?; Py::Bool({ let t317 = py::len(&v_v.clone())?; if py::eq(&t316, &t317) { true } else { false } }) }).truthy() { py::truediv(&py::minmax(&[{ let mut out = vec![]; for t321 in py::iter(&(py::zip(&[f__mv(f__T(v_B.clone())?, v_c.clone())?, v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t322 = (t321).items()?;
if t322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t322[0].clone();
v_b = t322[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::add(&py::mul(&f__fro(v_B.clone())?, &f__norm(v_c.clone())?)?, &f__norm(v_v.clone())?)?], true)?)? } else { c_BAD()? }));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&v_i.clone(), &Py::str("basis"))?, py::getitem(&v_o.clone(), &Py::str("coordinates"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3227: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_expm(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3225 = v_r.clone(); Py::Bool({ let t3226 = v_r.clone(); if py::eq(&t3225, &t3226) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3227 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3289(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3314: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_inverse(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3312 = v_r.clone(); Py::Bool({ let t3313 = v_r.clone(); if py::eq(&t3312, &t3313) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3314 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3375: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_matmul(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3373 = v_r.clone(); Py::Bool({ let t3374 = v_r.clone(); if py::eq(&t3373, &t3374) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3375 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3464: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3458(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3462 = v_r.clone(); Py::Bool({ let t3463 = v_r.clone(); if py::eq(&t3462, &t3463) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3464 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3458(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_w: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_s2: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("frobenius"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_w.clone(), &Py::str("frobenius"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_w.clone(), &Py::str("frobenius"))?, &Py::Int(2i64))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("norm1"))?, py::getitem(&v_w.clone(), &Py::str("norm1"))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("norm_inf"))?, py::getitem(&v_w.clone(), &Py::str("norm_inf"))?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("norm2"))?, v_s2.clone())?], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_matrix_norms(py::getitem(&v_i.clone(), &Py::str("A"))?)?, py::getitem(&py::getitem(&f__svd_jacobi((if ({ let t3459 = py::len(&py::getitem(&v_i.clone(), &Py::str("A"))?)?; Py::Bool({ let t3460 = py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("A"))?, &Py::Int(0i64))?)?; if py::lt(&t3459, &t3460)? { true } else { false } }) }).truthy() { f__T(py::getitem(&v_i.clone(), &Py::str("A"))?)? } else { { let mut out = vec![]; for t3461 in py::iter(&(py::getitem(&v_i.clone(), &Py::str("A"))?))? { let mut v_r: Py; 
v_r = t3461;
out.push(py::slice(&v_r.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?); } Py::list(out) } }))?, &Py::Int(1i64))?, &Py::Int(0i64))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3520(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3545: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3521(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3543 = v_r.clone(); Py::Bool({ let t3544 = v_r.clone(); if py::eq(&t3543, &t3544) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3545 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3521(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_power(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3540 = v_r.clone(); Py::Bool({ let t3541 = v_r.clone(); if py::eq(&t3540, &t3541) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3597(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3619: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3598(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3617 = v_r.clone(); Py::Bool({ let t3618 = v_r.clone(); if py::eq(&t3617, &t3618) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3619 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3616: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3599(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3614 = v_r.clone(); Py::Bool({ let t3615 = v_r.clone(); if py::eq(&t3614, &t3615) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3616 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3599(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t3600 = v_inputs.clone(); f_matrix_rank(py::getitem(&t3600, &Py::str("A"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3601 in py::iter(&(Py::list(vec![Py::str("rank")])))? { let mut v_k: Py; 
v_k = t3601;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3710(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3737: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_sqrtm(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3735 = v_r.clone(); Py::Bool({ let t3736 = v_r.clone(); if py::eq(&t3735, &t3736) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3737 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3777(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3789: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3778(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3787 = v_r.clone(); Py::Bool({ let t3788 = v_r.clone(); if py::eq(&t3787, &t3788) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3789 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3778(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let t3780 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_i.clone(), &Py::str("B"))?, py::getitem(&v_o.clone(), &Py::str("difference"))?])).items()?;
    if t3780.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3780[0].clone();
    v_B = t3780[1].clone();
    v_C = t3780[2].clone();
    if ({ let t3781 = f__shape(v_C.clone())?; Py::Bool({ let t3782 = f__shape(v_A.clone())?; if py::ne(&t3781, &t3782) { true } else { false } }) }).truthy() {
        return Ok(c_BAD()?);
    }
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3783 in py::iter(&(py::zip(&[v_A.clone(), v_B.clone(), v_C.clone()])?))? { let mut v_r: Py; let mut v_s: Py; let mut v_t: Py; 
let t3784 = (t3783).items()?;
if t3784.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_r = t3784[0].clone();
v_s = t3784[1].clone();
v_t = t3784[2].clone();
for t3785 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t3786 = (t3785).items()?;
if t3786.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t3786[0].clone();
v_b = t3786[1].clone();
v_c = t3786[2].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_c.clone(), &s_3779(v_a.clone(), v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?], true)?)?); }} Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3779(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(py::sub(&v_a.clone(), &v_b.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_382(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t407: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_charpoly(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t405 = v_r.clone(); Py::Bool({ let t406 = v_r.clone(); if py::eq(&t405, &t406) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t407 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3822(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3826: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3823(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3824 = v_r.clone(); Py::Bool({ let t3825 = v_r.clone(); if py::eq(&t3824, &t3825) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3826 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3823(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("trace"))?, &v_t.clone())?)?, &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::getitem(&f_matrix_trace(py::getitem(&v_i.clone(), &Py::str("A"))?)?, &Py::str("trace"))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3860(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3869: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3861(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3867 = v_r.clone(); Py::Bool({ let t3868 = v_r.clone(); if py::eq(&t3867, &t3868) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3869 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3861(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t3862 = f__shape(py::getitem(&v_o.clone(), &Py::str("transpose"))?)?; Py::Bool({ let t3863 = Py::tuple(vec![py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("A"))?, &Py::Int(0i64))?)?, py::len(&py::getitem(&v_i.clone(), &Py::str("A"))?)?]); if py::eq(&t3862, &t3863) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3865 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("A"))?)?).i()?]) { let mut v_r: Py; 
v_r = t3865;
for t3866 in py::range_iter(&[(py::len(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("A"))?, &Py::Int(0i64))?)?).i()?]) { let mut v_c: Py; 
v_c = t3866;
out.push(py::truediv(&py::abs(&py::sub(&py::getitem(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("transpose"))?, &v_c.clone())?, &v_r.clone())?, &py::getitem(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("A"))?, &v_r.clone())?, &v_c.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("A"))?, &v_r.clone())?, &v_c.clone())?)?], true)?)?); }} Py::list(out) }], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3923(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3934: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3924(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3932 = v_r.clone(); Py::Bool({ let t3933 = v_r.clone(); if py::eq(&t3932, &t3933) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3934 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3924(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t3925 = py::len(&py::getitem(&v_o.clone(), &Py::str("image"))?)?; Py::Bool({ let t3926 = py::len(&py::getitem(&v_i.clone(), &Py::str("A"))?)?; if py::eq(&t3925, &t3926) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t3927 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("image"))?])?))? { let mut v_r: Py; let mut v_y: Py; 
let t3928 = (t3927).items()?;
if t3928.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t3928[0].clone();
v_y = t3928[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_y.clone(), &f__dot(v_r.clone(), py::getitem(&v_i.clone(), &Py::str("x"))?)?)?)?, &py::minmax(&[Py::Float(1.0), py::mul(&f__norm(v_r.clone())?, &f__norm(py::getitem(&v_i.clone(), &Py::str("x"))?)?)?], true)?)?); } Py::list(out) }], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3971(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3993: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3972(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3991 = v_r.clone(); Py::Bool({ let t3992 = v_r.clone(); if py::eq(&t3991, &t3992) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3993 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3972(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t3990: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_3973(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t3988 = v_r.clone(); Py::Bool({ let t3989 = v_r.clone(); if py::eq(&t3988, &t3989) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3990 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3973(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t3974 = v_inputs.clone(); f_midpoint_rule(py::getitem(&t3974, &Py::str("coefficients"))?, py::getitem(&t3974, &Py::str("a"))?, py::getitem(&t3974, &Py::str("b"))?, py::getitem(&t3974, &Py::str("n"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t3975 in py::iter(&(Py::list(vec![Py::str("integral")])))? { let mut v_k: Py; 
v_k = t3975;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4060(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4082: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4061(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4080 = v_r.clone(); Py::Bool({ let t4081 = v_r.clone(); if py::eq(&t4080, &t4081) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4082 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4061(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4079: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4062(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4077 = v_r.clone(); Py::Bool({ let t4078 = v_r.clone(); if py::eq(&t4077, &t4078) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4079 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4062(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4063 = v_inputs.clone(); f_minor_cofactor(py::getitem(&t4063, &Py::str("A"))?, py::getitem(&t4063, &Py::str("i"))?, py::getitem(&t4063, &Py::str("j"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4064 in py::iter(&(Py::list(vec![Py::str("minor"), Py::str("cofactor")])))? { let mut v_k: Py; 
v_k = t4064;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4156(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4175 = v_r.clone(); Py::Bool({ let t4176 = v_r.clone(); if py::eq(&t4175, &t4176) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4156(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4174: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4157(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4172 = v_r.clone(); Py::Bool({ let t4173 = v_r.clone(); if py::eq(&t4172, &t4173) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4174 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4157(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4158 = v_inputs.clone(); f_natural_cubic_spline(py::getitem(&t4158, &Py::str("x"))?, py::getitem(&t4158, &Py::str("y"))?, py::getitem(&t4158, &Py::str("at"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4159 in py::iter(&(Py::list(vec![Py::str("value")])))? { let mut v_k: Py; 
v_k = t4159;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4212(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4234: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4213(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4232 = v_r.clone(); Py::Bool({ let t4233 = v_r.clone(); if py::eq(&t4232, &t4233) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4234 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4213(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4231: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4214(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4229 = v_r.clone(); Py::Bool({ let t4230 = v_r.clone(); if py::eq(&t4229, &t4230) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4231 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4214(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4215 = v_inputs.clone(); f_divided_differences(py::getitem(&t4215, &Py::str("x"))?, py::getitem(&t4215, &Py::str("y"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4216 in py::iter(&(Py::list(vec![Py::str("coefficients")])))? { let mut v_k: Py; 
v_k = t4216;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4310: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4307(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4308 = v_r.clone(); Py::Bool({ let t4309 = v_r.clone(); if py::eq(&t4308, &t4309) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4310 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4307(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__root_residual(v_i.clone(), v_o.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4370(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4385: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_null(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4383 = v_r.clone(); Py::Bool({ let t4384 = v_r.clone(); if py::eq(&t4383, &t4384) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4385 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4409(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4431: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4410(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4429 = v_r.clone(); Py::Bool({ let t4430 = v_r.clone(); if py::eq(&t4429, &t4430) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4431 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4410(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4428: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4411(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4426 = v_r.clone(); Py::Bool({ let t4427 = v_r.clone(); if py::eq(&t4426, &t4427) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4428 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4411(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4412 = v_inputs.clone(); f_numerical_derivative(py::getitem(&t4412, &Py::str("y"))?, py::getitem(&t4412, &Py::str("h"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4413 in py::iter(&(Py::list(vec![Py::str("derivative")])))? { let mut v_k: Py; 
v_k = t4413;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4454(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4463: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4455(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4461 = v_r.clone(); Py::Bool({ let t4462 = v_r.clone(); if py::eq(&t4461, &t4462) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4463 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4455(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t4456 = f__shape(py::getitem(&v_o.clone(), &Py::str("outer"))?)?; Py::Bool({ let t4457 = Py::tuple(vec![py::len(&py::getitem(&v_i.clone(), &Py::str("u"))?)?, py::len(&py::getitem(&v_i.clone(), &Py::str("v"))?)?]); if py::eq(&t4456, &t4457) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t4459 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("u"))?)?).i()?]) { let mut v_r: Py; 
v_r = t4459;
for t4460 in py::range_iter(&[(py::len(&py::getitem(&v_i.clone(), &Py::str("v"))?)?).i()?]) { let mut v_c: Py; 
v_c = t4460;
out.push(py::truediv(&py::abs(&py::sub(&py::getitem(&py::getitem(&py::getitem(&v_o.clone(), &Py::str("outer"))?, &v_r.clone())?, &v_c.clone())?, &py::mul(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("u"))?, &v_r.clone())?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("v"))?, &v_c.clone())?)?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&py::getitem(&v_i.clone(), &Py::str("u"))?, &v_r.clone())?, &py::getitem(&py::getitem(&v_i.clone(), &Py::str("v"))?, &v_c.clone())?)?)?], true)?)?); }} Py::list(out) }], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4497: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4492(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4495 = v_r.clone(); Py::Bool({ let t4496 = v_r.clone(); if py::eq(&t4495, &t4496) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4497 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4492(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::call(&{ let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::minmax(&[py::truediv(&py::abs(&py::sub(&py::pow(&py::getitem(&v_o.clone(), &Py::str("area"))?, &Py::Int(2i64))?, &py::pow(&py::getitem(&v_r.clone(), &Py::str("area"))?, &Py::Int(2i64))?)?)?, &py::minmax(&[Py::Float(1.0), py::pow(&py::getitem(&v_r.clone(), &Py::str("area"))?, &Py::Int(2i64))?], true)?)?, f__rel(py::getitem(&v_o.clone(), &Py::str("triangle_area"))?, py::truediv(&py::getitem(&v_o.clone(), &Py::str("area"))?, &Py::Int(2i64))?)?, (if ({ let t4493 = py::getitem(&v_o.clone(), &Py::str("area"))?; Py::Bool({ let t4494 = Py::Int(0i64); if py::ge(&t4493, &t4494)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? })], true)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[f_parallelogram_area(py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_poisson1d(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4533 = v_r.clone(); Py::Bool({ let t4534 = v_r.clone(); if py::eq(&t4533, &t4534) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4595(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4616: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_poisson2d(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4614 = v_r.clone(); Py::Bool({ let t4615 = v_r.clone(); if py::eq(&t4614, &t4615) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4616 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_466(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t493: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_chol(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t491 = v_r.clone(); Py::Bool({ let t492 = v_r.clone(); if py::eq(&t491, &t492) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t493 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4712(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4753: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_polar(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4751 = v_r.clone(); Py::Bool({ let t4752 = v_r.clone(); if py::eq(&t4751, &t4752) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4753 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4779(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4801: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4780(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4799 = v_r.clone(); Py::Bool({ let t4800 = v_r.clone(); if py::eq(&t4799, &t4800) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4801 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4780(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4798: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4781(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4796 = v_r.clone(); Py::Bool({ let t4797 = v_r.clone(); if py::eq(&t4796, &t4797) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4798 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4781(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4782 = v_inputs.clone(); f_polynomial_evaluation(py::getitem(&t4782, &Py::str("coefficients"))?, py::getitem(&t4782, &Py::str("x"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4783 in py::iter(&(Py::list(vec![Py::str("value"), Py::str("derivative")])))? { let mut v_k: Py; 
v_k = t4783;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4879: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4858(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4877 = v_r.clone(); Py::Bool({ let t4878 = v_r.clone(); if py::eq(&t4877, &t4878) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4879 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t4876: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_4859(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t4874 = v_r.clone(); Py::Bool({ let t4875 = v_r.clone(); if py::eq(&t4874, &t4875) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4876 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4859(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t4860 = v_inputs.clone(); f_positive_definiteness(py::getitem(&t4860, &Py::str("A"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t4861 in py::iter(&(Py::list(vec![Py::str("positive_definite"), Py::str("leading_minors")])))? { let mut v_k: Py; 
v_k = t4861;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5053(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5071: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5054(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5069 = v_r.clone(); Py::Bool({ let t5070 = v_r.clone(); if py::eq(&t5069, &t5070) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5071 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5054(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_edge: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_mods: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t5055 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("eigenvalue"))?, py::getitem(&v_o.clone(), &Py::str("eigenvector"))?])).items()?;
    if t5055.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5055[0].clone();
    v_lam = t5055[1].clone();
    v_x = t5055[2].clone();
    if ({ let t5063 = { let t5056 = py::len(&v_x.clone())?; Py::Bool({ let t5057 = py::len(&v_A.clone())?; if py::ne(&t5056, &t5057) { true } else { false } }) }; if t5063.truthy() { t5063 } else { { let t5062 = py::not(&f__sign_ok(v_x.clone(), Py::Float(1e-09))?); if t5062.truthy() { t5062 } else { { let t5060 = py::abs(&py::sub(&f__norm(v_x.clone())?, &Py::Int(1i64))?)?; Py::Bool({ let t5061 = Py::Float(1e-12); if py::gt(&t5060, &t5061)? { true } else { false } }) } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    v_sc = py::minmax(&[Py::Float(1.0), f__fro(v_A.clone())?], true)?;
    v_res = py::truediv(&f__norm({ let mut out = vec![]; for t5064 in py::iter(&(py::zip(&[f__mv(v_A.clone(), v_x.clone())?, v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5065 = (t5064).items()?;
if t5065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5065[0].clone();
v_b = t5065[1].clone();
out.push(py::sub(&v_a.clone(), &py::mul(&v_lam.clone(), &v_b.clone())?)?); } Py::list(out) })?, &v_sc.clone())?;
    v_mods = { let mut out = vec![]; for t5066 in py::iter(&(f__eig_qr(v_A.clone())?))? { let mut v_z: Py; 
v_z = t5066;
out.push(py::abs(&py::complex(&[v_z.clone()])?)?); } Py::list(out) };
    v_edge = (if ({ let t5067 = Py::str("max"); Py::Bool({ let t5068 = Py::str("max"); if py::eq(&t5067, &t5068) { true } else { false } }) }).truthy() { py::minmax(&[v_mods.clone()], true)? } else { py::minmax(&[v_mods.clone()], false)? });
    return Ok(py::minmax(&[v_res.clone(), py::truediv(&py::abs(&py::sub(&py::abs(&v_lam.clone())?, &v_edge.clone())?)?, &v_sc.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5140(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5165: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_projmat(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5163 = v_r.clone(); Py::Bool({ let t5164 = v_r.clone(); if py::eq(&t5163, &t5164) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5165 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5237(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5267: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_pinv(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5265 = v_r.clone(); Py::Bool({ let t5266 = v_r.clone(); if py::eq(&t5265, &t5266) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5267 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5361(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5399: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_qr(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5397 = v_r.clone(); Py::Bool({ let t5398 = v_r.clone(); if py::eq(&t5397, &t5398) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5399 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5420(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5429: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_rotation(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5427 = v_r.clone(); Py::Bool({ let t5428 = v_r.clone(); if py::eq(&t5427, &t5428) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5429 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5484(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5506: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5485(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5504 = v_r.clone(); Py::Bool({ let t5505 = v_r.clone(); if py::eq(&t5504, &t5505) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5506 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5485(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5486(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5501 = v_r.clone(); Py::Bool({ let t5502 = v_r.clone(); if py::eq(&t5501, &t5502) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5486(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t5487 = v_inputs.clone(); f_rref(py::getitem(&t5487, &Py::str("A"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t5488 in py::iter(&(Py::list(vec![Py::str("rref"), Py::str("pivot_columns"), Py::str("rank")])))? { let mut v_k: Py; 
v_k = t5488;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5540(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5551: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5541(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5549 = v_r.clone(); Py::Bool({ let t5550 = v_r.clone(); if py::eq(&t5549, &t5550) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5551 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5541(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t5542 = f__shape(py::getitem(&v_o.clone(), &Py::str("product"))?)?; Py::Bool({ let t5543 = f__shape(py::getitem(&v_i.clone(), &Py::str("A"))?)?; if py::eq(&t5542, &t5543) { true } else { false } }) }).truthy() { py::minmax(&[{ let mut out = vec![]; for t5545 in py::iter(&(py::zip(&[py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("product"))?])?))? { let mut v_r: Py; let mut v_s: Py; 
let t5546 = (t5545).items()?;
if t5546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t5546[0].clone();
v_s = t5546[1].clone();
for t5547 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_p: Py; 
let t5548 = (t5547).items()?;
if t5548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5548[0].clone();
v_p = t5548[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_p.clone(), &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &v_a.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &v_a.clone())?)?], true)?)?); }} Py::list(out) }], true)? } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5571(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5572(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5577 = v_r.clone(); Py::Bool({ let t5578 = v_r.clone(); if py::eq(&t5577, &t5578) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5572(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t5573 = py::len(&py::getitem(&v_o.clone(), &Py::str("product"))?)?; Py::Bool({ let t5574 = py::len(&py::getitem(&v_i.clone(), &Py::str("v"))?)?; if py::eq(&t5573, &t5574) { true } else { false } }) }).truthy() { { let t = { let mut out = vec![]; for t5575 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("product"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])?))? { let mut v_a: Py; let mut v_p: Py; 
let t5576 = (t5575).items()?;
if t5576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t5576[0].clone();
v_a = t5576[1].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_p.clone(), &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &v_a.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::abs(&py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &v_a.clone())?)?], true)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } } } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5656(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5660: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5657(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5658 = v_r.clone(); Py::Bool({ let t5659 = v_r.clone(); if py::eq(&t5658, &t5659) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5660 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5657(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__root_residual(v_i.clone(), v_o.clone(), Py::None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_566(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t588: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_567(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t586 = v_r.clone(); Py::Bool({ let t587 = v_r.clone(); if py::eq(&t586, &t587) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t588 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_567(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t585: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_568(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t583 = v_r.clone(); Py::Bool({ let t584 = v_r.clone(); if py::eq(&t583, &t584) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t585 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_568(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t569 = v_inputs.clone(); f_clamped_cubic_spline(py::getitem(&t569, &Py::str("x"))?, py::getitem(&t569, &Py::str("y"))?, py::getitem(&t569, &Py::str("slope_start"))?, py::getitem(&t569, &Py::str("slope_end"))?, py::getitem(&t569, &Py::str("at"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t570 in py::iter(&(Py::list(vec![Py::str("value")])))? { let mut v_k: Py; 
v_k = t570;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5696(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5697(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5716 = v_r.clone(); Py::Bool({ let t5717 = v_r.clone(); if py::eq(&t5716, &t5717) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5718 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5697(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5698(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5713 = v_r.clone(); Py::Bool({ let t5714 = v_r.clone(); if py::eq(&t5713, &t5714) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t5699 = v_inputs.clone(); f_simpson_rule(py::getitem(&t5699, &Py::str("x"))?, py::getitem(&t5699, &Py::str("y"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t5700 in py::iter(&(Py::list(vec![Py::str("integral")])))? { let mut v_k: Py; 
v_k = t5700;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5792(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5822: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_singular(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5820 = v_r.clone(); Py::Bool({ let t5821 = v_r.clone(); if py::eq(&t5820, &t5821) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5822 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5893(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5908: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5894(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5906 = v_r.clone(); Py::Bool({ let t5907 = v_r.clone(); if py::eq(&t5906, &t5907) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5908 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5894(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__verify_solution(v_i.clone(), v_o.clone(), Py::str("solution"))?, f__verify_det_value(v_i.clone(), v_o.clone(), Py::str("det"))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_59(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t74: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_adjugate(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t72 = v_r.clone(); Py::Bool({ let t73 = v_r.clone(); if py::eq(&t72, &t73) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t74 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5979(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t5994: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_5980(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t5992 = v_r.clone(); Py::Bool({ let t5993 = v_r.clone(); if py::eq(&t5992, &t5993) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5994 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5980(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__verify_solution(v_i.clone(), v_o.clone(), Py::str("solution"))?, f__verify_det_value(v_i.clone(), v_o.clone(), Py::str("det"))?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6069(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_solution(v_inputs.clone(), v_outputs.clone(), Py::str("solution"))?)?;
            return Ok(Some((if ({ let t6081 = v_r.clone(); Py::Bool({ let t6082 = v_r.clone(); if py::eq(&t6081, &t6082) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6165: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6144(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6163 = v_r.clone(); Py::Bool({ let t6164 = v_r.clone(); if py::eq(&t6163, &t6164) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6165 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6162: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6145(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6160 = v_r.clone(); Py::Bool({ let t6161 = v_r.clone(); if py::eq(&t6160, &t6161) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6162 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6145(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t6146 = v_inputs.clone(); f_sparse_matvec(py::getitem(&t6146, &Py::str("data"))?, py::getitem(&t6146, &Py::str("indices"))?, py::getitem(&t6146, &Py::str("indptr"))?, py::getitem(&t6146, &Py::str("x"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t6147 in py::iter(&(Py::list(vec![Py::str("product")])))? { let mut v_k: Py; 
v_k = t6147;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6232(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6252: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6233(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6250 = v_r.clone(); Py::Bool({ let t6251 = v_r.clone(); if py::eq(&t6250, &t6251) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6252 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6233(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::minmax(&[f__verify_spectrum(py::getitem(&v_i.clone(), &Py::str("A"))?, py::getitem(&v_o.clone(), &Py::str("eigenvalues"))?)?, (if ({ let t6248 = py::getitem(&v_o.clone(), &Py::str("eigenvalues"))?; Py::Bool({ let t6249 = py::sorted(&py::getitem(&v_o.clone(), &Py::str("eigenvalues"))?, (Py::Bool(false)).truthy())?; if py::eq(&t6248, &t6249) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_BAD()? })], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6336(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6383: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_eigvecs(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6381 = v_r.clone(); Py::Bool({ let t6382 = v_r.clone(); if py::eq(&t6381, &t6382) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6383 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6415(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6437: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6416(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6435 = v_r.clone(); Py::Bool({ let t6436 = v_r.clone(); if py::eq(&t6435, &t6436) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6437 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6416(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6417(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6432 = v_r.clone(); Py::Bool({ let t6433 = v_r.clone(); if py::eq(&t6432, &t6433) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6417(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t6418 = v_inputs.clone(); f_trapezoid_rule(py::getitem(&t6418, &Py::str("x"))?, py::getitem(&t6418, &Py::str("y"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t6419 in py::iter(&(Py::list(vec![Py::str("integral")])))? { let mut v_k: Py; 
v_k = t6419;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_643(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_644(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t663 = v_r.clone(); Py::Bool({ let t664 = v_r.clone(); if py::eq(&t663, &t664) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_644(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t662: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_645(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t660 = v_r.clone(); Py::Bool({ let t661 = v_r.clone(); if py::eq(&t660, &t661) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t662 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_645(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t646 = v_inputs.clone(); f_column_space(py::getitem(&t646, &Py::str("A"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t647 in py::iter(&(Py::list(vec![Py::str("dimension"), Py::str("basis")])))? { let mut v_k: Py; 
v_k = t647;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6466(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_tridiag(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6489 = v_r.clone(); Py::Bool({ let t6490 = v_r.clone(); if py::eq(&t6489, &t6490) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6491 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6523(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6545: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6524(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6543 = v_r.clone(); Py::Bool({ let t6544 = v_r.clone(); if py::eq(&t6543, &t6544) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6545 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6524(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6542: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6525(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6540 = v_r.clone(); Py::Bool({ let t6541 = v_r.clone(); if py::eq(&t6540, &t6541) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6542 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6525(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    v_want = { let t6526 = v_inputs.clone(); f_triple_scalar_product(py::getitem(&t6526, &Py::str("u"))?, py::getitem(&t6526, &Py::str("v"))?, py::getitem(&t6526, &Py::str("w"))?)? };
    return Ok(py::minmax(&[{ let mut out = vec![]; for t6527 in py::iter(&(Py::list(vec![Py::str("triple"), Py::str("volume")])))? { let mut v_k: Py; 
v_k = t6527;
out.push(f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, py::getitem(&v_want.clone(), &v_k.clone())?)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6570(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6579: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_unit(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6577 = v_r.clone(); Py::Bool({ let t6578 = v_r.clone(); if py::eq(&t6577, &t6578) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6579 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6603(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6611: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6604(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6609 = v_r.clone(); Py::Bool({ let t6610 = v_r.clone(); if py::eq(&t6609, &t6610) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6611 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6604(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t6605 = py::len(&py::getitem(&v_o.clone(), &Py::str("sum"))?)?; Py::Bool({ let t6606 = py::len(&py::getitem(&v_i.clone(), &Py::str("u"))?)?; if py::eq(&t6605, &t6606) { true } else { false } }) }).truthy() { { let t = { let mut out = vec![]; for t6607 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("sum"))?, py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_s: Py; 
let t6608 = (t6607).items()?;
if t6608.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_s = t6608[0].clone();
v_a = t6608[1].clone();
v_b = t6608[2].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_s.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?], true)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } } } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6633(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6641: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_norm(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6639 = v_r.clone(); Py::Bool({ let t6640 = v_r.clone(); if py::eq(&t6639, &t6640) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6641 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6670(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6683: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_projection(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6681 = v_r.clone(); Py::Bool({ let t6682 = v_r.clone(); if py::eq(&t6681, &t6682) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6683 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6707(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t6715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_6708(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t6713 = v_r.clone(); Py::Bool({ let t6714 = v_r.clone(); if py::eq(&t6713, &t6714) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6715 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6708(mut v_i: Py, mut v_o: Py) -> R {
    return Ok((if ({ let t6709 = py::len(&py::getitem(&v_o.clone(), &Py::str("difference"))?)?; Py::Bool({ let t6710 = py::len(&py::getitem(&v_i.clone(), &Py::str("u"))?)?; if py::eq(&t6709, &t6710) { true } else { false } }) }).truthy() { { let t = { let mut out = vec![]; for t6711 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("difference"))?, py::getitem(&v_i.clone(), &Py::str("u"))?, py::getitem(&v_i.clone(), &Py::str("v"))?])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_s: Py; 
let t6712 = (t6711).items()?;
if t6712.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_s = t6712[0].clone();
v_a = t6712[1].clone();
v_b = t6712[2].clone();
out.push(py::truediv(&py::abs(&py::sub(&v_s.clone(), &py::sub(&v_a.clone(), &v_b.clone())?)?)?, &py::minmax(&[Py::Float(1.0), py::add(&py::abs(&v_a.clone())?, &py::abs(&v_b.clone())?)?], true)?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } } } else { c_BAD()? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_781(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t790: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_cond(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t788 = v_r.clone(); Py::Bool({ let t789 = v_r.clone(); if py::eq(&t788, &t789) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t790 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_887(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t895: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_solution(v_inputs.clone(), v_outputs.clone(), Py::str("solution"))?)?;
            return Ok(Some((if ({ let t893 = v_r.clone(); Py::Bool({ let t894 = v_r.clone(); if py::eq(&t893, &t894) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t895 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_916(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t925: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&f__verify_cross(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t923 = v_r.clone(); Py::Bool({ let t924 = v_r.clone(); if py::eq(&t923, &t924) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t925 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1010: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_r = py::float(&s_965(v_inputs.clone(), v_outputs.clone())?)?;
            return Ok(Some((if ({ let t1008 = v_r.clone(); Py::Bool({ let t1009 = v_r.clone(); if py::eq(&t1008, &t1009) { true } else { false } }) }).truthy() { v_r.clone() } else { c_BAD()? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1010 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_BAD()?);
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_965(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_data: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_inner: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_outer: Py = Py::None;
    let mut v_ptr: Py = Py::None;
    v_A = py::getitem(&v_i.clone(), &Py::str("A"))?;
    let t966 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t966.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_m = t966[0].clone();
    v_n = t966[1].clone();
    let t967 = ((if (Py::Bool(false)).truthy() { Py::tuple(vec![v_m.clone(), v_n.clone()]) } else { Py::tuple(vec![v_n.clone(), v_m.clone()]) })).items()?;
    if t967.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_outer = t967[0].clone();
    v_inner = t967[1].clone();
    let t968 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("data"))?, py::getitem(&v_o.clone(), &Py::str("indices"))?, py::getitem(&v_o.clone(), &Py::str("indptr"))?])).items()?;
    if t968.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_data = t968[0].clone();
    v_idx = t968[1].clone();
    v_ptr = t968[2].clone();
    if ({ let t983 = { let t969 = py::len(&v_ptr.clone())?; Py::Bool({ let t970 = py::add(&v_outer.clone(), &Py::Int(1i64))?; if py::ne(&t969, &t970) { true } else { false } }) }; if t983.truthy() { t983 } else { { let t982 = { let t971 = py::getitem(&v_ptr.clone(), &Py::Int(0i64))?; Py::Bool({ let t972 = Py::Int(0i64); if py::ne(&t971, &t972) { true } else { false } }) }; if t982.truthy() { t982 } else { { let t981 = { let t973 = py::getitem(&v_ptr.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t974 = py::len(&v_data.clone())?; if py::ne(&t973, &t974) { true } else { false } }) }; if t981.truthy() { t981 } else { { let t980 = { let t975 = py::len(&v_idx.clone())?; Py::Bool({ let t976 = py::len(&v_data.clone())?; if py::ne(&t975, &t976) { true } else { false } }) }; if t980.truthy() { t980 } else { Py::Bool(({ let mut out = vec![]; for t977 in py::iter(&(v_data.clone()))? { let mut v_x: Py; 
v_x = t977;
out.push({ let t978 = v_x.clone(); Py::Bool({ let t979 = Py::Int(0i64); if py::eq(&t978, &t979) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } } } } } }).truthy() {
        return Ok(c_BAD()?);
    }
    if ({ let t991 = Py::Bool(({ let mut out = vec![]; for t984 in py::range_iter(&[(v_outer.clone()).i()?]) { let mut v_k: Py; 
v_k = t984;
out.push({ let t985 = py::getitem(&v_ptr.clone(), &v_k.clone())?; Py::Bool({ let t986 = py::getitem(&v_ptr.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t985, &t986)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if t991.truthy() { t991 } else { Py::Bool(({ let mut out = vec![]; for t987 in py::iter(&(v_idx.clone()))? { let mut v_j: Py; 
v_j = t987;
out.push(py::not(&{ let t988 = Py::Int(0i64); Py::Bool({ let t989 = v_j.clone(); if py::le(&t988, &t989)? { { let t990 = v_inner.clone(); if py::lt(&t989, &t990)? { true } else { false } } } else { false } }) })); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Ok(c_BAD()?);
    }
    if (Py::Bool(({ let mut out = vec![]; for t992 in py::range_iter(&[(v_outer.clone()).i()?]) { let mut v_o2: Py; 
v_o2 = t992;
for t993 in py::range_iter(&[(py::int(&py::getitem(&v_ptr.clone(), &v_o2.clone())?)?).i()?, (py::sub(&py::int(&py::getitem(&v_ptr.clone(), &py::add(&v_o2.clone(), &Py::Int(1i64))?)?)?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t993;
out.push({ let t994 = py::getitem(&v_idx.clone(), &v_k.clone())?; Py::Bool({ let t995 = py::getitem(&v_idx.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::ge(&t994, &t995)? { true } else { false } }) }); }} Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Ok(c_BAD()?);
    }
    v_M = f__expand(v_data.clone(), v_idx.clone(), v_ptr.clone(), v_outer.clone(), v_inner.clone(), Py::Bool(false))?;
    return Ok(py::minmax(&[py::truediv(&py::minmax(&[{ let mut out = vec![]; for t1002 in py::iter(&(py::zip(&[v_M.clone(), v_A.clone()])?))? { let mut v_r: Py; let mut v_t: Py; 
let t1003 = (t1002).items()?;
if t1003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t1003[0].clone();
v_t = t1003[1].clone();
for t1004 in py::iter(&(py::zip(&[v_r.clone(), v_t.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t1005 = (t1004).items()?;
if t1005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t1005[0].clone();
v_b = t1005[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); }} Py::list(out) }], true)?, &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t1006 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1006;
for t1007 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t1007;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)?], true)?)?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("nnz"))?, &py::len(&v_data.clone())?)?)?, py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("density"))?, &py::truediv(&py::len(&v_data.clone())?, &py::mul(&v_m.clone(), &v_n.clone())?)?)?)?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
