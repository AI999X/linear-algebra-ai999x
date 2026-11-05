//! Linear Algebra's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: the input matrices beside the returned matrices and vectors (each panel shaded by its own largest
//! magnitude), plane vectors as arrows, eigenvalues in the complex plane, data with the interpolant or fit the model
//! returns (evaluated by further runs of the model), a matrix acting on the unit circle and the unit square (every
//! image point a run of the model's matrix-vector product), bisection brackets each solved by the model. A figure
//! the model cannot support for these inputs (a vector that is not a plane vector, a refused run) is left out, never
//! drawn wrong (docs/FIGURE_PARITY.md).

use ax_engine::figkit::{self as k, num, Canvas, DPlot, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use serde_json::Value;

#[path = "figures_xy.rs"]
mod xy;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // the matrices of the request: inputs (and the returned matrix or vector) side by side
        "change_of_basis" => vec![mats(s, &["basis", "v"], &["coordinates"], &[], "Coordinates of a vector in a basis"),
                                  vectors(s, &["v"], &["coordinates"], "Coordinates of a vector in a basis")],
        "characteristic_polynomial" => vec![mats(s, &["A"], &["coefficients"], &[],
                                                 "Characteristic polynomial of a matrix")],
        "condition_number" => vec![mats(s, &["A"], &[], &["cond2", "cond1"], "Condition number of a matrix")],
        "conjugate_gradient" | "gauss_seidel" | "jacobi_iteration" | "solve_linear_system" => {
            let t = match s.key {
                "conjugate_gradient" => "Conjugate gradient method for Ax = b",
                "gauss_seidel" => "Gauss-Seidel iteration for Ax = b",
                "jacobi_iteration" => "Jacobi iteration for Ax = b",
                _ => "A square linear system Ax = b",
            };
            vec![mats(s, &["A", "b"], &["solution"], &[], t), vectors(s, &["b"], &["solution"], t)]
        }
        "solve_2x2_system" => vec![mats(s, &["A", "b"], &["solution"], &["det"], "Two linear equations in two unknowns"),
                                   vectors(s, &["b"], &["solution"], "Two linear equations in two unknowns")],
        "solve_3x3_system" => vec![mats(s, &["A", "b"], &["solution"], &["det"],
                                        "Three linear equations in three unknowns")],
        "csc_format" => vec![mats(s, &["A"], &[], &["nnz", "density"], "Compressed sparse column form of a matrix")],
        "csr_format" => vec![mats(s, &["A"], &[], &["nnz", "density"], "Compressed sparse row form of a matrix")],
        "determinant" | "determinant_2x2" | "determinant_3x3" => vec![mats(s, &["A"], &[], &["det"],
                                                                           "Determinant of a matrix")],
        "eigenvalues_2x2" => vec![mats(s, &["A"], &[], &["lambda1", "lambda2"], "Eigenvalues of a 2 x 2 matrix")],
        "general_eigenvalues" => vec![mats(s, &["A"], &["eigenvalues"], &["spectral_radius"],
                                           "Eigenvalues of a general square matrix"),
                                      eig_plane(s, "Eigenvalues of a general square matrix")],
        "inverse_iteration" => vec![mats(s, &["A"], &["eigenvector"], &["eigenvalue"],
                                         "Smallest eigenvalue by inverse iteration")],
        "power_iteration" => vec![mats(s, &["A"], &["eigenvector"], &["eigenvalue"],
                                       "Dominant eigenvalue by power iteration")],
        "least_squares_solution" => vec![mats(s, &["A", "b"], &["solution"], &["residual_norm"],
                                              "Least-squares solution of an overdetermined system")],
        "linear_independence" => vec![mats(s, &["vectors"], &[], &["independent", "rank"],
                                           "Linear independence of vectors (one per row)")],
        "linear_system_classification" => vec![mats(s, &["A", "b"], &[], &["solutions", "rank_A", "rank_augmented"],
                                                    "Number of solutions of a linear system")],
        "matrix_norms" => vec![mats(s, &["A"], &[], &["frobenius", "norm1", "norm_inf", "norm2"], "Norms of a matrix")],
        "matrix_rank" => vec![mats(s, &["A"], &[], &["rank"], "Rank of a matrix")],
        "matrix_trace" => vec![mats(s, &["A"], &[], &["trace"], "Trace of a square matrix")],
        "matrix_vector_product" => vec![mats(s, &["A", "x"], &["image"], &[], "Matrix times vector"),
                                        vectors(s, &["x"], &["image"], "Matrix times vector")],
        "minor_cofactor" => vec![mats(s, &["A"], &[], &["minor", "cofactor"], "Minor and cofactor of an entry")],
        "positive_definiteness" => vec![mats(s, &["A"], &["leading_minors"], &["positive_definite"],
                                             "Positive definiteness test (leading minors)")],
        "singular_values" => vec![mats(s, &["A"], &["singular_values"], &[], "Singular values of a matrix"),
                                  xy::circle_map(s)],
        "symmetric_eigenvalues" => vec![mats(s, &["A"], &["eigenvalues"], &[], "Eigenvalues of a symmetric matrix"),
                                        eig_plane(s, "Eigenvalues of a symmetric matrix")],
        "symmetric_eigenvectors" => vec![eig_plane(s, "Eigenvalues of a symmetric matrix"), xy::square_map(s)],
        "lu_decomposition" => vec![lu_panels(s)],
        // plane vectors
        "rotate_vector_2d" => vec![vectors(s, &["v"], &["rotated"], "Rotation of a plane vector")],
        "unit_vector" => vec![vectors(s, &["v"], &["unit"], "Unit vector in the direction of a vector")],
        "vector_projection" => vec![vectors(s, &["u", "v"], &["projection"], "Projection of one vector onto another")],
        "vector_subtraction" => vec![vectors(s, &["u", "v"], &["difference"], "Difference of two vectors")],
        // inputs and results
        "sparse_matvec" => vec![io(s, &["data", "indices", "indptr", "x"], &["product"],
                                   "Sparse matrix times vector (CSR)")],
        "triple_scalar_product" => vec![io(s, &["u", "v", "w"], &["triple", "volume"],
                                           "Scalar triple product (volume of a parallelepiped)")],
        // data with what the model returned drawn on it
        "clamped_cubic_spline" | "natural_cubic_spline" | "lagrange_interpolation" | "linear_interpolation" => {
            let mut v = vec![xy::interp(s)];
            if s.key == "natural_cubic_spline" {
                v.push(xy::interp_compare(s));
            }
            v
        }
        "interpolating_polynomial" | "newton_divided_differences" => vec![xy::poly_through(s)],
        "least_squares_line" => vec![xy::ls_line(s)],
        "least_squares_polynomial" => vec![xy::ls_poly(s, false), xy::ls_poly(s, true)],
        "simpson_rule" | "trapezoid_rule" => vec![xy::rule_area(s)],
        "bisection_root" => vec![xy::brackets(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ helpers
pub(crate) fn esc(t: &str) -> String {
    t.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// A list as the model reads it ("1, 2, 3"): every number in full precision.
pub(crate) fn vtext(v: &[f64]) -> String {
    v.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(", ")
}

/// A matrix as text ("[[1, 2], [3, 4]]").
pub(crate) fn mtext(m: &[Vec<f64>]) -> String {
    format!("[{}]", m.iter().map(|r| format!("[{}]", vtext(r))).collect::<Vec<_>>().join(", "))
}

/// A vector for a caption: "(2, 1)".
pub(crate) fn fv(v: &[f64]) -> String {
    format!("({})", v.iter().map(|x| num(k::snap(*x, 1.0))).collect::<Vec<_>>().join(", "))
}

/// A matrix output (an array of arrays of numbers).
pub(crate) fn out_mat(s: &Solved, sym: &str) -> Option<Vec<Vec<f64>>> {
    let a = s.outj(sym)?.as_array()?;
    let m: Vec<Vec<f64>> = a.iter().map(|r| r.as_array().map(|r| r.iter().filter_map(Value::as_f64).collect()))
        .collect::<Option<Vec<Vec<f64>>>>()?;
    let ok = !m.is_empty() && m.iter().all(|r| !r.is_empty() && r.len() == m[0].len() && r.iter().all(|x| x.is_finite()));
    ok.then_some(m)
}

/// A returned scalar as words ("det = 4", "independent = no").
fn scalar_text(s: &Solved, sym: &str) -> Option<String> {
    match s.outj(sym)? {
        Value::Number(n) => Some(format!("{sym} = {}", num(k::snap(n.as_f64()?, 1.0)))),
        Value::Bool(b) => Some(format!("{sym} = {}", if *b { "yes" } else { "no" })),
        Value::String(t) => Some(format!("{sym}: {t}")),
        _ => None,
    }
}

/// The matrix (or a list as a column) an input or output stands for.
fn panel_of(s: &Solved, sym: &str, input: bool) -> Option<Vec<Vec<f64>>> {
    if input {
        if let Some(m) = s.mat(sym) {
            return Some(m);
        }
        return s.vec(sym).filter(|v| v.len() >= 2).map(|v| v.into_iter().map(|x| vec![x]).collect());
    }
    if let Some(m) = out_mat(s, sym) {
        return Some(m);
    }
    s.out_list(sym).map(|v| v.into_iter().map(|x| vec![x]).collect())
}

/// One row of heat-map panels from y0 (each panel shaded by its own largest magnitude, values written in the
/// cells); returns the height used. `mark`: (panel, row, column) of a cell outlined.
fn panel_row(c: &mut Canvas, y0: f64, panels: &[(String, Vec<Vec<f64>>)], cell: f64, mark: Option<(usize, usize, usize)>)
             -> f64 {
    let total = row_width(panels, cell);
    let mut slot_x = (520.0 - total) / 2.0 + 8.0;
    let rmax = panels.iter().map(|p| p.1.len()).max().unwrap_or(1) as f64;
    let font = (cell * 0.3).clamp(8.0, 14.0);
    for (pi, (name, m)) in panels.iter().enumerate() {
        let (r, cc) = (m.len(), m[0].len());
        let slot = slot_w(name, cc, cell);
        let x = slot_x + (slot - cc as f64 * cell) / 2.0;
        slot_x += slot + PANEL_GAP;
        let top = y0 + 16.0 + (rmax - r as f64) * cell / 2.0;
        let mx = m.iter().flatten().fold(0.0f64, |a, v| a.max(v.abs()));
        for (j, _) in m[0].iter().enumerate() {
            c.raw(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"11\">{}</text>",
                           x + cell * (j as f64 + 0.5), top - 5.0, j + 1));
        }
        for (i, row) in m.iter().enumerate() {
            c.raw(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"11\">{}</text>",
                           x - 5.0, top + cell * (i as f64 + 0.5) + 4.0, i + 1));
            for (j, v) in row.iter().enumerate() {
                let v = k::snap(*v, mx);
                let (cx, cy) = (x + cell * j as f64, top + cell * i as f64);
                let op = if mx > 0.0 { 0.08 + 0.5 * v.abs() / mx } else { 0.10 };
                c.raw(&format!("<rect class=\"{}\" x=\"{cx:.1}\" y=\"{cy:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill-opacity=\"{op:.2}\"/>",
                               if v < 0.0 { "axp-hn" } else { "axp-hp" }, cell - 1.0, cell - 1.0));
                if cell >= 18.0 {
                    let t = ax_engine::num::py_g(v, if cell >= 44.0 { 4 } else { 3 });
                    c.raw(&format!("<text class=\"axp-ht\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"{font:.0}\">{}</text>",
                                   cx + cell / 2.0, cy + cell / 2.0 + font * 0.35, esc(&t)));
                }
            }
        }
        if let Some((p, i, j)) = mark {
            if p == pi && i < r && j < cc {
                c.raw(&format!("<rect class=\"axp-k\" style=\"stroke-width:3\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                               x + cell * j as f64, top + cell * i as f64, cell - 1.0, cell - 1.0));
                // the row and the column struck out for the minor
                c.raw(&format!("<rect class=\"axp-k\" stroke-dasharray=\"4 3\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>\
                                <rect class=\"axp-k\" stroke-dasharray=\"4 3\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                               x, top + cell * i as f64, cell * cc as f64 - 1.0, cell - 1.0,
                               x + cell * j as f64, top, cell - 1.0, cell * r as f64 - 1.0));
            }
        }
        let w = cc as f64 * cell;
        c.text(x + w / 2.0, y0 + 16.0 + rmax * cell + 18.0, "middle", "axp-bt", name);
    }
    16.0 + rmax * cell + 28.0
}

const PANEL_GAP: f64 = 26.0;

/// The width a panel takes: its cells, or its name under it when that is wider.
fn slot_w(name: &str, cols: usize, cell: f64) -> f64 {
    (cols as f64 * cell).max(name.chars().count() as f64 * 7.6 + 6.0)
}

fn row_width(panels: &[(String, Vec<Vec<f64>>)], cell: f64) -> f64 {
    panels.iter().map(|p| slot_w(&p.0, p.1[0].len(), cell)).sum::<f64>() + PANEL_GAP * (panels.len() as f64 - 1.0)
}

/// The cell size for a row of panels: fits the width (names under narrow panels included) and at most `hmax` tall.
fn cell_for(panels: &[(String, Vec<Vec<f64>>)], hmax: f64) -> f64 {
    let rows = panels.iter().map(|p| p.1.len()).max().unwrap_or(1) as f64;
    let mut cell = (hmax / rows).min(62.0);
    while cell > 8.0 && row_width(panels, cell) > 480.0 {
        cell -= 1.0;
    }
    cell
}

/// The matrix figure: the inputs `ins` (matrices, or lists as columns) and the returned `outs` as heat-map panels,
/// returned scalars in the heading.
fn mats(s: &Solved, ins: &[&str], outs: &[&str], scalars: &[&str], what: &str) -> Option<Figure> {
    let mut panels: Vec<(String, Vec<Vec<f64>>)> = vec![];
    for sym in ins {
        if let Some(m) = panel_of(s, sym, true) {
            panels.push((format!("{sym} (input)"), m));
        }
    }
    if panels.is_empty() {
        return None;
    }
    let n_in = panels.len();
    for sym in outs {
        if let Some(m) = panel_of(s, sym, false) {
            panels.push((format!("{sym} (returned)"), m));
        }
    }
    if panels.iter().any(|p| p.1.len() > 12 || p.1[0].len() > 12) {
        return None;
    }
    let sc: Vec<String> = scalars.iter().filter_map(|x| scalar_text(s, x)).collect();
    let header = sc.join(", ");
    let mark = if s.key == "minor_cofactor" {
        let int = |sym: &str| -> Option<i64> {
            match s.val(sym)? {
                ax_engine::pack::Val::Int(i) => Some(i),
                ax_engine::pack::Val::Num(x) if x.fract() == 0.0 => Some(x as i64),
                _ => None,
            }
        };
        let (i, j) = (int("i")?, int("j")?);
        (i >= 1 && j >= 1).then(|| (0usize, i as usize - 1, j as usize - 1))
    } else {
        None
    };
    // the returned scalars as heading lines of at most 54 characters (two lines at most)
    let mut lines: Vec<String> = vec![];
    for t in &sc {
        match lines.last_mut() {
            Some(l) if l.chars().count() + 2 + t.chars().count() <= 54 => {
                l.push_str(", ");
                l.push_str(t);
            }
            _ => lines.push(t.clone()),
        }
    }
    lines.truncate(2);
    let _ = header;
    let cell = cell_for(&panels, 230.0 - 20.0 * lines.len().saturating_sub(1) as f64);
    let mut c = Canvas::new(330.0);
    for (i, l) in lines.iter().enumerate() {
        c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"{}\">{}</text>", 22 + 20 * i, esc(&clip(l, 54))));
    }
    let y0 = 40.0 + 20.0 * lines.len().saturating_sub(1) as f64;
    let used = panel_row(&mut c, y0, &panels, cell, mark);
    c.h = (y0 + used + 8.0).max(160.0);
    let names: Vec<String> = panels.iter().map(|p| p.0.clone()).collect();
    let mut cap = format!("The matrices of the request as heat maps (rows and columns numbered, each cell shaded by its \
                           magnitude, negative values in the second colour, each panel shaded by its own largest \
                           magnitude): {}.", names.join("; "));
    if panels.len() > n_in {
        let ret: Vec<String> = panels[n_in..].iter().map(|p| {
            let flat: Vec<f64> = p.1.iter().flatten().copied().collect();
            format!("{} = {}", p.0.trim_end_matches(" (returned)"), if p.1[0].len() == 1 { fv(&flat) } else { mtext_short(&p.1) })
        }).collect();
        cap.push_str(&format!(" Returned by the model: {}.", ret.join("; ")));
    }
    if !sc.is_empty() {
        cap.push_str(&format!(" Also returned: {}.", sc.join(", ")));
    }
    if let Some((_, i, j)) = mark {
        cap.push_str(&format!(" Outlined: entry ({}, {}); dashed: its row and column, struck out for the minor.", i + 1, j + 1));
    }
    Some(s.figure("matrix", &format!("{what}: the matrices"), &cap, c.body(), c.h))
}

fn mtext_short(m: &[Vec<f64>]) -> String {
    format!("[{}]", m.iter().map(|r| format!("[{}]", r.iter().map(|x| num(k::snap(*x, 1.0))).collect::<Vec<_>>().join(", ")))
        .collect::<Vec<_>>().join(", "))
}

pub(crate) fn clip(t: &str, n: usize) -> String {
    let c: Vec<char> = t.chars().collect();
    if c.len() <= n { t.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>()) }
}

/// LU factorisation: A, P, L and U as heat-map panels, two rows of two.
fn lu_panels(s: &Solved) -> Option<Figure> {
    let a = s.mat("A")?;
    let (p, l, u) = (out_mat(s, "P")?, out_mat(s, "L")?, out_mat(s, "U")?);
    if a.len() > 8 {
        return None;
    }
    let top = vec![("A (input)".to_string(), a.clone()), ("P (returned)".to_string(), p.clone())];
    let bottom = vec![("L (returned)".to_string(), l.clone()), ("U (returned)".to_string(), u.clone())];
    let cell = cell_for(&top, 150.0).min(cell_for(&bottom, 150.0));
    let mut c = Canvas::new(600.0);
    c.raw("<text class=\"axp-mt\" x=\"16\" y=\"22\">A = P L U</text>");
    let h1 = panel_row(&mut c, 34.0, &top, cell, None);
    let h2 = panel_row(&mut c, 34.0 + h1, &bottom, cell, None);
    c.h = 34.0 + h1 + h2 + 6.0;
    let cap = format!("The LU factorisation with partial pivoting returned by the model for A = {}: A = P L U with P a \
                       permutation ({}), L unit lower triangular ({}) and U upper triangular ({}); cells shaded by \
                       magnitude, negative values in the second colour.", mtext_short(&a), mtext_short(&p),
                      mtext_short(&l), mtext_short(&u));
    Some(s.figure("heatmap", "LU factorisation as matrices", &cap, c.body(), c.h))
}

/// Plane vectors as arrows from the origin: the inputs `ins` (green, violet) and the returned `outs` (accent); None
/// unless every one has two entries.
fn vectors(s: &Solved, ins: &[&str], outs: &[&str], what: &str) -> Option<Figure> {
    let mut iv: Vec<(String, Vec<f64>)> = vec![];
    for sym in ins {
        let v = s.vec(sym)?;
        iv.push((sym.to_string(), v));
    }
    let mut ov: Vec<(String, Vec<f64>)> = vec![];
    for sym in outs {
        ov.push((sym.to_string(), s.out_list(sym)?));
    }
    if iv.iter().chain(ov.iter()).any(|v| v.1.len() != 2) {
        return None;
    }
    let mut xs = vec![0.0];
    let mut ys = vec![0.0];
    for v in iv.iter().chain(ov.iter()) {
        xs.push(v.1[0]);
        ys.push(v.1[1]);
    }
    let mut p = DPlot::equal(&xs, &ys, "x", "y", "");
    p.hline(0.0);
    p.vline(0.0);
    // the construction behind the answer, dashed
    let mut extra = String::new();
    match s.key {
        "vector_projection" => {
            let (u, pr) = (&iv[0].1, &ov[0].1);
            p.seg((u[0], u[1]), (pr[0], pr[1]), 3, true);
            extra = " Dashed: from the tip of u to its projection, at right angles to v.".into();
        }
        "vector_subtraction" => {
            let (u, v) = (&iv[0].1, &iv[1].1);
            p.seg((v[0], v[1]), (u[0], u[1]), 0, true);
            extra = " Dashed: the difference placed from the tip of v to the tip of u.".into();
        }
        _ => {}
    }
    // the legend with the values when it fits above the plot, else the names only (the values are in the caption)
    let long: Vec<String> = iv.iter().map(|(n, v)| format!("{n} = {} (input)", fv(v)))
        .chain(ov.iter().map(|(n, v)| format!("{n} = {} (returned)", fv(v)))).collect();
    let width: f64 = long.iter().map(|t| 7.0 * t.chars().count().min(24) as f64 + 26.0).sum();
    let short = width > 400.0 || long.iter().any(|t| t.chars().count() > 24);
    let keys: Vec<String> = if short {
        iv.iter().map(|(n, _)| format!("{n} (input)")).chain(ov.iter().map(|(n, _)| format!("{n} (returned)"))).collect()
    } else {
        long
    };
    for (kk, (_, v)) in iv.iter().enumerate() {
        let col = if kk == 0 { 1 } else { 2 };
        p.arrow((0.0, 0.0), (v[0], v[1]), col);
        p.key(&keys[kk], col);
    }
    for (kk, (_, v)) in ov.iter().enumerate() {
        p.arrow((0.0, 0.0), (v[0], v[1]), 0);
        p.dot(v[0], v[1], 1, 4.0);
        p.key(&keys[iv.len() + kk], 0);
    }
    let sc: Vec<String> = match s.key {
        "solve_2x2_system" => vec![scalar_text(s, "det")].into_iter().flatten().collect(),
        "vector_projection" => vec![scalar_text(s, "scalar_projection")].into_iter().flatten().collect(),
        "rotate_vector_2d" => s.given.iter().find(|g| g.0 == "angle")
            .and_then(|g| match &g.1 { Input::Text(t) => Some(t.clone()), Input::Float(x) => Some(num(*x)),
                                        Input::Int(i) => Some(i.to_string()), _ => None })
            .map(|a| format!("angle = {a}")).into_iter().collect(),
        _ => vec![],
    };
    let cap = format!("The plane vectors of the request as arrows from the origin: input {}; returned by the model: {}{}. \
                       Marked: the tip of each returned vector.{extra}",
                      iv.iter().map(|(n, v)| format!("{n} = {}", fv(v))).collect::<Vec<_>>().join(", "),
                      ov.iter().map(|(n, v)| format!("{n} = {}", fv(v))).collect::<Vec<_>>().join(", "),
                      if sc.is_empty() { String::new() } else { format!("; also {}", sc.join(", ")) });
    Some(s.chart("vectors", &format!("{what}: the vectors"), &cap, &p.body()))
}

/// The returned eigenvalues in the complex plane.
fn eig_plane(s: &Solved, what: &str) -> Option<Figure> {
    let z = k::cx_list(s.a, "eigenvalues")?;
    if z.len() > 12 {
        return None;
    }
    let sc = |x: f64| k::snap(x, 1.0);
    let mine: Vec<(String, (f64, f64))> = z.iter().enumerate()
        .map(|(i, z)| (format!("λ{} = {}", i + 1, cx(sc(z.0), sc(z.1))), (sc(z.0), sc(z.1)))).collect();
    let mut xs: Vec<f64> = mine.iter().map(|m| m.1 .0).collect();
    let mut ys: Vec<f64> = mine.iter().map(|m| m.1 .1).collect();
    xs.extend([0.0, 0.0]);
    ys.extend([-1.0, 1.0]);
    let header = clip(&mine.iter().map(|m| m.0.clone()).collect::<Vec<_>>().join(", "), 54);
    let mut p = DPlot::equal(&xs, &ys, "real part", "imaginary part", &header);
    p.hline(0.0);
    p.vline(0.0);
    let (_, (y0, y1)) = p.ranges();
    let dy = (y1 - y0) * 0.075;
    for (i, (_, (re, im))) in mine.iter().enumerate() {
        p.dot(*re, *im, 0, 6.0);
        // names alternate above and below, so neighbours on the real axis stay apart
        let off = if i % 2 == 0 { dy } else { -dy * 1.6 };
        p.label(*re, im + off, "middle", "axp-bt", &format!("λ{}", i + 1));
    }
    let body = p.body();
    let cap = format!("The eigenvalues returned by the model for A = {}, drawn in the complex plane (real part across, \
                       imaginary part up): {}.", s.mat("A").map(|m| mtext_short(&m)).unwrap_or_default(),
                      mine.iter().map(|m| m.0.clone()).collect::<Vec<_>>().join(", "));
    Some(s.chart("complex", &format!("{what} in the complex plane"), &cap, &body))
}

fn cx(re: f64, im: f64) -> String {
    if im == 0.0 { num(re) } else if im > 0.0 { format!("{} + {}i", num(re), num(im)) } else {
        format!("{} - {}i", num(re), num(-im))
    }
}

/// The request as inputs and results: a table.
fn io(s: &Solved, ins: &[&str], outs: &[&str], what: &str) -> Option<Figure> {
    let mut rows: Vec<Vec<String>> = vec![];
    for sym in ins {
        let v = s.vec(sym)?;
        rows.push(vec!["stated".into(), sym.to_string(), vtext_short(&v)]);
    }
    let mut res = vec![];
    for sym in outs {
        let t = match s.outj(sym)? {
            Value::Number(n) => num(n.as_f64()?),
            Value::Array(_) => vtext_short(&s.out_list(sym)?),
            _ => return None,
        };
        res.push(format!("{sym} = {t}"));
        rows.push(vec!["returned".into(), sym.to_string(), t]);
    }
    let mark = Some(ins.len());
    let (body, h) = k::table(&["".into(), "quantity".into(), "value".into()], &rows, mark, what);
    let cap = format!("The inputs and results of the request: stated {}; returned by the model: {}.",
                      ins.iter().filter_map(|x| s.vec(x).map(|v| format!("{x} = {}", vtext_short(&v))))
                          .collect::<Vec<_>>().join(", "), res.join(", "));
    Some(s.figure("io", &format!("{what}: inputs and results"), &cap, &body, h))
}

fn vtext_short(v: &[f64]) -> String {
    let t: Vec<String> = v.iter().take(8).map(|x| num(*x)).collect();
    format!("{}{}", t.join(", "), if v.len() > 8 { ", ..." } else { "" })
}

/// An input for another run: a list as text.
pub(crate) fn tin(v: &[f64]) -> Input {
    Input::Text(vtext(v))
}
