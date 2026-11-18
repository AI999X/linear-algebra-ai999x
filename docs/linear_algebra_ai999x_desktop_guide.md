# Linear Algebra: the desktop app, user guide

**Linear Algebra**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Linear Algebra covers vectors, matrices, determinants, linear systems, eigenvalues, decompositions, least squares, sparse matrices and numerical methods (root finding, interpolation, quadrature). Its desktop app holds all 89 calculations in 13 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-linear-algebra-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-linear-algebra-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-linear-algebra-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-linear-algebra.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-linear-algebra-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-linear-algebra-1.6.5-linux-x86_64.tar.gz
./ai999x-linear-algebra-1.6.5-linux-x86_64/ai999x-linear-algebra
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Linear Algebra app, opened on Determinant of a 3 x 3 matrix with its worked example.](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/hero.webp)

*The Linear Algebra app, opened on Determinant of a 3 x 3 matrix with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Determinants › Determinant of a 3 x 3 matrix (or type part of its name in the search box). It opens with a worked example filled in: “Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].” The answer: det = 6.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Determinant of a 3 x 3 matrix: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-1.webp)

*Determinant of a 3 x 3 matrix: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Determinant of a 3 x 3 matrix
det = 6
  by: Determinant of a 3 x 3 matrix  (rule of Sarrus: three down-diagonal products minus three up-diagonal products (exact))
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Two linear equations in two unknowns](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-2.webp)

*Two linear equations in two unknowns*

![Eigenvalues of a 2 x 2 matrix](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-3.webp)

*Eigenvalues of a 2 x 2 matrix*

![Root of a polynomial by Newton's method](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-4.webp)

*Root of a polynomial by Newton's method*

![QR decomposition (Householder)](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-5.webp)

*QR decomposition (Householder)*

![Least-squares regression line](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-6.webp)

*Least-squares regression line*

![Condition number of a matrix](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-7.webp)

*Condition number of a matrix*

![1-D Poisson equation by finite differences](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/scene-8.webp)

*1-D Poisson equation by finite differences*

![Determinant of a 3 x 3 matrix: a chart, enlarged.](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/chart-1.webp)

*Determinant of a 3 x 3 matrix: a chart, enlarged.*

![Two linear equations in two unknowns: a chart, enlarged.](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/chart-2.webp)

*Two linear equations in two unknowns: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Linear Algebra app in the dark theme.](https://www.ai999x.com/?ax_shot=linear-algebra-ai999x/hero-dark.webp)

*The Linear Algebra app in the dark theme.*

## 7. Every calculation

89 calculations in 13 topics; each has its own entry on the model's page (https://www.ai999x.com/models/linear-algebra-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Decompositions | Best rank-k approximation (Eckart-Young), Cholesky decomposition, LU decomposition with partial pivoting, Polar decomposition, QR decomposition (Householder), Singular values of a matrix |
| Determinants | Adjugate (classical adjoint) of a matrix, Determinant of a 2 x 2 matrix, Determinant of a 3 x 3 matrix, Determinant of an n x n matrix, Minor and cofactor of an entry |
| Eigenvalues | Characteristic polynomial of a matrix, Dominant eigenvalue by power iteration, Eigenvalues and eigenvectors of a symmetric matrix, Eigenvalues of a 2 x 2 matrix, Eigenvalues of a general square matrix (QR algorithm), Eigenvalues of a symmetric matrix (Jacobi method), Positive definiteness test (Sylvester's criterion), Smallest eigenvalue by inverse iteration |
| Least squares | Least-squares polynomial fit, Least-squares regression line, Least-squares solution of an overdetermined system, Moore-Penrose pseudoinverse (full rank), Orthogonal projection matrix onto a column space |
| Linear systems | Conjugate gradient method for Ax = b, Gauss-Seidel iteration for Ax = b, Jacobi iteration for Ax = b, Number of solutions of a linear system, Reduced row echelon form, Solve a square linear system Ax = b, Three linear equations in three unknowns, Tridiagonal system (Thomas algorithm), Two linear equations in two unknowns |
| Linear transformations | Matrix times vector (image under a linear map), Rotation of a plane vector |
| Matrices | Condition number of a matrix, Difference of two matrices, Inverse of a square matrix, Kronecker (tensor) product of two matrices, Norms of a matrix, Power of a square matrix, Product of two matrices, Rank of a matrix, Scalar multiple of a matrix, Sum of two matrices, Trace of a square matrix, Transpose of a matrix |
| Matrix functions | Matrix exponential, Square root of a positive definite matrix |
| Numerical PDE | 1-D Poisson equation by finite differences, 2-D Poisson equation on a grid (5-point stencil) |
| Numerical methods | Clamped cubic spline interpolation, Coefficients of the interpolating polynomial, Composite midpoint rule for a polynomial, Derivative of equally spaced data (finite differences), Evaluate a polynomial and its derivative (Horner's method), Gauss-Legendre quadrature nodes and weights, Gauss-Legendre quadrature of a polynomial, Lagrange interpolation, Natural cubic spline interpolation, Newton divided differences, Piecewise linear interpolation, Root of a polynomial by bisection, Root of a polynomial by Newton's method, Root of a polynomial by the secant method, Simpson's rule for tabulated data, Trapezoidal rule for tabulated data |
| Sparse matrices | Compressed sparse column (CSC) form of a matrix, Compressed sparse row (CSR) form of a matrix, Dense matrix from CSR arrays, Sparse matrix times vector (CSR) |
| Vector spaces | Column space of a matrix, Coordinates of a vector in a basis, Gram-Schmidt orthonormalization, Linear independence of vectors, Null space (kernel) of a matrix |
| Vectors | Angle between two vectors, Area of the parallelogram (and triangle) spanned by two vectors, Cross product of two 3-D vectors, Difference of two vectors, Distance between two points, Dot product of two vectors, Length (norm) of a vector, Outer product of two vectors, Projection of one vector onto another, Scalar multiple of a vector, Scalar triple product (volume of a parallelepiped), Sum of two vectors, Unit vector in the direction of a vector |

## 8. Other ways to use it

The same engine runs on the command line (`linear-algebra`, its own guide), in Python (`pip install linear-algebra-ai999x`, `import linear_algebra_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/linear-algebra-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/linear-algebra-ai999x/
