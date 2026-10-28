"""linear-algebra-ai999x — offline linear algebra answers with their formula, steps and verification.

    import linear_algebra_ai999x as m
    m.ask("Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].").show()        # det = 6
    m.solve("solve_linear_system", A="[[2, 1], [1, 3]]", b="3, 5").show()              # x = (0.8, 1.4)
    m.capabilities()                   # every certified capability

Covers vectors (sums, dot/cross products, norms, angles, projections), matrices (products, powers, transpose,
trace, norms, Kronecker products), determinants (2x2, 3x3, n x n, minors, cofactors, adjugate), linear systems
(Cramer's rule, Gaussian elimination, RREF, rank, null and column spaces, existence and uniqueness), eigenvalues
(2x2, symmetric by Jacobi, general by the QR algorithm, power and inverse iteration, characteristic polynomial),
decompositions (LU, QR, Cholesky, Gram-Schmidt, SVD singular values, polar, best low-rank approximation), matrix
functions (exponential, square root), least squares (lines, polynomials, overdetermined systems, pseudoinverse,
projections), iterative solvers (Jacobi, Gauss-Seidel, conjugate gradients), sparse matrices (CSR/CSC, tridiagonal,
discrete Poisson problems) and numerical methods (bisection, Newton and secant roots; Lagrange, Newton, linear and
cubic-spline interpolation; trapezoid, Simpson, midpoint and Gauss-Legendre quadrature; finite differences).
Rational results are computed exactly from the decimals given. A question that is not determined is refused
rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("linear_algebra", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
