# linear-algebra-ai999x — offline, verified linear algebra solver with step-by-step answers

[![tests](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/linear-algebra-ai999x)](https://pypi.org/project/linear-algebra-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/linear-algebra-ai999x)

**[Install](https://pypi.org/project/linear-algebra-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/linear-algebra-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/linear-algebra-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

linear-algebra-ai999x is an offline, deterministic Python solver that answers linear algebra questions with verified results from 89 certified capabilities, gives 399 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**linear-algebra-ai999x** is a compact, deterministic linear algebra engine for Python. It reads a question in plain English (or a typed call), picks a certified linear algebra capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified linear algebra capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 399 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** vectors, matrices, determinants, linear systems, eigenvalues, decompositions, least squares, sparse matrices and numerical methods (root finding, interpolation, quadrature).

## Model overview

| Property | Value |
|---|---|
| Package | `linear-algebra-ai999x` 1.0.1 (import `linear_algebra_ai999x`) |
| Type | Deterministic neuro-symbolic linear algebra engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 89 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 399 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install linear-algebra-ai999x
```

```python
import linear_algebra_ai999x as linear_algebra

linear_algebra.ask("Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].").show()
# -> SOLVED — det = 6  (determinant_3x3)
linear_algebra.ask("Solve the system 2x + 3y = 7, x - y = 1.").show()
# -> SOLVED — solution = [2.0, 1.0], det = -5  (solve_2x2_system)
linear_algebra.ask("Find the eigenvalues of [[2, 1], [1, 2]].").show()
# -> SOLVED — lambda1 = 1, lambda2 = 3  (eigenvalues_2x2)
linear_algebra.ask("Use Newton's method to find a root of x^3 - 2x - 5 starting from x0 = 2.").show()
# -> SOLVED — root = 2.09455  (newton_root)
linear_algebra.solve("qr_decomposition", A="[[3, 1], [4, 2]]").show()
# -> SOLVED — Q = [[0.5999999999999999, -0.7999999999999999], [0.7999999999999999, 0.6000000000000001]], R = [[5.0, 2.1999999999999997], [-0.0, 0.40000000000000013]]  (qr_decomposition)
linear_algebra.ask("Who created you?").message
# -> linear-algebra-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `linear-algebra-ai999x "<question>"`, `linear-algebra-ai999x --capabilities`. The outputs above were produced by running this exact code on linear-algebra-ai999x 1.0.1.

## Find the right capability, and draw it

`linear_algebra.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "linear-algebra-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import linear_algebra_ai999x as linear_algebra

linear_algebra.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `linear-algebra-ai999x`; the answer is the one it returned, with the steps it took.

**Determinant of a 3 x 3 matrix** · _Determinants_

> Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].

→ **det = 6**

<details><summary>Steps</summary>

- Determinant of a 3 x 3 matrix: A = [2 0 1; 1 3 2; 1 1 2] → det = 6

</details>

**Two linear equations in two unknowns** · _Linear systems_

> Solve the system 2x + 3y = 7, x - y = 1.

→ **solution = [2, 1], det = -5**

<details><summary>Steps</summary>

- Two linear equations in two unknowns: A = [2 3; 1 -1], b = [7, 1] → solution = [2, 1], det = -5

</details>

**Eigenvalues of a 2 x 2 matrix** · _Eigenvalues_

> Find the eigenvalues of [[2, 1], [1, 2]].

→ **lambda1 = 1, lambda2 = 3**

<details><summary>Steps</summary>

- Eigenvalues of a 2 x 2 matrix: A = [2 1; 1 2] → lambda1 = 1, lambda2 = 3

</details>

**Root of a polynomial by Newton's method** · _Numerical methods_

> Use Newton's method to find a root of x^3 - 2x - 5 starting from x0 = 2.

→ **root = 2.094551482**

<details><summary>Steps</summary>

- Root of a polynomial by Newton's method: coefficients = [1, 0, -2, -5], x0 = 2 → root = 2.094551482

</details>

**QR decomposition (Householder)** · _Decompositions_

`m.solve("qr_decomposition", A="[[3, 1], [4, 2]]")`

→ **Q = [[0.6, -0.8], [0.8, 0.6]], R = [[5, 2.2], [-0, 0.4]]**

<details><summary>Steps</summary>

- QR decomposition (Householder): A = [3 1; 4 2] → Q = [[0.6, -0.8], [0.8, 0.6]], R = [[5, 2.2], [-0, 0.4]]

</details>

**Least-squares regression line** · _Least squares_

> Find the least squares line for the points (1, 2), (2, 3), (3, 5), (4, 4).

→ **slope = 0.8, intercept = 1.5, r_squared = 0.64**

<details><summary>Steps</summary>

- Least-squares regression line: x = [1, 2, 3, 4], y = [2, 3, 5, 4] → slope = 0.8, intercept = 1.5, r_squared = 0.64

</details>

## Figures

Every point was computed by `linear-algebra-ai999x`.

**A linear map of the plane and its eigenvectors**

![A linear map of the plane and its eigenvectors — computed by linear-algebra-ai999x](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/figures/symmetric-eigenvectors-map.svg)

_The unit square and its image under A = [[2, 1], [1, 2]] (each corner mapped by the package's matrix–vector product), with the eigenvectors it returned and their images: λ1 = 1 along (0.707107, -0.707107); λ2 = 3 along (0.707107, 0.707107)._

**The unit circle under a matrix: singular values as semi-axes**

![The unit circle under a matrix: singular values as semi-axes — computed by linear-algebra-ai999x](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/figures/singular-values-map.svg)

_72 points of the unit circle mapped by A = [[3, 0], [4, 5]]: an ellipse whose semi-axes are the singular values σ₁ = 6.7082 and σ₂ = 2.23607 returned by the package (the longest and shortest mapped radii drawn: 6.7082 and 2.23607)._

**LU factorisation as matrices**

![LU factorisation as matrices — computed by linear-algebra-ai999x](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/figures/lu-decomposition-heatmap.svg)

_The LU factorisation with partial pivoting returned by the package for A = [[2, 1, 1], [4, -6, 0], [-2, 7, 2]]: A = P L U with P a permutation, L unit lower triangular and U upper triangular (cells shaded by magnitude; negative values in the second colour). The product P L U, also computed by the package, equals A._

**Least-squares polynomial and line through the same points**

![Least-squares polynomial and line through the same points — computed by linear-algebra-ai999x](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/figures/least-squares-polynomial-fit.svg)

_Least-squares fits to the points x = 0, 1, 2, 3, y = 1, 0, 3, 10: the degree-2 polynomial y = 1 − 3x + 2x² (coefficients from the package, the curve evaluated by it at 61 points) and the straight line y = −1 + 3x with R² = 0.737705._

**Interpolation through five nodes: spline, polynomial, linear**

![Interpolation through five nodes: spline, polynomial, linear — computed by linear-algebra-ai999x](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/figures/natural-cubic-spline-curve.svg)

_Three interpolants through the nodes x = 0, 1, 2, 3, 4, y = 0, 2, 1, 3, 2, each evaluated by the package at 81 points: the natural cubic spline, the degree-4 Lagrange polynomial and piecewise-linear interpolation; marked: x = 2.5, spline value = 1.91964._

## Capabilities

89 certified procedures. A few of them:

- **Procedures:** 1-D Poisson equation by finite differences; 2-D Poisson equation on a grid (5-point stencil); Adjugate (classical adjoint) of a matrix; Angle between two vectors; Area of the parallelogram (and triangle) spanned by two vectors; Best rank-k approximation (Eckart-Young); Characteristic polynomial of a matrix; Cholesky decomposition; Clamped cubic spline interpolation; Coefficients of the interpolating polynomial; Column space of a matrix; Composite midpoint rule for a polynomial; Compressed sparse column (CSC) form of a matrix; Compressed sparse row (CSR) form of a matrix; Condition number of a matrix; Conjugate gradient method for Ax = b; Coordinates of a vector in a basis; Cross product of two 3-D vectors; Dense matrix from CSR arrays; Derivative of equally spaced data (finite differences); Determinant of a 2 x 2 matrix; Determinant of a 3 x 3 matrix; Determinant of an n x n matrix; Difference of two matrices …

`linear_algebra_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified linear algebra capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (vectors, matrices, determinants, linear systems, eigenvalues, decompositions, least squares, sparse matrices and numerical methods (root finding, interpolation, quadrature)); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{linear_algebra_ai999x_1_0_1,
  title  = {linear-algebra-ai999x: an offline, verified linear algebra solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
  url    = {https://huggingface.co/AI999X/linear-algebra-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/linear-algebra-ai999x/discussions or visit https://ai999x.com.

## Support the project

linear-algebra-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/linear-algebra-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/linear-algebra-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/linear-algebra-ai999x/tree/main/onnx).
