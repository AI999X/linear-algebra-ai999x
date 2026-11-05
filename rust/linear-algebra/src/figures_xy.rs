//! Linear Algebra's figures over data and in the plane: the data with the interpolant, polynomial or fit the model
//! returns (every curve point a further run of the model), the area a quadrature rule sums, bisection brackets, and a
//! 2 x 2 matrix acting on the unit circle and the unit square (every image point a run of its matrix-vector product).

use super::{fv, mtext, num, out_mat, tin, Canvas, DPlot, Solved};
use ax_engine::figkit::{self as k, num_of};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The data (x, y) as read: equal lengths, at least two points.
fn data(s: &Solved) -> Option<(Vec<f64>, Vec<f64>)> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    (x.len() == y.len() && x.len() >= 2 && x.len() <= 200).then_some((x, y))
}

fn span_of(v: &[f64]) -> (f64, f64) {
    v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)))
}

/// The y range of everything drawn (a little room when flat).
fn yrange(sets: &[&[(f64, f64)]]) -> (f64, f64) {
    let ys: Vec<f64> = sets.iter().flat_map(|s| s.iter().map(|p| p.1)).filter(|y| y.is_finite()).collect();
    let (lo, hi) = span_of(&ys);
    if hi > lo { (lo, hi) } else { (lo - 1.0, hi + 1.0) }
}

/// The value of a polynomial (coefficients highest power first) at x: a run of the model's polynomial evaluation.
fn peval(s: &Solved, hi_first: &[f64], x: f64) -> Option<f64> {
    let a = s.solve_other("polynomial_evaluation", &[("coefficients", tin(hi_first)), ("x", Input::Float(x))])?;
    num_of(&a, "value")
}

/// The Lagrange interpolant of the data at t: a run of the model's Lagrange interpolation.
fn lagrange(s: &Solved, x: &[f64], y: &[f64], t: f64) -> Option<f64> {
    let a = s.solve_other("lagrange_interpolation", &[("x", tin(x)), ("y", tin(y)), ("at", Input::Float(t))])?;
    num_of(&a, "value")
}

fn data_dots(p: &mut DPlot, x: &[f64], y: &[f64]) {
    for (a, b) in x.iter().zip(y) {
        p.dot(*a, *b, 0, 5.0);
    }
}

fn title_of(key: &str) -> &'static str {
    match key {
        "clamped_cubic_spline" => "Clamped cubic spline interpolation",
        "natural_cubic_spline" => "Natural cubic spline interpolation",
        "lagrange_interpolation" => "Lagrange interpolation",
        "linear_interpolation" => "Piecewise linear interpolation",
        "interpolating_polynomial" => "Coefficients of the interpolating polynomial",
        "newton_divided_differences" => "Newton divided differences",
        "least_squares_line" => "Least-squares regression line",
        "least_squares_polynomial" => "Least-squares polynomial fit",
        "simpson_rule" => "Simpson's rule for tabulated data",
        "trapezoid_rule" => "Trapezoidal rule for tabulated data",
        _ => "",
    }
}

/// An interpolating capability (a spline, Lagrange, piecewise linear): the data and the interpolant, each of 41
/// points a run of the model at a new `at`; the visitor's point marked.
pub fn interp(s: &Solved) -> Option<Figure> {
    let (x, y) = data(s)?;
    let (at, v) = (s.inp("at")?, s.out("value")?);
    let (lo, hi) = span_of(&x);
    let pts: Vec<(f64, f64)> = k::span(lo, hi, 41, false).into_iter()
        .filter_map(|t| s.with(&[("at", t)]).and_then(|a| num_of(&a, "value")).map(|y| (t, y))).collect();
    if pts.len() < 2 {
        return None;
    }
    let d: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let yr = yrange(&[&pts, &d, &[(at, v)]]);
    let mut p = DPlot::new((lo.min(at), hi.max(at)), yr, "x", "y", &format!("value at {} = {}", num(at), num(v)));
    p.curve(&pts, 1);
    data_dots(&mut p, &x, &y);
    p.mark(at, v, "");
    p.key("data (input)", 0);
    p.key("interpolant (41 runs)", 1);
    let what = title_of(s.key);
    let cap = format!("The data y against x as given ({} points) and the interpolant drawn through them: each of its 41 \
                       points is a run of the model at a new evaluation point. Marked: the requested point, at = {}, \
                       value = {}.", x.len(), num(at), num(v));
    Some(s.chart("xy", &format!("{what}: the data and what was returned"), &cap, &p.body()))
}

