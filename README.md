# Linear Algebra (linear-algebra-ai999x)

[![tests](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/linear-algebra-ai999x)](https://pypi.org/project/linear-algebra-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/linear-algebra-ai999x)

Linear Algebra covers vectors, matrices, determinants, linear systems, eigenvalues, decompositions, least squares, sparse matrices and numerical methods (root finding, interpolation, quadrature): 89 calculations in 13 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Linear Algebra desktop app: Eigenvalues and eigenvectors of a symmetric matrix, with its worked example and live scene](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/hero.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-1.6.4-linux-x86_64.tar.gz) |
| **Command line** `linear-algebra` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.4/ai999x-cli-1.6.4-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#try) |
| **Python** 3.10+ | `pip install linear-algebra-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import linear_algebra_ai999x as linear_algebra

a = linear_algebra.ask(
    "What is the condition number of the matrix [2 0; 0 0.5]?"
)
print(a.outcome, a.values)
# SOLVED {'cond2': 4.0, 'cond1': 4.0}

a = linear_algebra.ask(
    "Solve the 1-D Poisson equation -u'' = f with zero boundary "
    "values, L = 1 and f = (1, 1, 1) at the interior points."
)
print(a.outcome, a.values)
# SOLVED {'u': [0.09375, 0.125, 0.09375], 'h': 0.25}

a = linear_algebra.ask(
    "Use a clamped cubic spline through the points (0, 0), (1, 1), (2, "
    "0) with end slopes 1 and -1 to estimate the value at x = 0.5."
)
print(a.outcome, a.values)
# SOLVED {'value': 0.625}

a = linear_algebra.ask(
    "Find the angle between the vectors (1, 0) and (1, 1)."
)
print(a.outcome, a.values)
# SOLVED {'angle_degrees': 45.0, 'angle_radians': 0.7853981633974483}

a = linear_algebra.ask("Find the determinant of [[1, 2], [3, 4]].")
print(a.outcome, a.values)
# SOLVED {'det': -2.0}

a = linear_algebra.ask(
    "Find the least squares line for the points (1, 2), (2, 3), (3, "
    "5), (4, 4)."
)
print(a.outcome, a.values)
# SOLVED {'slope': 0.8, 'intercept': 1.5, 'r_squared': 0.64}

a = linear_algebra.ask(
    "Find the area of the parallelogram spanned by (3, 0) and (1, 2)."
)
print(a.outcome, a.values)
# SOLVED {'area': 6.0, 'triangle_area': 3.0}

a = linear_algebra.ask(
    "Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]]."
)
print(a.outcome, a.values)
# SOLVED {'det': 6.0}

a = linear_algebra.ask("Find the inverse of [[4, 7], [2, 6]].")
print(a.outcome, a.values)
# SOLVED {'inverse': [[0.6, -0.7], [-0.2, 0.4]]}

a = linear_algebra.ask(
    "Use the midpoint rule with 4 subintervals to integrate x^2 from 0 "
    "to 2."
)
print(a.outcome, a.values)
# SOLVED {'integral': 2.625}

a = linear_algebra.ask(
    "Find the determinant of [[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 5, "
    "6], [0, 0, 7, 8]]."
)
print(a.outcome, a.values)
# SOLVED {'det': 4.0}

a = linear_algebra.ask("Find the eigenvalues of [[2, 1], [1, 2]].")
print(a.outcome, a.values)
# SOLVED {'lambda1': 1.0, 'lambda2': 3.0}

a = linear_algebra.ask(
    "How many solutions does the system x + y = 2, 2x + 2y = 5 have?"
)
print(a.outcome, a.values)
# SOLVED {'solutions': 'none', 'rank_A': 1.0, 'rank_augmented': 2.0}

a = linear_algebra.ask(
    "Are the vectors (1, 2, 3), (4, 5, 6) and (7, 8, 9) linearly "
    "independent?"
)
print(a.outcome, a.values)
# SOLVED {'independent': False, 'rank': 2.0}

a = linear_algebra.ask(
    "Find the cofactor C23 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]]."
)
print(a.outcome, a.values)
# SOLVED {'minor': -6.0, 'cofactor': 6.0}
```

## Screenshots

[![The Linear Algebra desktop app: Eigenvalues and eigenvectors of a symmetric matrix, with its worked example and live scene](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/hero-dark.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

[![Singular values of a matrix: the live scene in the Linear Algebra desktop app](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/see-1.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

*Singular values of a matrix*

[![Least-squares polynomial fit: the live scene in the Linear Algebra desktop app](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/see-2.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

*Least-squares polynomial fit*

[![Coordinates of a vector in a basis: the live scene in the Linear Algebra desktop app](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/see-3.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

*Coordinates of a vector in a basis*

[![Root of a polynomial by bisection: the live scene in the Linear Algebra desktop app](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/see-4.webp)](https://www.ai999x.com/models/linear-algebra-ai999x/)

*Root of a polynomial by bisection*

Every calculation, with its formula, a worked example and its figures: **[Linear Algebra on ai999x.com](https://www.ai999x.com/models/linear-algebra-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.4 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/linear-algebra-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.4`; the [GitHub release](https://github.com/AI999X/linear-algebra-ai999x/releases/tag/v1.6.4) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `linear-algebra` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/linear-algebra`) and its data (`rust/models/linear_algebra/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p linear-algebra-ai999x
./target/release/linear-algebra --help
cargo test -p linear-algebra-ai999x
```
