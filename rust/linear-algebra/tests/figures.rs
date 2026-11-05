//! Every Linear Algebra capability's own live figures (src/figures.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use linear_algebra_ai999x::pack;

/// The published figure kinds the model draws itself (publish/website/models/linear-algebra-ai999x/figures, except
/// the generic "curve" and the kinds the generic engine draws, GENERIC below), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("bisection_root", &["interval"]),
    ("change_of_basis", &["matrix", "vectors"]),
    ("characteristic_polynomial", &["matrix"]),
    ("clamped_cubic_spline", &["xy"]),
    ("condition_number", &["matrix"]),
    ("conjugate_gradient", &["matrix", "vectors"]),
    ("csc_format", &["matrix"]),
    ("csr_format", &["matrix"]),
    ("determinant", &["matrix"]),
    ("determinant_2x2", &["matrix"]),
    ("determinant_3x3", &["matrix"]),
    ("eigenvalues_2x2", &["matrix"]),
    ("gauss_seidel", &["matrix", "vectors"]),
    ("general_eigenvalues", &["complex", "matrix"]),
    ("interpolating_polynomial", &["xy"]),
    ("inverse_iteration", &["matrix"]),
    ("jacobi_iteration", &["matrix", "vectors"]),
    ("lagrange_interpolation", &["xy"]),
    ("least_squares_line", &["xy"]),
    ("least_squares_polynomial", &["fit", "xy"]),
    ("least_squares_solution", &["matrix"]),
    ("linear_independence", &["matrix"]),
    ("linear_interpolation", &["xy"]),
    ("linear_system_classification", &["matrix"]),
    ("lu_decomposition", &["heatmap"]),
    ("matrix_norms", &["matrix"]),
    ("matrix_rank", &["matrix"]),
    ("matrix_trace", &["matrix"]),
    ("matrix_vector_product", &["matrix", "vectors"]),
    ("minor_cofactor", &["matrix"]),
    ("natural_cubic_spline", &["curve-2", "xy"]),
    ("newton_divided_differences", &["xy"]),
    ("positive_definiteness", &["matrix"]),
    ("power_iteration", &["matrix"]),
    ("rotate_vector_2d", &["vectors"]),
    ("simpson_rule", &["xy"]),
    ("singular_values", &["map", "matrix"]),
    ("solve_2x2_system", &["matrix", "vectors"]),
    ("solve_3x3_system", &["matrix"]),
    ("solve_linear_system", &["matrix", "vectors"]),
    ("sparse_matvec", &["io"]),
    ("symmetric_eigenvalues", &["complex", "matrix"]),
    ("symmetric_eigenvectors", &["complex", "map"]),
    ("trapezoid_rule", &["xy"]),
    ("triple_scalar_product", &["io"]),
    ("unit_vector", &["vectors"]),
    ("vector_projection", &["vectors"]),
    ("vector_subtraction", &["vectors"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("minor_cofactor", "A", "[[2, 1, 3], [4, 6, 5], [7, 8, 9]]"),
    ("bisection_root", "coefficients", "1, 0, -3"),
    ("linear_system_classification", "b", "2, 4"),
    ("linear_independence", "vectors", "1 0 1; 0 1 1; 1 1 0"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // ("infinite" is a word of some titles: check_figure looks for inf as a number)
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a singular system is refused by the model: no figure at all; a vector in space gets no plane-vector figure
    let p = pack();
    let given: Vec<(String, Input)> = [("A", "[[1, 2], [2, 4]]"), ("b", "1, 2")].iter()
        .map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "solve_linear_system", &given, None);
    let figs = figures(p, "solve_linear_system", &given, None, None);
    if !a.solved() {
        assert!(figs.is_empty(), "a refused input drew a figure");
    }
    let given: Vec<(String, Input)> = [("u", "1, 2, 3"), ("v", "0, 1, 1")].iter()
        .map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let figs = figures(p, "vector_projection", &given, None, None);
    assert!(figs.iter().all(|f| f.kind != "vectors" || f.slug.contains("-live-")), "a 3-vector drawn as a plane vector");
    for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
        check_texts(f);
    }
}

/// The published kinds the generic engine already draws for these capabilities: still drawn beside the own figures.
const GENERIC: &[(&str, &str)] = &[
    ("adjugate", "matrix"), ("angle_between_vectors", "sensitivity"), ("characteristic_polynomial", "series"),
    ("cholesky", "matrix"), ("column_space", "matrix"), ("cross_product", "series"), ("csr_format", "series"),
    ("csr_to_dense", "matrix"), ("distance_between_points", "sensitivity"), ("dot_product", "sensitivity"),
    ("gauss_legendre_rule", "series"), ("gram_schmidt", "matrix"), ("interpolating_polynomial", "series"),
    ("kronecker_product", "matrix"), ("least_squares_polynomial", "series"), ("low_rank_approximation", "matrix"),
    ("lu_decomposition", "matrix"), ("matrix_addition", "matrix"), ("matrix_exponential", "matrix"),
    ("matrix_inverse", "matrix"), ("matrix_multiplication", "matrix"), ("matrix_power", "matrix"),
    ("matrix_square_root", "matrix"), ("matrix_subtraction", "matrix"), ("matrix_transpose", "matrix"),
    ("newton_divided_differences", "series"), ("newton_root", "sensitivity"), ("null_space", "matrix"),
    ("numerical_derivative", "series"), ("outer_product", "matrix"), ("parallelogram_area", "sensitivity"),
    ("poisson_1d", "series"), ("poisson_2d", "matrix"), ("polar_decomposition", "matrix"), ("projection_matrix", "matrix"),
    ("pseudoinverse", "matrix"), ("qr_decomposition", "matrix"), ("rref", "matrix"),
    ("scalar_matrix_multiplication", "matrix"), ("scalar_vector_multiplication", "series"), ("secant_root", "sensitivity"),
    ("solve_3x3_system", "series"), ("symmetric_eigenvectors", "matrix"), ("tridiagonal_solve", "series"),
    ("vector_addition", "series"), ("vector_norm", "sensitivity"),
];

#[test]
fn generic_kinds_stay() {
    let p = pack();
    let mut lost = vec![];
    for (key, kind) in GENERIC {
        let figs = figures(p, key, &example(key), None, None);
        if !figs.iter().any(|f| f.kind == *kind) {
            lost.push(format!("{key}: {kind}"));
        }
    }
    assert!(lost.is_empty(), "published kinds no longer drawn: {lost:?}");
}

/// Polynomial coefficients for the model's evaluation are highest power first (the figures rely on it).
#[test]
fn polynomial_evaluation_takes_the_highest_power_first() {
    let p = pack();
    let given: Vec<(String, Input)> = vec![("coefficients".into(), Input::Text("1, 0, -2".into())),
                                           ("x".into(), Input::Text("3".into()))];
    let a = ax_engine::solve(p, "polynomial_evaluation", &given, None);
    let v = a.values.iter().find(|v| v.0 == "value").and_then(|v| v.1.as_f64()).unwrap();
    assert!((v - 7.0).abs() < 1e-12, "{v}");
}

#[test]
fn own_figures_time() {
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        let t = std::time::Instant::now();
        let _ = figures(p, key, &given, None, None);
        eprintln!("TIME {key}: {} ms", t.elapsed().as_millis());
    }
}