/// Natural cubic spline against two other interpolants of the same data (piecewise linear and Lagrange), each point
/// a run of the model.
pub fn interp_compare(s: &Solved) -> Option<Figure> {
    let (x, y) = data(s)?;
    let (at, v) = (s.inp("at")?, s.out("value")?);
    let (lo, hi) = span_of(&x);
    let ts = k::span(lo, hi, 31, false);
    let spline: Vec<(f64, f64)> = ts.iter()
        .filter_map(|t| s.with(&[("at", *t)]).and_then(|a| num_of(&a, "value")).map(|y| (*t, y))).collect();
    let lin: Vec<(f64, f64)> = ts.iter().filter_map(|t| {
        s.solve_other("linear_interpolation", &[("x", tin(&x)), ("y", tin(&y)), ("at", Input::Float(*t))])
            .and_then(|a| num_of(&a, "value")).map(|y| (*t, y))
    }).collect();
    if spline.len() < 2 {
        return None;
    }
    let mut series = vec![("natural spline".to_string(), spline)];
    if lin.len() >= 2 {
        series.push(("piecewise linear".to_string(), lin));
    }
    let body = k::lines(&series, "Evaluation point at", "Interpolated value", &format!("S({}) = {}", num(at), num(v)),
                        Some((at, v)), &format!("at = {}", num(at)), false);
    let cap = format!("The natural cubic spline S(at) of the data against the evaluation point from {} to {} (31 runs of \
                       the model), beside piecewise linear interpolation of the same data (31 runs). Marked: at = {}, \
                       value = {}.", num(lo), num(hi), num(at), num(v));
    Some(s.chart("curve-2", "Natural cubic spline interpolation: spline value against the point", &cap, &body))
}

/// The polynomial through the data: from the returned coefficients (lowest power first) evaluated by the model, or,
/// for divided differences, the same interpolant through the model's Lagrange interpolation. Drawn only when it
/// passes through every data point.
pub fn poly_through(s: &Solved) -> Option<Figure> {
    let (x, y) = data(s)?;
    let c = s.out_list("coefficients")?;
    let hi_first: Vec<f64> = c.iter().rev().copied().collect();
    let newton = s.key == "newton_divided_differences";
    let f = |t: f64| if newton { lagrange(s, &x, &y, t) } else { peval(s, &hi_first, t) };
    let scale = y.iter().fold(1.0f64, |a, v| a.max(v.abs()));
    for (a, b) in x.iter().zip(&y) {
        if (f(*a)? - b).abs() > 1e-6 * scale {
            return None;
        }
    }
    let (lo, hi) = span_of(&x);
    let pad = (hi - lo) * 0.08;
    let pts: Vec<(f64, f64)> = k::span(lo - pad, hi + pad, 41, false).into_iter().filter_map(|t| f(t).map(|v| (t, v))).collect();
    if pts.len() < 2 {
        return None;
    }
    let d: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let mut p = DPlot::new((lo - pad, hi + pad), yrange(&[&pts, &d]), "x", "y",
                          &super::clip(&format!("coefficients {}", fv(&c)), 50));
    p.curve(&pts, 1);
    data_dots(&mut p, &x, &y);
    p.key("data (input)", 0);
    p.key(if newton { "interpolant (model)" } else { "polynomial (returned)" }, 1);
    let how = if newton {
        format!("the returned divided differences {} define the interpolating polynomial p(t) = c0 + c1 (t - x0) + ...; \
                 it is drawn from 41 runs of the model's Lagrange interpolation of the same data, which it equals",
                fv(&c))
    } else {
        format!("the returned coefficients {} (lowest power first), the polynomial evaluated by the model at 41 points",
                fv(&c))
    };
    let cap = format!("The data y against x as given ({} points); drawn on it: {}. It passes through every data point.",
                      x.len(), how);
    Some(s.chart("xy", &format!("{}: the data and what was returned", title_of(s.key)), &cap, &p.body()))
}

