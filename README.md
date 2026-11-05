# linear-algebra-ai999x — offline, verified linear algebra solver with step-by-step answers

[![tests](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/linear-algebra-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/linear-algebra-ai999x)](https://pypi.org/project/linear-algebra-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/linear-algebra-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/linear-algebra-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/linear-algebra-ai999x/)** · **[Guides](https://huggingface.co/AI999X/linear-algebra-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/linear-algebra-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

linear-algebra-ai999x is an offline, deterministic Python solver that answers linear algebra questions with verified results from 89 certified capabilities, gives 399 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**linear-algebra-ai999x** is a compact, deterministic linear algebra engine for Python. It reads a question in plain English (or a typed call), picks a certified linear algebra capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** linear-algebra-ai999x comes as the AI999X desktop app and as the command-line program `linear-algebra` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Linear Algebra desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-linear-algebra-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`linear-algebra`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install linear-algebra-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Linear Algebra also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.3](https://github.com/AI999X/linear-algebra-ai999x/releases/tag/v1.0.3) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Linear Algebra) | Windows x86-64 | `ai999x-linear-algebra-1.2.0-windows-x86_64.zip` | 7.6 MB | [Hub](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/rust/ai999x-linear-algebra-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/linear-algebra-ai999x/releases/download/v1.0.3/ai999x-linear-algebra-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-linear-algebra-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-linear-algebra.exe` (no installer) |
| Desktop app (Linear Algebra) | Linux x86-64 | `ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz` | 8.6 MB | [Hub](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/rust/ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/linear-algebra-ai999x/releases/download/v1.0.3/ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-linear-algebra` (X11 or Wayland desktop) |
| Command line (`linear-algebra`) | Windows x86-64 | `ai999x-linear-algebra-cli-1.2.0-windows-x86_64.zip` | 3.4 MB | [Hub](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/rust/ai999x-linear-algebra-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/linear-algebra-ai999x/releases/download/v1.0.3/ai999x-linear-algebra-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `linear-algebra.exe ask "<question>"` or `linear-algebra.exe solve <capability> name=value` |
| Command line (`linear-algebra`) | Linux x86-64 | `ai999x-linear-algebra-cli-1.2.0-linux-x86_64.tar.gz` | 3.4 MB | [Hub](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/rust/ai999x-linear-algebra-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/linear-algebra-ai999x/releases/download/v1.0.3/ai999x-linear-algebra-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./linear-algebra ask "<question>"` or `./linear-algebra solve <capability> name=value` |

The command-line archive holds `linear-algebra` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/linear-algebra-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Linear Algebra** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Linear Algebra among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-linear-algebra-1.2.0-linux-x86_64.tar.gz
./ai999x-linear-algebra-1.2.0-linux-x86_64/ai999x-linear-algebra
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
linear-algebra --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
linear-algebra --version
```

Every command below was run as shown; the text under it is what the program printed.

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ linear-algebra info newton_root
Root of a polynomial by Newton's method
  Newton-Raphson iteration until the step is below the last bit of x
  in   coefficients polynomial coefficients, highest power first (written out in x in a question) (vector) [1]
  in   x0       starting value [1]
  out  root     the root the iteration converges to [1]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ linear-algebra solve newton_root "coefficients=[1, 0, -2, -5]" x0=2.0
root = 2.094551482
  by: Root of a polynomial by Newton's method  (Newton-Raphson iteration until the step is below the last bit of x)
  x0 = 2
  assumes: the iteration converges (within 200 steps) to a simple, well-conditioned root; p'(x) never vanishes at an iterate
```

**Draw the figures** — The live figures for your own inputs, written as SVG files into the folder given with --out (here the current folder); open them in any browser. Without --out the SVG text comes back as JSON.

```console
$ linear-algebra figures newton_root "coefficients=[1, 0, -2, -5]" x0=2.0 --out .
[
  {
    "slug": "newton-root-live-1",
    "title": "The root the iteration converges to as polynomial coefficients, highest power first (w... varies",
    "caption": "The root the iteration converges to as coefficients[1] runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your coefficients[1] = 1 gives root = 2.095. Your inputs: coefficients = (list), x0 = 2.0.",
    "file": "./newton-root-live-1.svg"
  },
  {
    "slug": "newton-root-live-2",
    "title": "The root the iteration converges to as polynomial coefficients, highest power first (w... varies",
    "caption": "The root the iteration converges to as coefficients[4] runs from -8.75 to -1.25, every other input as you entered it (29 solutions). Marked: your coefficients[4] = -5 gives root = 2.095. Your inputs: coefficients = (list), x0 = 2.0.",
    "file": "./newton-root-live-2.svg"
  }
]
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | linear-algebra batch

`cases.jsonl`:

```json
{"key": "newton_root", "inputs": {"coefficients": "[1, 0, -2, -5]", "x0": 2.0}}
{"key": "qr_decomposition", "inputs": {"A": "[[3, 1], [4, 2]]"}}
```

```console
$ linear-algebra batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":2.0945514815423265,"unit":"","solved_for":"root","si":[["x0",2.0]],"values":[["root",2.0945514815423265,""]],"target":"root","phrase":"the root the iteration converges to","assumptions":["the iteration converges (within 200 steps) to a simple, well-conditioned root","p'(x) never vanishes at an iterate"],"name":"Root of a polynomial by Newton's method","equation":"Newton-Raphson iteration until the step is below the last bit of x"}
{"outcome":"SOLVED","message":"","value":null,"unit":"","solved_for":"R","values":[["Q",[[0.5999999999999999,-0.7999999999999999],[0.7999999999999999,0.6000000000000001]],""],["R",[[5.0,2.1999999999999997],[0.0,0.40000000000000013]],""]],"target":"R","phrase":"m x n matrix with orthonormal columns","assumptions":["rows >= columns","independent columns","at most 10 x 10","convention: the diagonal of R is positive (unique)"],"name":"QR decomposition (Householder)","equation":"Householder reflections, then signs fixed so that diag(R) > 0"}
```

### In your browser

Open [Linear Algebra on ai999x.com](https://www.ai999x.com/models/linear-algebra-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `linear_algebra_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install linear-algebra-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Best rank-k approximation (Eckart-Young)

The rank-k matrix closest to A in the 2- and Frobenius norms (truncated SVD), with both approximation errors.

Command line:

```console
$ linear-algebra solve low_rank_approximation "A=[[3, 0], [0, 1]]" k=1
approximation = [[3.0,0.0],[0.0,0.0]]
error2 = 1
errorF = 1
  by: Best rank-k approximation (Eckart-Young)  (one-sided Jacobi SVD, keep the k largest singular triplets)
  assumes: 1 <= k <= min(rows, columns); sigma_k > sigma_(k+1) (otherwise A_k is not unique); at most 10 x 10
```

Python:

```python
import linear_algebra_ai999x as m

a = m.solve("low_rank_approximation", A=[[3, 0], [0, 1]], k="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#cap-low_rank_approximation) and press **Compute in your browser**.

_What this means:_ This calculation produces the closest matrix of rank k to the input matrix A, denoted as [[3, 0], [0, 0]]. The method computes this by retaining only the largest k singular values and their corresponding vectors from the singular value decomposition. The resulting approximation minimizes the distance to A in both the spectral norm and the Frobenius norm. The specific errors introduced by this truncation are reported as 1 for the spectral norm and 1 for the Frobenius norm.

### Cholesky decomposition

Factor a symmetric positive definite matrix as A = L L^T with L lower triangular, positive diagonal.

Command line:

```console
$ linear-algebra solve cholesky "A=[[4, 12, -16], [12, 37, -43], [-16, -43, 98]]"
L = [[2.0,0.0,0.0],[6.0,1.0,0.0],[-8.0,5.0,3.0]]
  by: Cholesky decomposition  (the Cholesky-Banachiewicz recursion l_jj = sqrt(a_jj - sum l_jk^2), l_ij = (a_ij - sum l_ik l_jk)/l_jj)
  assumes: A is symmetric; A is positive definite (checked exactly by its leading minors); at most 10 x 10
```

Python:

```python
import linear_algebra_ai999x as m

a = m.solve("cholesky", A=[[4, 12, -16], [12, 37, -43], [-16, -43, 98]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#cap-cholesky) and press **Compute in your browser**.

_What this means:_ This calculation computes the Cholesky decomposition of the symmetric positive definite matrix [4 12 -16; 12 37 -43; -16 -43 98], yielding the lower triangular factor [[2, 0, 0], [6, 1, 0], [-8, 5, 3]]. The result means that multiplying [[2, 0, 0], [6, 1, 0], [-8, 5, 3]] by its transpose reproduces the original matrix, which simplifies solving systems and computing determinants for this specific type of matrix.

### LU decomposition with partial pivoting

Factor a nonsingular square matrix as A = P L U (permutation, unit lower triangular, upper triangular).

Command line:

```console
$ linear-algebra solve lu_decomposition "A=[[1, 2], [3, 4]]"
P = [[0.0,1.0],[1.0,0.0]]
L = [[1.0,0.0],[0.3333333333333333,1.0]]
U = [[3.0,4.0],[0.0,0.6666666666666667]]
  by: LU decomposition with partial pivoting  (Gaussian elimination with partial pivoting (Doolittle form) in floating point)
  assumes: A is square and nonsingular; at most 10 x 10; pivot: the first row of largest magnitude in the column (the scipy.linalg.lu convention A = P L U)
```

Python:

```python
import linear_algebra_ai999x as m

a = m.solve("lu_decomposition", A=[[1, 2], [3, 4]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#cap-lu_decomposition) and press **Compute in your browser**.

_What this means:_ This calculation factors the square nonsingular matrix [1 2; 3 4] into three components: a permutation matrix [[0, 1], [1, 0]], a unit lower triangular matrix [[1, 0], [0.333333, 1]], and an upper triangular matrix [[3, 4], [0, 0.666667]]. The product of these matrices reconstructs the original input, satisfying the relationship where the permuted rows of the lower and upper factors combine to form the initial data. This decomposition is achieved through Gaussian elimination with partial pivoting, ensuring numerical stability by selecting the largest available pivot element at each step.

### Polar decomposition

Write a nonsingular square matrix as A = U P: U orthogonal (a rotation or reflection), P symmetric positive definite (a stretch).

Command line:

```console
$ linear-algebra solve polar_decomposition "A=[[0, -2], [1, 0]]"
U = [[0.0,-1.0],[1.0,0.0]]
P = [[1.0,0.0],[0.0,2.0]]
  by: Polar decomposition  (from the one-sided Jacobi SVD A = W S V^T: U = W V^T, P = V S V^T)
  assumes: A is square and nonsingular; at most 10 x 10; right polar form A = U P (scipy.linalg.polar's default)
```

Python:

```python
import linear_algebra_ai999x as m

a = m.solve("polar_decomposition", A=[[0, -2], [1, 0]])
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/linear-algebra-ai999x/#cap-polar_decomposition) and press **Compute in your browser**.

_What this means:_ Polar decomposition separates a nonsingular square matrix into two distinct geometric components. For the input matrix [0 -2; 1 0], the algorithm computes an orthogonal factor [[0, -1], [1, 0]], which represents a pure rotation or reflection, and a symmetric positive definite factor [[1, 0], [0, 2]], which represents a pure stretch. The product of these factors reconstructs the original transformation, allowing you to analyze rotational and scaling effects independently.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/linear-algebra-ai999x/blob/main/docs/linear_algebra_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/linear-algebra-ai999x/blob/main/docs/linear_algebra_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/linear-algebra-ai999x/blob/main/docs/linear_algebra_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/linear-algebra-ai999x/blob/main/docs/linear_algebra_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/linear-algebra-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified linear algebra capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 399 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `linear-algebra` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** vectors, matrices, determinants, linear systems, eigenvalues, decompositions, least squares, sparse matrices and numerical methods (root finding, interpolation, quadrature).

## Model overview

| Property | Value |
|---|---|
| Package | `linear-algebra-ai999x` 1.0.3 (import `linear_algebra_ai999x`) |
| Type | Deterministic neuro-symbolic linear algebra engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 89 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 399 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `linear-algebra-ai999x "<question>"`; for the native program use `linear-algebra` (above). The outputs above were produced by running this exact code on linear-algebra-ai999x 1.0.3.

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

## Python worked examples, with their steps

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
@software{linear_algebra_ai999x_1_0_3,
  title  = {linear-algebra-ai999x: an offline, verified linear algebra solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
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