/// The least-squares line from the returned slope and intercept (its ends evaluated by the model) over the data.
pub fn ls_line(s: &Solved) -> Option<Figure> {
    let (x, y) = data(s)?;
    let (m, b) = (s.out("slope")?, s.out("intercept")?);
    let r2 = s.out("r_squared");
    let (lo, hi) = span_of(&x);
    let pad = (hi - lo) * 0.05;
    let ends: Vec<(f64, f64)> = [lo - pad, hi + pad].iter().filter_map(|t| peval(s, &[m, b], *t).map(|v| (*t, v))).collect();
    if ends.len() < 2 {
        return None;
    }
    let d: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let header = format!("y = {} + {}·x{}", num(b), num(m), r2.map(|r| format!(", R² = {}", num(r))).unwrap_or_default());
    let mut p = DPlot::new((lo - pad, hi + pad), yrange(&[&ends, &d]), "x", "y", &header);
    p.curve(&ends, 1);
    data_dots(&mut p, &x, &y);
    p.key("data (input)", 0);
    p.key("returned line", 1);
    let cap = format!("The data y against x as given ({} points); drawn on it: the line y = {} + {}·x from the returned \
                       intercept and slope.{}", x.len(), num(b), num(m),
                      r2.map(|r| format!(" Also returned: r_squared = {}.", num(r))).unwrap_or_default());
    Some(s.chart("xy", "Least-squares regression line: the data and what was returned", &cap, &p.body()))
}

/// The least-squares polynomial (coefficients lowest power first, evaluated by the model) over the data; `fit`: also
/// the least-squares line of the same points (another capability of the model) for comparison.
pub fn ls_poly(s: &Solved, fit: bool) -> Option<Figure> {
    let (x, y) = data(s)?;
    let c = s.out_list("coefficients")?;
    let hi_first: Vec<f64> = c.iter().rev().copied().collect();
    let (lo, hi) = span_of(&x);
    let pad = (hi - lo) * 0.05;
    let pts: Vec<(f64, f64)> = k::span(lo - pad, hi + pad, 41, false).into_iter()
        .filter_map(|t| peval(s, &hi_first, t).map(|v| (t, v))).collect();
    if pts.len() < 2 {
        return None;
    }
    let deg = c.len() - 1;
    let d: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let poly_txt = poly_words(&c);
    if !fit {
        let mut p = DPlot::new((lo - pad, hi + pad), yrange(&[&pts, &d]), "x", "y", &super::clip(&format!("y = {poly_txt}"), 54));
        p.curve(&pts, 1);
        data_dots(&mut p, &x, &y);
        p.key("data (input)", 0);
        p.key(&format!("degree-{deg} fit (returned)"), 1);
        let cap = format!("The data y against x as given ({} points); drawn on it: the returned least-squares polynomial \
                           y = {poly_txt}, evaluated by the model at 41 points.", x.len());
        return Some(s.chart("xy", "Least-squares polynomial fit: the data and what was returned", &cap, &p.body()));
    }
    let line = s.solve_other("least_squares_line", &[("x", tin(&x)), ("y", tin(&y))])?;
    let (m, b) = (num_of(&line, "slope")?, num_of(&line, "intercept")?);
    let r2 = num_of(&line, "r_squared");
    let ends: Vec<(f64, f64)> = [lo - pad, hi + pad].iter().filter_map(|t| peval(s, &[m, b], *t).map(|v| (*t, v))).collect();
    let mut p = DPlot::new((lo - pad, hi + pad), yrange(&[&pts, &d, &ends]), "x", "y", "");
    p.dashed(&ends, 2);
    p.curve(&pts, 0);
    for (a, bb) in x.iter().zip(&y) {
        p.dot(*a, *bb, 1, 5.0);
    }
    p.key(&format!("line{}", r2.map(|r| format!(", R² = {}", num(r))).unwrap_or_default()), 2);
    p.key(&format!("degree-{deg} fit"), 0);
    p.key("data", 1);
    let cap = format!("Least-squares fits to the points x = {}, y = {}: the degree-{deg} polynomial y = {poly_txt} \
                       (coefficients returned by the model, the curve evaluated by it at 41 points) and the straight line \
                       y = {} + {}·x{} from the model's least-squares line.", vtext_c(&x), vtext_c(&y), num(b), num(m),
                      r2.map(|r| format!(" with R² = {}", num(r))).unwrap_or_default());
    Some(s.chart("fit", "Least-squares polynomial and line through the same points", &cap, &p.body()))
}

fn vtext_c(v: &[f64]) -> String {
    let t: Vec<String> = v.iter().take(10).map(|x| num(*x)).collect();
    format!("{}{}", t.join(", "), if v.len() > 10 { ", ..." } else { "" })
}

/// "1 - 3x + 2x²" from coefficients lowest power first.
fn poly_words(c: &[f64]) -> String {
    let sup = |n: usize| -> String {
        n.to_string().chars().map(|d| match d {
            '0' => '⁰', '1' => '¹', '2' => '²', '3' => '³', '4' => '⁴', '5' => '⁵', '6' => '⁶', '7' => '⁷', '8' => '⁸',
            _ => '⁹',
        }).collect()
    };
    let mut out = String::new();
    for (i, a) in c.iter().enumerate() {
        let a = k::snap(*a, 1.0);
        if a == 0.0 && c.len() > 1 {
            continue;
        }
        let mag = num(a.abs());
        let term = match i {
            0 => mag.clone(),
            1 => if mag == "1" { "x".into() } else { format!("{mag}x") },
            _ => if mag == "1" { format!("x{}", sup(i)) } else { format!("{mag}x{}", sup(i)) },
        };
        if out.is_empty() {
            out = if a < 0.0 { format!("-{term}") } else { term };
        } else {
            out.push_str(if a < 0.0 { " - " } else { " + " });
            out.push_str(&term);
        }
    }
    if out.is_empty() { "0".into() } else { out }
}

/// Trapezoid or Simpson's rule: the area the rule sums under the data, the chords (trapezoids) or a parabola through
/// each pair of panels (each parabola point a run of the model's Lagrange interpolation of the panel's three points).
pub fn rule_area(s: &Solved) -> Option<Figure> {
    let (x, y) = data(s)?;
    let integral = s.out("integral")?;
    let simpson = s.key == "simpson_rule";
    let mut top: Vec<(f64, f64)> = vec![];
    if simpson {
        if x.len() < 3 || x.len() % 2 == 0 {
            return None;
        }
        let panels = (x.len() - 1) / 2;
        let per = if panels <= 12 { 9 } else if panels <= 40 { 5 } else { return None };
        for i in 0..panels {
            let (px, py) = (&x[2 * i..2 * i + 3], &y[2 * i..2 * i + 3]);
            for (j, t) in k::span(px[0], px[2], per, false).into_iter().enumerate() {
                if i > 0 && j == 0 {
                    continue;
                }
                top.push((t, lagrange(s, px, py, t)?));
            }
        }
    } else {
        top = x.iter().copied().zip(y.iter().copied()).collect();
    }
    let (lo, hi) = span_of(&x);
    let mut region = vec![(lo, 0.0)];
    region.extend(top.iter().copied());
    region.push((hi, 0.0));
    let d: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let mut yr = yrange(&[&top, &d]);
    yr = (yr.0.min(0.0), yr.1.max(0.0));
    let mut p = DPlot::new((lo, hi), yr, "x", "y", &format!("integral = {}", num(integral)));
    p.fill(&region, 1);
    p.hline(0.0);
    for (i, (a, b)) in x.iter().zip(&y).enumerate() {
        if !simpson || i % 2 == 0 {
            p.seg((*a, 0.0), (*a, *b), 3, true);
        }
    }
    p.curve(&top, 1);
    data_dots(&mut p, &x, &y);
    p.key("data (input)", 0);
    p.key(if simpson { "parabolas (model)" } else { "trapezoid tops" }, 1);
    let how = if simpson {
        format!("a parabola through each pair of panels (each of its points a run of the model's Lagrange \
                 interpolation of the panel's three data points; panels separated by dashed lines)")
    } else {
        "straight chords between neighbouring data points (the trapezoids, separated by dashed lines)".to_string()
    };
    let cap = format!("The data y against x as given ({} points) and the area the rule sums under them: {how}. Returned \
                       by the model: integral = {}.", x.len(), num(integral));
    Some(s.chart("xy", &format!("{}: the data and what was returned", title_of(s.key)), &cap, &p.body()))
}

/// Bisection: the request's bracket [a, b] and three wider ones, each solved by the model, with the root it returns
/// marked in each.
pub fn brackets(s: &Solved) -> Option<Figure> {
    let (a, b, root) = (s.inp("a")?, s.inp("b")?, s.out("root")?);
    if !(b > a) {
        return None;
    }
    let w = b - a;
    let mut rows: Vec<(f64, f64, f64)> = vec![(a, b, root)];
    for (lo, hi) in [(a - w / 2.0, b), (a, b + w), (a - w / 2.0, b + w)] {
        if let Some(r) = s.with(&[("a", lo), ("b", hi)]).and_then(|x| num_of(&x, "root")) {
            rows.push((lo, hi, r));
        }
    }
    let lo = rows.iter().map(|r| r.0).fold(f64::INFINITY, f64::min);
    let hi = rows.iter().map(|r| r.1).fold(f64::NEG_INFINITY, f64::max);
    let (x0, x1) = (70.0, 470.0);
    let px = |v: f64| x0 + (v - lo) / (hi - lo) * (x1 - x0);
    let mut c = Canvas::new(330.0);
    let coef = s.vec("coefficients").map(|v| vtext_c(&v)).unwrap_or_default();
    c.raw(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>",
                   super::esc(&super::clip(&format!("root = {} for coefficients {coef}", num(root)), 54))));
    let ybase = 52.0;
    for (i, (l, h, r)) in rows.iter().enumerate() {
        let y = ybase + 22.0 + i as f64 * 52.0;
        c.wire(&[(px(*l), y), (px(*h), y)]);
        c.dot((px(*l), y));
        c.dot((px(*h), y));
        c.raw(&format!("<circle class=\"axp-m\" cx=\"{:.1}\" cy=\"{y:.1}\" r=\"6\"/>", px(*r)));
        c.name(px(*l), y - 9.0, "start", &format!("[{}, {}]{}", num(*l), num(*h), if i == 0 { " (as given)" } else { "" }));
        c.value(px(*r), y + 20.0, "middle", &num(*r));
    }
    // the axis: five ticks across the brackets
    let yax = ybase + 22.0 + rows.len() as f64 * 52.0;
    c.line((x0, yax), (x1, yax));
    for t in k::span(lo, hi, 5, false) {
        c.line((px(t), yax), (px(t), yax + 5.0));
        c.name(px(t), yax + 20.0, "middle", &num(k::snap(t, hi - lo)));
    }
    c.name(270.0, yax + 40.0, "middle", "x");
    c.h = yax + 50.0;
    let cap = format!("Brackets [a, b] for the root of the polynomial with coefficients {coef} (highest power first): the \
                       request's [{}, {}] (top) and wider ones; on each, the root the model returned is marked: {}.",
                      num(a), num(b), rows.iter().map(|(l, h, r)| format!("[{}, {}] gives {}", num(*l), num(*h), num(*r)))
                          .collect::<Vec<_>>().join("; "));
    Some(s.figure("interval", "Bisection: the root found inside each bracket", &cap, c.body(), c.h))
}

/// A 2 x 2 matrix as text, or None.
fn a2(s: &Solved) -> Option<Vec<Vec<f64>>> {
    let a = s.mat("A")?;
    (a.len() == 2 && a.iter().all(|r| r.len() == 2)).then_some(a)
}

/// A x by the model's matrix-vector product.
fn matvec(s: &Solved, a: &[Vec<f64>], x: &[f64]) -> Option<(f64, f64)> {
    let r = s.solve_other("matrix_vector_product", &[("A", Input::Text(mtext(a))), ("x", tin(x))])?;
    let v = k::list_of(&r, "image")?;
    (v.len() == 2).then(|| (v[0], v[1]))
}

/// The unit circle and its image under a 2 x 2 matrix (48 points, each a run of the model's matrix-vector product):
/// an ellipse whose longest and shortest radii match the returned singular values.
pub fn circle_map(s: &Solved) -> Option<Figure> {
    let a = a2(s)?;
    let sv = s.out_list("singular_values")?;
    if sv.len() != 2 {
        return None;
    }
    let n = 48;
    let circle: Vec<(f64, f64)> = (0..=n).map(|i| {
        let t = std::f64::consts::TAU * i as f64 / n as f64;
        (t.cos(), t.sin())
    }).collect();
    let mut img: Vec<(f64, f64)> = vec![];
    for p in &circle[..n] {
        img.push(matvec(s, &a, &[p.0, p.1])?);
    }
    // the longest and the shortest mapped radius among the points drawn
    let r2 = |p: &(f64, f64)| p.0 * p.0 + p.1 * p.1;
    let far = *img.iter().max_by(|x, y| r2(x).partial_cmp(&r2(y)).unwrap_or(std::cmp::Ordering::Equal))?;
    let near = *img.iter().min_by(|x, y| r2(x).partial_cmp(&r2(y)).unwrap_or(std::cmp::Ordering::Equal))?;
    let mut xs: Vec<f64> = img.iter().map(|p| p.0).collect();
    let mut ys: Vec<f64> = img.iter().map(|p| p.1).collect();
    xs.extend([-1.0, 1.0]);
    ys.extend([-1.0, 1.0]);
    let mut p = DPlot::equal(&xs, &ys, "x₁", "x₂", "");
    p.fill(&img, 0);
    p.outline(&img, 0);
    p.dashed(&circle, 3);
    p.arrow((0.0, 0.0), far, 1);
    p.arrow((0.0, 0.0), near, 2);
    p.key("unit circle", 3);
    p.key("image under A", 0);
    p.key(&format!("σ₁ = {}", num(sv[0])), 1);
    p.key(&format!("σ₂ = {}", num(sv[1])), 2);
    let cap = format!("{n} points of the unit circle mapped by A = {} (each image a run of the model's matrix-vector \
                       product): an ellipse whose semi-axes are the singular values σ₁ = {} and σ₂ = {} returned by the \
                       model (the longest and shortest mapped radii among the points drawn as arrows).",
                      super::mtext_short(&a), num(sv[0]), num(sv[1]));
    Some(s.chart("map", "The unit circle under a matrix: singular values as semi-axes", &cap, &p.body()))
}

/// The unit square and its image under a symmetric 2 x 2 matrix (corners mapped by the model), with the returned
/// eigenvectors and their images (also mapped by the model): each image lies along its eigenvector.
pub fn square_map(s: &Solved) -> Option<Figure> {
    let a = a2(s)?;
    let lam = s.out_list("eigenvalues")?;
    let ev = out_mat(s, "eigenvectors")?;
    if lam.len() != 2 || ev.len() != 2 || ev[0].len() != 2 {
        return None;
    }
    let sq = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let mut img = vec![(0.0, 0.0)];
    for c in &sq[1..] {
        img.push(matvec(s, &a, &[c.0, c.1])?);
    }
    let mut imv = vec![];
    for v in &ev {
        imv.push(matvec(s, &a, v)?);
    }
    let mut xs: Vec<f64> = img.iter().map(|p| p.0).chain(imv.iter().map(|p| p.0)).collect();
    let mut ys: Vec<f64> = img.iter().map(|p| p.1).chain(imv.iter().map(|p| p.1)).collect();
    for v in &ev {
        xs.push(v[0]);
        ys.push(v[1]);
    }
    xs.extend([0.0, 1.0]);
    ys.extend([0.0, 1.0]);
    let mut p = DPlot::equal(&xs, &ys, "x₁", "x₂", "");
    p.fill(&img, 0);
    p.outline(&img, 0);
    let mut closed = sq.to_vec();
    closed.push(sq[0]);
    p.dashed(&closed, 3);
    for (v, w) in ev.iter().zip(&imv) {
        p.arrow((0.0, 0.0), *w, 1);
        p.arrow((0.0, 0.0), (v[0], v[1]), 2);
    }
    p.key("unit square", 3);
    p.key("its image under A", 0);
    p.key("eigenvector v", 2);
    p.key("A v = λ v", 1);
    let cap = format!("The unit square and its image under A = {} (each corner mapped by the model's matrix-vector \
                       product), with the eigenvectors it returned and their images: {}.", super::mtext_short(&a),
                      lam.iter().zip(&ev).enumerate()
                          .map(|(i, (l, v))| format!("λ{} = {} along {}", i + 1, num(k::snap(*l, 1.0)), fv(v)))
                          .collect::<Vec<_>>().join("; "));
    Some(s.chart("map", "A linear map of the plane and its eigenvectors", &cap, &p.body()))
}
