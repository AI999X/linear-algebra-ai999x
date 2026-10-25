"""Linear algebra procedures: deterministic algorithms with typed inputs (vectors, matrices, whole numbers) and one or
several outputs (domain code, loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question templates it parses,
verified worked examples, an INDEPENDENT reference computation (numpy / scipy / sympy / pyamg, imported inside the
reference and used only by certification) and a deliberately wrong variant (certification shows the checks reject it).

Conventions (stated once, used everywhere):
  * Inputs are read as the decimals the user wrote (0.1 is one tenth). Where a result is rational in the inputs
    (sums, products, determinants, inverses, solutions, interpolants) it is computed in exact rational arithmetic and
    rounded once to a float at the end.
  * Matrices are lists of rows; vectors are lists. Indices in questions and outputs are 1-based.
  * Sign conventions that are otherwise arbitrary are fixed: an eigenvector or singular vector has unit length and its
    first entry of largest magnitude positive; QR and Gram-Schmidt give R with a positive diagonal; LU follows A = P L U
    with partial pivoting (the first row of largest magnitude is chosen); eigenvalues are listed real ascending, then
    complex by real part with the positive imaginary part first.
"""
from __future__ import annotations

import cmath
import math
import re
from fractions import Fraction

MAX_DIM = 10            # exact rational routines: matrices up to 10 x 10 (the result stays fast)


# ============================================================================================================
# exact helpers
# ============================================================================================================
def _finite(x, name):
    if isinstance(x, bool) or not isinstance(x, (int, float)) or not math.isfinite(x):
        raise ValueError(f"{name} must contain finite real numbers")
    return x


def _F(x, name="input"):
    """The exact decimal the user wrote (repr of a float is the shortest decimal that reads back to it)."""
    _finite(x, name)
    return Fraction(int(x)) if isinstance(x, int) else Fraction(repr(float(x)))


def _need_vec(v, name, n=None, min_n=1, max_n=None):
    if not isinstance(v, (list, tuple)) or any(isinstance(x, (list, tuple)) for x in v):
        raise ValueError(f"{name} must be a list of numbers")
    for x in v:
        _finite(x, name)
    if len(v) < min_n:
        raise ValueError(f"{name} needs at least {min_n} entries")
    if n is not None and len(v) != n:
        raise ValueError(f"{name} must have exactly {n} entries")
    if max_n is not None and len(v) > max_n:
        raise ValueError(f"{name} allows at most {max_n} entries")
    return list(v)


def _need_mat(A, name, square=False, rows=None, cols=None, max_dim=MAX_DIM):
    if not isinstance(A, (list, tuple)) or not A or not all(isinstance(r, (list, tuple)) and r for r in A):
        raise ValueError(f"{name} must be a matrix (a list of rows)")
    w = len(A[0])
    if any(len(r) != w for r in A):
        raise ValueError(f"{name} must be rectangular (rows of equal length)")
    for r in A:
        for x in r:
            _finite(x, name)
    if square and len(A) != w:
        raise ValueError(f"{name} must be square")
    if rows is not None and len(A) != rows:
        raise ValueError(f"{name} must have {rows} rows")
    if cols is not None and w != cols:
        raise ValueError(f"{name} must have {cols} columns")
    if max_dim and (len(A) > max_dim or w > max_dim):
        raise ValueError(f"{name} may have at most {max_dim} rows and columns")
    return [list(r) for r in A]


def _FM(A):
    return [[_F(x) for x in r] for r in A]


def _FV(v):
    return [_F(x) for x in v]


def _fl(M):
    """Fractions (or nested lists of them) to floats."""
    if isinstance(M, (list, tuple)):
        return [_fl(x) for x in M]
    return float(M)


def _det_F(M):
    M = [r[:] for r in M]
    n, det = len(M), Fraction(1)
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        if M[p][c] == 0:
            return Fraction(0)
        if p != c:
            M[c], M[p] = M[p], M[c]
            det = -det
        det *= M[c][c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f:
                for k in range(c, n):
                    M[r][k] -= f * M[c][k]
    return det


def _rref_F(M):
    """Reduced row echelon form (exact) and the pivot columns (0-based)."""
    M = [r[:] for r in M]
    rows, cols = len(M), len(M[0])
    piv, r = [], 0
    for c in range(cols):
        if r == rows:
            break
        p = max(range(r, rows), key=lambda i: abs(M[i][c]))
        if M[p][c] == 0:
            continue
        M[r], M[p] = M[p], M[r]
        pv = M[r][c]
        M[r] = [x / pv for x in M[r]]
        for i in range(rows):
            if i != r and M[i][c] != 0:
                f = M[i][c]
                M[i] = [a - f * b for a, b in zip(M[i], M[r])]
        piv.append(c)
        r += 1
    return M, piv


def _inv_F(M):
    n = len(M)
    aug = [list(r) + [Fraction(int(i == j)) for j in range(n)] for i, r in enumerate(M)]
    R, piv = _rref_F(aug)
    if piv[:n] != list(range(n)):
        raise ValueError("the matrix is singular (its determinant is 0), so it has no inverse")
    return [r[n:] for r in R]


def _solve_F(A, b):
    n = len(A)
    aug = [list(r) + [bi] for r, bi in zip(A, b)]
    R, piv = _rref_F(aug)
    if piv != list(range(n)):
        raise ValueError("the system has no unique solution (the coefficient matrix is singular)")
    return [R[i][n] for i in range(n)]


def _mm_F(A, B):
    Bt = list(zip(*B))
    return [[sum((a * b for a, b in zip(r, c)), Fraction(0)) for c in Bt] for r in A]


def _mm(A, B):
    Bt = list(zip(*B))
    return [[math.fsum(a * b for a, b in zip(r, c)) for c in Bt] for r in A]


def _mv(A, x):
    return [math.fsum(a * b for a, b in zip(r, x)) for r in A]


def _T(A):
    return [list(c) for c in zip(*A)]


def _eye(n):
    return [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]


def _dot(u, v):
    return math.fsum(a * b for a, b in zip(u, v))


def _norm(v):
    return math.sqrt(math.fsum(x * x for x in v))


def _fro(A):
    return math.sqrt(math.fsum(x * x for r in A for x in r))


# ============================================================================================================
# verification helpers (plain Python): a residual that is 0 for a correct output and large for a wrong one
# ============================================================================================================
BAD = 1.0


def _safe(fn):
    def wrapped(inputs, outputs):
        try:
            r = float(fn(inputs, outputs))
            return r if r == r else BAD
        except Exception:                          # a malformed output is a rejected output
            return BAD
    return wrapped


def _rel(a, b):
    return abs(complex(a) - complex(b)) / max(1.0, abs(complex(a)), abs(complex(b)))


def _diff(x, y):
    """Largest element-wise relative difference of two numbers / lists / matrices of the same shape (BAD otherwise)."""
    if isinstance(x, (list, tuple)) or isinstance(y, (list, tuple)):
        if not (isinstance(x, (list, tuple)) and isinstance(y, (list, tuple))) or len(x) != len(y):
            return BAD
        return max((_diff(a, b) for a, b in zip(x, y)), default=0.0)
    if isinstance(x, bool) or isinstance(y, bool) or isinstance(x, str) or isinstance(y, str):
        return 0.0 if x == y else BAD
    return _rel(x, y)


def _shape(M):
    return (len(M), len(M[0])) if M and isinstance(M[0], (list, tuple)) else (len(M),)


def _recompute(run, keys):
    """Residual: the outputs agree with a fresh exact recomputation (used where the result is an exact rational)."""
    def verify(inputs, outputs):
        want = run(**inputs)
        return max(_diff(outputs[k], want[k]) for k in keys)
    return _safe(verify)


# ============================================================================================================
# reading questions: matrices, vectors and named numbers written in the text
# ============================================================================================================
_N = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_OBJ = re.compile(
    r"(?P<m2>\[\s*\[[^\[\]]*\](?:\s*,?\s*\[[^\[\]]*\])*\s*\])"           # [[1, 2], [3, 4]]
    r"|(?P<m1>\[[^\[\]]*;[^\[\]]*\])"                                    # [1 2; 3 4]
    r"|(?P<v1>\[[^\[\];]*\])"                                            # [1, 2, 3]
    r"|(?P<v2>\(\s*" + _N + r"(?:\s*,\s*" + _N + r")+\s*\))"              # (1, 2, 3)
    r"|(?P<v3><\s*" + _N + r"(?:\s*,\s*" + _N + r")+\s*>)")               # <1, 2, 3>
_NAME = re.compile(r"([A-Za-z][A-Za-z0-9_]*)\s*=\s*$")


def _floats(text):
    parts = [p for p in re.split(r"[,\s]+", text.strip()) if p]
    try:
        return [float(p) for p in parts] if parts else None
    except ValueError:
        return None


def _as_matrix(raw):
    s = raw.strip()
    if s.startswith("[["):
        rows = [r for r in re.split(r"\]\s*,?\s*\[", s.strip("[] ")) if r.strip()]
    else:
        rows = [r for r in s.strip("[] ").split(";") if r.strip()]
    out = [_floats(r) for r in rows]
    return out if out and all(r for r in out) else None


class _Q:
    """A question read once: lower-case text, the objects (matrices/vectors) in order, the rest of the text."""

    def __init__(self, question):
        self.raw = question
        self.text = question.lower().replace("−", "-").replace("×", "x")
        self.objs, rest, last = [], [], 0
        for m in _OBJ.finditer(question.replace("−", "-")):
            kind = "matrix" if (m.group("m1") or m.group("m2")) else "vector"
            raw = m.group(0)
            if kind == "matrix":
                value = _as_matrix(raw)
                text = raw if raw.strip().startswith("[[") else raw.strip()[1:-1]
            else:
                value = _floats(raw.strip()[1:-1])
                text = raw.strip()[1:-1]
            name = _NAME.search(question[:m.start()])
            self.objs.append({"kind": kind, "text": text, "value": value, "name": name.group(1) if name else None,
                              "start": m.start(), "end": m.end(), "bracket": raw.strip()[:1]})
            rest.append(question[last:m.start()])
            rest.append(" @ ")
            last = m.end()
        rest.append(question[last:])
        self.rest = "".join(rest).lower()
        self.mats = [o for o in self.objs if o["kind"] == "matrix"]
        self.vecs = [o for o in self.objs if o["kind"] == "vector"]

    def has(self, *patterns):
        return any(re.search(p, self.text) for p in patterns)

    def named(self, name):
        """A number given as 'name = 3' outside the objects."""
        m = re.search(r"(?<![A-Za-z0-9_])" + re.escape(name) + r"\s*=\s*(" + _N + r")(?!\d|\.\d)", self.raw)
        return float(m.group(1)) if m else None

    def numbers(self):
        """Numbers written outside the objects and outside 'name = value' pairs."""
        rest = re.sub(r"[a-z][a-z0-9_]*\s*=\s*" + _N, " ", self.rest)
        rest = re.sub(r"\b\d+\s*x\s*\d+\b", " ", rest)                     # '3x3' names a size, not a value
        return [float(x) for x in re.findall(r"(?<![a-z0-9_.])" + _N + r"(?![a-z0-9_])", rest)]


def _safe_parse(fn):
    def wrapped(question):
        try:
            return fn(_Q(question)) if isinstance(question, str) else None
        except Exception:
            return None
    return wrapped


def _dims(o):
    v = o["value"]
    if v is None:
        return None
    return (len(v), len(v[0])) if o["kind"] == "matrix" else (len(v),)


def _is_square(o, n=None):
    d = _dims(o)
    return d is not None and len(d) == 2 and d[0] == d[1] and (n is None or d[0] == n)


def _pick(q, kind, name=None, index=0):
    objs = q.mats if kind == "matrix" else q.vecs
    if name is not None:
        for o in objs:
            if o["name"] == name:
                return o
    return objs[index] if len(objs) > index else None


# --- whole-question reading rules shared by the parsers ------------------------------------------------------------
# A parser answers only a question it reads completely: every number written in the question is bound to an input,
# the task the question names is the parser's own, and the operands are taken in the order the question states.
_SIZE = re.compile(r"\b\d+\s*(?:x|×|by|-by-)\s*\d+\b")                   # '3x3', '3 by 3' name a size, not a value


def _loose(q, rest=None):
    """Every number written outside the matrices and vectors (including those of 'name = value' pairs)."""
    rest = _SIZE.sub(" ", q.rest if rest is None else rest)
    return [float(x) for x in re.findall(r"(?<![a-z0-9_.])" + _N + r"(?![a-z0-9_])", rest)]


def _whole(fn, allow=()):
    """A single-matrix parser: refuse a question that states any number besides the matrix (a rank k, a tolerance,
    a count of iterations, a power ... belongs to a different task or changes the answer). `allow`: written forms
    whose number is part of the capability's own name ('1-norm', 'A^-1')."""
    def wrapped(question):
        if isinstance(question, str):
            try:
                q = _Q(question)
                rest = q.rest
                for a in allow:
                    rest = re.sub(a, " ", rest)
                if _loose(q, rest):
                    return None
            except Exception:
                return None
        return fn(question)
    return wrapped


# the MODULUS of the dominant eigenvalue is asked (the spectral radius), not the signed eigenvalue; 'the eigenvalue of
# largest magnitude' only selects an eigenvalue and is not among these
_ASKS_MODULUS = (r"\b(?:absolute value|magnitude|modulus)(?:es)? of (?:the |its )?(?:dominant|largest|maximum|biggest)\b",
                 r"\b(?:dominant|largest|maximum) eigenvalue(?:'s)? (?:absolute value|magnitude|modulus)\b",
                 r"\b(?:largest|maximum|biggest) absolute (?:value|eigenvalue)", r"\beigenvalue (?:magnitude|modulus)\b")
# an ordered LIST of eigenvalues is asked (the list outputs are ascending); descending order is not what they give
_LIST_ORDER = (r"\bascending\b", r"\bsorted\b", r"\bincreasing order\b")
_LIST_DESC = (r"\bdescending\b", r"\bdecreasing\b", r"\blargest to smallest\b", r"\bhighest to lowest\b")
_NORM_NAMES = (r"(?<![a-z0-9_.])[12]\s*-?\s*norms?\b", r"\bnorm\s*-?\s*[12](?![0-9.])", r"\b(?:l|ell)\s*[-_]?\s*[12]\b",
               r"\b(?:ell|l)\s*\^\s*[12]\b")                    # '1-norm', '2 norm', 'norm-2', 'L1': names, not values


def _bare_vector(q,names=r"(?:\bb\b|right-hand[- ]side|\brhs\b|\bvector|load|source|forcing|\bvalues?\b)"):
    """A vector written as a bare list ('b = -9, 8.6', 'the load vector as 2.6, 3.2, -1'): exactly one comma-separated
    list of numbers outside the brackets, introduced by a word that names it as the right-hand side, and no other
    number in the question. Returns the list as text, or None."""
    lists = list(re.finditer(r"(?<![a-z0-9_.])(" + _N + r"(?:\s*,\s*" + _N + r")+)(?![a-z0-9_])", q.rest))
    if len(lists) != 1:
        return None
    m = lists[0]
    lead = q.rest[max(0, m.start() - 40):m.start()]
    if not re.search(names + r"[^@.;:?!]*$", lead):                    # named in the same clause, no object between
        return None
    if _loose(q, q.rest[:m.start()] + " " + q.rest[m.end():]):
        return None
    return m.group(1)


_AB_NAMED = re.compile(r"(?<![A-Za-z0-9_])([AB])\s*(?:=|:|\bis\b|\bbe\b|\bwith\b|\bconsists\b|\brepresented\b|\bgiven\b|"
                       r"\bdefined\b|\bhas\b)")
_AB_ROLE = re.compile(r"\b(first|left|second|right)\b(?![- ]hand)")


def _matrix_words(q):
    return len(re.findall(r"\bmatrix\b", q.rest)) + 2 * len(re.findall(r"\bmatrices\b", q.rest))


def _operands(q, rows=False):
    """The two matrix operands (A, B) of a product. With rows=True a list written in square brackets, [1 2 3], is the
    one-row matrix it is written as. The order is the one the question states ('A is', 'B = ', 'the first/left
    matrix', 'the second/right matrix'), else the order written; a contradictory statement is refused."""
    objs = sorted(q.objs, key=lambda o: o["start"])
    if len(objs) != 2 or _loose(q):
        return None
    ops = []
    for o in objs:
        if o["kind"] == "matrix":
            ops.append(o)
        elif rows and o["bracket"] == "[" and o["value"]:
            ops.append(dict(o, kind="matrix", text="[[" + ", ".join(p for p in re.split(r"[,\s]+", o["text"].strip()) if p) + "]]"))
        else:
            return None
    roles, prev = [], 0
    for o in ops:
        seg, role, pos = q.raw[prev:o["start"]], None, -1
        for m in _AB_NAMED.finditer(seg):
            role, pos = (0 if m.group(1) == "A" else 1), m.start()
        for m in _AB_ROLE.finditer(seg.lower()):
            if m.start() > pos:
                role, pos = (0 if m.group(1) in ("first", "left") else 1), m.start()
        roles.append(role)
        prev = o["end"]
    if roles[0] is not None and roles[0] == roles[1]:
        return None
    if roles[0] == 1 or roles[1] == 0:
        ops = ops[::-1]
    return {"A": ops[0]["text"], "B": ops[1]["text"]}


# ============================================================================================================
# random inputs (seeded; small decimals so the exact routines stay fast and the float routines well conditioned)
# ============================================================================================================
def _rnum(rng):
    return rng.choice([rng.randint(-9, 9), rng.randint(-9, 9), round(rng.uniform(-10, 10), 1), round(rng.uniform(-5, 5), 2)])


def _rvec(rng, n):
    return [_rnum(rng) for _ in range(n)]


def _rmat(rng, m, n=None):
    return [_rvec(rng, n or m) for _ in range(m)]


def _I(sym, typ, meaning, **kw):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": kw.pop("required", True)}
    if typ != "number":
        d["type"] = typ
    d.update(kw)
    return d


def _O(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}


def _add(key, **spec):
    spec.setdefault("conditions", [])
    spec["verify"] = _safe(spec["verify"])
    allow = spec.pop("loose_ok", ())
    if [i.get("type", "number") for i in spec["inputs"]] == ["matrix"] and "parse" in spec:
        spec["parse"] = _whole(spec["parse"], allow)        # a one-matrix question states no other number
    PROCEDURES[key] = spec


# ============================================================================================================
# BATCH 1 — vectors
# ============================================================================================================
def _two_vectors(q, n=None):
    if len(q.vecs) != 2 or q.mats:
        return None
    u, v = _pick(q, "vector", "u") or q.vecs[0], _pick(q, "vector", "v") or q.vecs[1]
    if u is v:
        u, v = q.vecs
    return {"u": u["text"], "v": v["text"]}


def vector_addition(u, v):
    u, v = _need_vec(u, "u"), _need_vec(v, "v")
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries")
    return {"sum": [float(a + b) for a, b in zip(_FV(u), _FV(v))]}


_add("vector_addition",
     name="Sum of two vectors", topic="vectors / vector arithmetic",
     purpose="Add two vectors of the same length entry by entry.",
     inputs=[_I("u", "vector", "first vector"), _I("v", "vector", "second vector")],
     outputs=[_O("sum", "the vector u + v")], primary="sum",
     conditions=["u and v have the same number of entries"],
     method="entry-by-entry addition (exact decimal arithmetic)", latex=r"\mathbf{u}+\mathbf{v} = (u_1+v_1, \ldots, u_n+v_n)",
     run=vector_addition,
     verify=lambda i, o: max((abs(s - (a + b)) / max(1.0, abs(a) + abs(b)) for s, a, b in zip(o["sum"], i["u"], i["v"])),
                             default=0.0) if len(o["sum"]) == len(i["u"]) else BAD,
     reference=lambda u, v: {"sum": __import__("numpy").add(u, v).tolist()},
     wrong=lambda u, v: {"sum": [a - b for a, b in zip(u, v)]},
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 5)),
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\badd\b", r"\bsum of\b", r"\bu\s*\+\s*v\b", r"\bplus\b")
                       and q.has(r"vector") else None),
     templates=["Add the vectors (1, 2, 3) and (4, 5, 6).", "What is the sum of the vectors [2, -1] and [3, 4]?",
                "Compute u + v for the vectors u = (1, 2) and v = (3, 4)."],
     examples=[{"inputs": {"u": "1, 2, 3", "v": "4, 5, 6"}, "outputs": {"sum": [5.0, 7.0, 9.0]}},
               {"inputs": {"u": [0.1, 0.2], "v": [0.2, 0.1]}, "outputs": {"sum": [0.3, 0.3]}}],
     questions=[{"q": "Add the vectors (1, 2, 3) and (4, 5, 6).", "outputs": {"sum": [5.0, 7.0, 9.0]}},
                {"q": "What is the sum of the vectors [2, -1] and [3, 4]?", "outputs": {"sum": [5.0, 3.0]}},
                {"q": "Add the vectors (1, 2, 3) and (4, 5).", "outcome": "INVALID_INPUT"}])


def vector_subtraction(u, v):
    u, v = _need_vec(u, "u"), _need_vec(v, "v")
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries")
    return {"difference": [float(a - b) for a, b in zip(_FV(u), _FV(v))]}


_add("vector_subtraction",
     name="Difference of two vectors", topic="vectors / vector arithmetic",
     purpose="Subtract the second vector from the first, entry by entry.",
     inputs=[_I("u", "vector", "first vector"), _I("v", "vector", "vector subtracted")],
     outputs=[_O("difference", "the vector u - v")], primary="difference",
     conditions=["u and v have the same number of entries"],
     method="entry-by-entry subtraction (exact decimal arithmetic)", latex=r"\mathbf{u}-\mathbf{v} = (u_1-v_1, \ldots, u_n-v_n)",
     run=vector_subtraction,
     verify=lambda i, o: max((abs(s - (a - b)) / max(1.0, abs(a) + abs(b)) for s, a, b in zip(o["difference"], i["u"], i["v"])),
                             default=0.0) if len(o["difference"]) == len(i["u"]) else BAD,
     reference=lambda u, v: {"difference": __import__("numpy").subtract(u, v).tolist()},
     wrong=lambda u, v: {"difference": [b - a for a, b in zip(u, v)]},
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 5)),
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\bsubtract", r"\bdifference\b", r"\bu\s*-\s*v\b", r"\bminus\b")
                       and q.has(r"vector") else None),
     templates=["Subtract the vector (4, 5, 6) from (1, 2, 3).", "Find the difference of the vectors [5, 3] and [2, 7].",
                "Compute u - v for the vectors u = (1, 2) and v = (3, 4)."],
     examples=[{"inputs": {"u": "5, 3", "v": "2, 7"}, "outputs": {"difference": [3.0, -4.0]}},
               {"inputs": {"u": [1, 2, 3], "v": [4, 5, 6]}, "outputs": {"difference": [-3.0, -3.0, -3.0]}}],
     questions=[{"q": "Find the difference of the vectors [5, 3] and [2, 7].", "outputs": {"difference": [3.0, -4.0]}},
                {"q": "Compute u - v for the vectors u = (1, 2) and v = (3, 4).", "outputs": {"difference": [-2.0, -2.0]}}])


def _parse_subtract(q):
    if not q.has(r"vector") or len(q.vecs) != 2 or q.mats:
        return None
    m = re.search(r"subtract\s+(?:the\s+vector\s+)?@\s+from\s+(?:the\s+vector\s+)?@", q.rest)
    if m:                                                   # 'subtract v from u'
        return {"u": q.vecs[1]["text"], "v": q.vecs[0]["text"]}
    return _two_vectors(q) if q.has(r"\bdifference\b", r"\bu\s*-\s*v\b", r"\bminus\b", r"\bsubtract") else None


PROCEDURES["vector_subtraction"]["parse"] = _safe_parse(_parse_subtract)
PROCEDURES["vector_subtraction"]["questions"].append(
    {"q": "Subtract the vector (4, 5, 6) from (1, 2, 3).", "outputs": {"difference": [-3.0, -3.0, -3.0]}})


def scalar_vector_multiplication(k, v):
    v = _need_vec(v, "v")
    kk = _F(k, "k")
    return {"product": [float(kk * a) for a in _FV(v)]}


def _parse_scalar_vector(q):
    if len(q.vecs) != 1 or q.mats or not q.has(r"vector") or not q.has(r"\bmultiply\b", r"\btimes\b", r"\bscale\b", r"\bscalar multiple\b"):
        return None
    if q.has(r"\bdot\b", r"\bcross\b", r"\bmatrix\b"):
        return None
    k = q.named("k")
    nums = q.numbers()
    if k is None:
        if len(nums) != 1:
            return None
        k = nums[0]
    return {"k": k, "v": q.vecs[0]["text"]}


_add("scalar_vector_multiplication",
     name="Scalar multiple of a vector", topic="vectors / vector arithmetic",
     purpose="Multiply every entry of a vector by a number k.",
     inputs=[_I("k", "number", "the scalar"), _I("v", "vector", "the vector")],
     outputs=[_O("product", "the vector k v")], primary="product",
     method="entry-by-entry multiplication (exact decimal arithmetic)", latex=r"k\mathbf{v} = (k v_1, \ldots, k v_n)",
     run=scalar_vector_multiplication,
     verify=lambda i, o: max((abs(p - i["k"] * a) / max(1.0, abs(i["k"] * a)) for p, a in zip(o["product"], i["v"])),
                             default=0.0) if len(o["product"]) == len(i["v"]) else BAD,
     reference=lambda k, v: {"product": (__import__("numpy").asarray(v, dtype=float) * k).tolist()},
     wrong=lambda k, v: {"product": [a + k for a in v]},
     random_inputs=lambda rng: {"k": _rnum(rng) or 2, "v": _rvec(rng, rng.randint(2, 5))},
     parse=_safe_parse(_parse_scalar_vector),
     templates=["Multiply the vector (1, -2, 3) by 4.", "Compute 2.5 times the vector [2, 4, 6].",
                "Scale the vector (3, 1) by k = -2."],
     examples=[{"inputs": {"k": 4, "v": "1, -2, 3"}, "outputs": {"product": [4.0, -8.0, 12.0]}},
               {"inputs": {"k": 2.5, "v": [2, 4, 6]}, "outputs": {"product": [5.0, 10.0, 15.0]}}],
     questions=[{"q": "Multiply the vector (1, -2, 3) by 4.", "outputs": {"product": [4.0, -8.0, 12.0]}},
                {"q": "Compute 2.5 times the vector [2, 4, 6].", "outputs": {"product": [5.0, 10.0, 15.0]}},
                {"q": "Scale the vector (3, 1) by k = -2.", "outputs": {"product": [-6.0, -2.0]}}])


def dot_product(u, v):
    u, v = _need_vec(u, "u"), _need_vec(v, "v")
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries")
    return {"dot": float(sum((a * b for a, b in zip(_FV(u), _FV(v))), Fraction(0)))}


_add("dot_product",
     name="Dot product of two vectors", topic="vectors / products of vectors",
     purpose="The dot (scalar, inner) product u . v = sum of u_i v_i.",
     inputs=[_I("u", "vector", "first vector"), _I("v", "vector", "second vector")],
     outputs=[_O("dot", "the dot product u . v")], primary="dot",
     conditions=["u and v have the same number of entries"],
     method="sum of entry-wise products (exact decimal arithmetic)", latex=r"\mathbf{u}\cdot\mathbf{v} = \sum_i u_i v_i",
     run=dot_product,
     verify=lambda i, o: abs(o["dot"] - dot_product(i["u"], i["v"])["dot"]) / max(1.0, abs(dot_product(i["u"], i["v"])["dot"])),
     reference=lambda u, v: {"dot": float(__import__("numpy").dot(u, v))},
     wrong=lambda u, v: {"dot": math.fsum(a * b for a, b in zip(u[:-1], v[:-1]))},
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 6)),
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\bdot product\b", r"\bscalar product\b", r"\binner product\b",
                                                          r"\bu\s*\.\s*v\b", r"\bu\s*·\s*v\b") else None),
     templates=["Find the dot product of (1, 2, 3) and (4, -5, 6).", "What is the scalar product of the vectors [2, 0, 1] and [3, 4, -2]?",
                "Compute the inner product of u = (1, 1) and v = (2, -3)."],
     examples=[{"inputs": {"u": "1, 2, 3", "v": "4, -5, 6"}, "outputs": {"dot": 12.0}},
               {"inputs": {"u": [0.1, 0.2], "v": [3, 3]}, "outputs": {"dot": 0.9}}],
     questions=[{"q": "Find the dot product of (1, 2, 3) and (4, -5, 6).", "outputs": {"dot": 12.0}},
                {"q": "What is the scalar product of the vectors [2, 0, 1] and [3, 4, -2]?", "outputs": {"dot": 4.0}},
                {"q": "Find the dot product of (1, 2, 3) and (4, 5).", "outcome": "INVALID_INPUT"}])


def cross_product(u, v):
    u, v = _need_vec(u, "u", 3), _need_vec(v, "v", 3)
    a, b = _FV(u), _FV(v)
    return {"cross": [float(a[1] * b[2] - a[2] * b[1]), float(a[2] * b[0] - a[0] * b[2]), float(a[0] * b[1] - a[1] * b[0])]}


def _verify_cross(i, o):
    """w . u = 0, w . v = 0, |w|^2 = |u|^2 |v|^2 - (u.v)^2 (Lagrange) and det[u, v, w] = |w|^2 > 0 (orientation)."""
    u, v, w = i["u"], i["v"], o["cross"]
    if len(w) != 3:
        return BAD
    s = max(1.0, _norm(u) * _norm(v))
    lag = _norm(u) ** 2 * _norm(v) ** 2 - _dot(u, v) ** 2
    det = math.fsum([u[0] * (v[1] * w[2] - v[2] * w[1]), -u[1] * (v[0] * w[2] - v[2] * w[0]), u[2] * (v[0] * w[1] - v[1] * w[0])])
    return max(abs(_dot(w, u)) / (s * max(1.0, _norm(u))), abs(_dot(w, v)) / (s * max(1.0, _norm(v))),
               abs(_dot(w, w) - lag) / (s * s), abs(det - _dot(w, w)) / (s * s))


_add("cross_product",
     name="Cross product of two 3-D vectors", topic="vectors / products of vectors",
     purpose="The cross product u x v of two vectors in three dimensions (right-hand rule).",
     inputs=[_I("u", "vector", "first vector (3 entries)", min_length=3, max_length=3),
             _I("v", "vector", "second vector (3 entries)", min_length=3, max_length=3)],
     outputs=[_O("cross", "the vector u x v")], primary="cross",
     conditions=["u and v have exactly 3 entries"],
     method="determinant formula (u2 v3 - u3 v2, u3 v1 - u1 v3, u1 v2 - u2 v1), exact decimal arithmetic",
     latex=r"\mathbf{u}\times\mathbf{v} = (u_2v_3-u_3v_2,\; u_3v_1-u_1v_3,\; u_1v_2-u_2v_1)",
     run=cross_product, verify=_verify_cross,
     reference=lambda u, v: {"cross": __import__("numpy").cross(u, v).astype(float).tolist()},
     wrong=lambda u, v: {"cross": cross_product(v, u)["cross"]},
     random_inputs=lambda rng: {"u": _rvec(rng, 3), "v": _rvec(rng, 3)},
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\bcross product\b", r"\bvector product\b", r"\bu\s*x\s*v\b")
                       and not q.has(r"\btriple\b", r"\barea\b") else None),
     templates=["Find the cross product of (1, 2, 3) and (4, 5, 6).", "Compute the vector product of [1, 0, 0] and [0, 1, 0].",
                "What is u x v for u = (2, -1, 0) and v = (1, 3, 2)?"],
     examples=[{"inputs": {"u": "1, 2, 3", "v": "4, 5, 6"}, "outputs": {"cross": [-3.0, 6.0, -3.0]}},
               {"inputs": {"u": [1, 0, 0], "v": [0, 1, 0]}, "outputs": {"cross": [0.0, 0.0, 1.0]}}],
     questions=[{"q": "Find the cross product of (1, 2, 3) and (4, 5, 6).", "outputs": {"cross": [-3.0, 6.0, -3.0]}},
                {"q": "Compute the vector product of [1, 0, 0] and [0, 1, 0].", "outputs": {"cross": [0.0, 0.0, 1.0]}},
                {"q": "Find the cross product of (1, 2) and (3, 4).", "outcome": "INVALID_INPUT"}])


def vector_norm(v):
    v = _need_vec(v, "v")
    F = _FV(v)
    return {"norm": math.sqrt(float(sum((a * a for a in F), Fraction(0)))), "norm1": float(sum(abs(a) for a in F)),
            "norm_inf": float(max(abs(a) for a in F))}


def _verify_norm(i, o):
    F = _FV(i["v"])
    s = float(sum((a * a for a in F), Fraction(0)))
    return max(abs(o["norm"] ** 2 - s) / max(1.0, s), _rel(o["norm1"], float(sum(abs(a) for a in F))),
               _rel(o["norm_inf"], float(max(abs(a) for a in F))), 0.0 if o["norm"] >= 0 else BAD)


_add("vector_norm",
     name="Length (norm) of a vector", topic="vectors / length and distance",
     purpose="The Euclidean length |v| of a vector, with its 1-norm (sum of magnitudes) and max-norm.",
     inputs=[_I("v", "vector", "the vector")],
     outputs=[_O("norm", "Euclidean length |v|_2"), _O("norm1", "1-norm: sum of |v_i|"), _O("norm_inf", "max-norm: largest |v_i|")],
     primary="norm", method="square root of the sum of squares (the sum exact)", latex=r"\|\mathbf{v}\|_2 = \sqrt{\textstyle\sum_i v_i^2}",
     run=vector_norm, verify=_verify_norm,
     reference=lambda v: (lambda np: {"norm": float(np.linalg.norm(v)), "norm1": float(np.linalg.norm(v, 1)),
                                      "norm_inf": float(np.linalg.norm(v, np.inf))})(__import__("numpy")),
     wrong=lambda v: {"norm": math.fsum(x * x for x in v), "norm1": math.fsum(abs(x) for x in v), "norm_inf": max(abs(x) for x in v)},
     random_inputs=lambda rng: {"v": _rvec(rng, rng.randint(2, 6))},
     parse=_safe_parse(lambda q: {"v": q.vecs[0]["text"]} if len(q.vecs) == 1 and not q.mats
                       and q.has(r"\bnorm\b", r"\bmagnitude\b", r"\blength of\b") and q.has(r"vector")
                       and not q.has(r"\bunit\b", r"\bnormali[sz]e") else None),
     templates=["Find the magnitude of the vector (3, 4).", "What is the norm of the vector [1, 2, 2]?",
                "Compute the length of the vector (1, -1, 1, -1)."],
     examples=[{"inputs": {"v": "3, 4"}, "outputs": {"norm": 5.0, "norm1": 7.0, "norm_inf": 4.0}},
               {"inputs": {"v": [1, 2, 2]}, "outputs": {"norm": 3.0}}],
     questions=[{"q": "Find the magnitude of the vector (3, 4).", "outputs": {"norm": 5.0}},
                {"q": "What is the norm of the vector [1, 2, 2]?", "outputs": {"norm": 3.0, "norm1": 5.0}},
                {"q": "Compute the length of the vector (1, -1, 1, -1).", "outputs": {"norm": 2.0}}])


def unit_vector(v):
    v = _need_vec(v, "v")
    n = vector_norm(v)["norm"]
    if n == 0:
        raise ValueError("the zero vector has no direction, so it has no unit vector")
    return {"unit": [x / n for x in v]}


def _verify_unit(i, o):
    u, v = o["unit"], i["v"]
    if len(u) != len(v):
        return BAD
    n = _norm(v)
    return max(abs(_norm(u) - 1.0), max(abs(a * n - b) / max(1.0, n) for a, b in zip(u, v)))


_add("unit_vector",
     name="Unit vector in the direction of a vector", topic="vectors / length and distance",
     purpose="Normalize a nonzero vector: v / |v|, the vector of length 1 pointing the same way.",
     inputs=[_I("v", "vector", "the vector (not all zero)")],
     outputs=[_O("unit", "the unit vector v / |v|")], primary="unit", conditions=["v is not the zero vector"],
     method="divide each entry by the Euclidean length", latex=r"\hat{\mathbf{v}} = \mathbf{v}/\|\mathbf{v}\|",
     run=unit_vector, verify=_verify_unit,
     reference=lambda v: (lambda np: {"unit": (np.asarray(v, float) / np.linalg.norm(v)).tolist()})(__import__("numpy")),
     wrong=lambda v: {"unit": [x / math.fsum(abs(y) for y in v) for x in v]},
     random_inputs=lambda rng: {"v": _rvec(rng, rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"v": q.vecs[0]["text"]} if len(q.vecs) == 1 and not q.mats
                       and q.has(r"\bunit vector\b", r"\bnormali[sz]e") else None),
     templates=["Find the unit vector in the direction of (3, 4).", "Normalize the vector [1, 2, 2]."],
     examples=[{"inputs": {"v": "3, 4"}, "outputs": {"unit": [0.6, 0.8]}},
               {"inputs": {"v": [0, 0, 5]}, "outputs": {"unit": [0.0, 0.0, 1.0]}}],
     questions=[{"q": "Find the unit vector in the direction of (3, 4).", "outputs": {"unit": [0.6, 0.8]}},
                {"q": "Normalize the vector [1, 2, 2].", "outputs": {"unit": [1 / 3, 2 / 3, 2 / 3]}},
                {"q": "Normalize the vector (0, 0, 0).", "outcome": "INVALID_INPUT"}])


def angle_between_vectors(u, v):
    u, v = _need_vec(u, "u"), _need_vec(v, "v")
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries")
    nu, nv = _norm(u), _norm(v)
    if nu == 0 or nv == 0:
        raise ValueError("the angle with the zero vector is undefined")
    a, b = [x / nu for x in u], [x / nv for x in v]
    rad = 2 * math.atan2(_norm([x - y for x, y in zip(a, b)]), _norm([x + y for x, y in zip(a, b)]))   # Kahan's form
    return {"angle_degrees": math.degrees(rad), "angle_radians": rad}


def _verify_angle(i, o):
    """cos(theta) |u||v| = u.v and sin(theta) |u||v| = sqrt(|u|^2|v|^2 - (u.v)^2) (exact), theta in [0, pi]."""
    u, v, t = i["u"], i["v"], o["angle_radians"]
    U, V = _FV(u), _FV(v)
    uu, vv, uv = (sum((a * b for a, b in zip(X, Y)), Fraction(0)) for X, Y in ((U, U), (V, V), (U, V)))
    s = math.sqrt(float(uu)) * math.sqrt(float(vv))
    sin_side = math.sqrt(float(uu * vv - uv * uv))
    in_range = 0.0 if -1e-15 <= t <= math.pi + 1e-15 else BAD
    return max(abs(math.cos(t) * s - float(uv)) / s, abs(math.sin(t) * s - sin_side) / s,
               abs(o["angle_degrees"] - math.degrees(t)) / max(1.0, abs(o["angle_degrees"])), in_range)


def _reference_angle(u, v):
    import sympy as sp
    U = [sp.Rational(repr(float(x))) for x in u]
    V = [sp.Rational(repr(float(x))) for x in v]
    c = sum(a * b for a, b in zip(U, V)) / (sp.sqrt(sum(a * a for a in U)) * sp.sqrt(sum(b * b for b in V)))
    t = sp.acos(c)
    return {"angle_radians": float(sp.N(t, 30)), "angle_degrees": float(sp.N(t * 180 / sp.pi, 30))}


_add("angle_between_vectors",
     name="Angle between two vectors", topic="vectors / products of vectors",
     purpose="The angle between two nonzero vectors, in degrees and radians (between 0 and 180 degrees).",
     inputs=[_I("u", "vector", "first vector"), _I("v", "vector", "second vector")],
     outputs=[_O("angle_degrees", "angle in degrees"), _O("angle_radians", "angle in radians")], primary="angle_degrees",
     conditions=["u and v have the same number of entries", "neither vector is zero"],
     method="theta = 2 atan2(|a - b|, |a + b|) with a, b the unit vectors (accurate also near 0 and 180 degrees)",
     latex=r"\cos\theta = \frac{\mathbf{u}\cdot\mathbf{v}}{\|\mathbf{u}\|\,\|\mathbf{v}\|}",
     run=angle_between_vectors, verify=_verify_angle, reference=_reference_angle,
     wrong=lambda u, v: (lambda r: {"angle_degrees": r["angle_radians"], "angle_radians": r["angle_radians"]})(angle_between_vectors(u, v)),
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 4)),
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\bangle\b") and not q.has(r"\brotat") else None),
     templates=["Find the angle between the vectors (1, 0) and (1, 1).", "What is the angle between [1, 2, 3] and [4, 5, 6]?"],
     examples=[{"inputs": {"u": "1, 0", "v": "1, 1"}, "outputs": {"angle_degrees": 45.0}},
               {"inputs": {"u": [1, 0, 0], "v": [0, 1, 0]}, "outputs": {"angle_degrees": 90.0, "angle_radians": math.pi / 2}},
               {"inputs": {"u": [1, 2], "v": [-2, -4]}, "outputs": {"angle_degrees": 180.0}}],
     questions=[{"q": "Find the angle between the vectors (1, 0) and (1, 1).", "outputs": {"angle_degrees": 45.0}},
                {"q": "What is the angle between [1, 0, 0] and [0, 0, 2]?", "outputs": {"angle_degrees": 90.0}},
                {"q": "Find the angle between the vectors (0, 0) and (1, 1).", "outcome": "INVALID_INPUT"}])


def distance_between_points(p, r):
    p, r = _need_vec(p, "p"), _need_vec(r, "r")
    if len(p) != len(r):
        raise ValueError("the two points must have the same number of coordinates")
    s = sum(((a - b) ** 2 for a, b in zip(_FV(p), _FV(r))), Fraction(0))
    return {"distance": math.sqrt(float(s))}


def _parse_distance(q):
    if len(q.vecs) != 2 or q.mats or not q.has(r"\bdistance\b") or q.has(r"manhattan", r"\bline\b", r"\bplane\b"):
        return None
    return {"p": q.vecs[0]["text"], "r": q.vecs[1]["text"]}


_add("distance_between_points",
     name="Distance between two points", topic="vectors / length and distance",
     purpose="The Euclidean distance between two points given by their coordinates (any dimension).",
     inputs=[_I("p", "vector", "coordinates of the first point"), _I("r", "vector", "coordinates of the second point")],
     outputs=[_O("distance", "Euclidean distance |p - r|")], primary="distance",
     conditions=["the points have the same number of coordinates"],
     method="square root of the sum of squared coordinate differences (the sum exact)",
     latex=r"d = \sqrt{\textstyle\sum_i (p_i - r_i)^2}",
     run=distance_between_points,
     verify=lambda i, o: (lambda s: abs(o["distance"] ** 2 - s) / max(1.0, s) + (0.0 if o["distance"] >= 0 else BAD))(
         float(sum(((a - b) ** 2 for a, b in zip(_FV(i["p"]), _FV(i["r"]))), Fraction(0)))),
     reference=lambda p, r: {"distance": float(__import__("scipy.spatial.distance", fromlist=["euclidean"]).euclidean(p, r))},
     wrong=lambda p, r: {"distance": math.fsum(abs(a - b) for a, b in zip(p, r))},
     random_inputs=lambda rng: (lambda n: {"p": _rvec(rng, n), "r": _rvec(rng, n)})(rng.randint(2, 4)),
     parse=_safe_parse(_parse_distance),
     templates=["Find the distance between the points (1, 2) and (4, 6).", "What is the distance between (0, 0, 0) and (2, 3, 6)?"],
     examples=[{"inputs": {"p": "1, 2", "r": "4, 6"}, "outputs": {"distance": 5.0}},
               {"inputs": {"p": [0, 0, 0], "r": [2, 3, 6]}, "outputs": {"distance": 7.0}}],
     questions=[{"q": "Find the distance between the points (1, 2) and (4, 6).", "outputs": {"distance": 5.0}},
                {"q": "What is the distance between (0, 0, 0) and (2, 3, 6)?", "outputs": {"distance": 7.0}}])


def triple_scalar_product(u, v, w):
    u, v, w = _need_vec(u, "u", 3), _need_vec(v, "v", 3), _need_vec(w, "w", 3)
    d = _det_F([_FV(u), _FV(v), _FV(w)])
    return {"triple": float(d), "volume": float(abs(d))}


_add("triple_scalar_product",
     name="Scalar triple product (volume of a parallelepiped)", topic="vectors / products of vectors",
     purpose="u . (v x w), the signed volume of the parallelepiped spanned by three 3-D vectors, and its absolute value.",
     inputs=[_I("u", "vector", "first vector (3 entries)", min_length=3, max_length=3),
             _I("v", "vector", "second vector (3 entries)", min_length=3, max_length=3),
             _I("w", "vector", "third vector (3 entries)", min_length=3, max_length=3)],
     outputs=[_O("triple", "u . (v x w)"), _O("volume", "volume |u . (v x w)|")], primary="triple",
     conditions=["each vector has exactly 3 entries"],
     method="the determinant of the matrix with rows u, v, w (exact)", latex=r"\mathbf{u}\cdot(\mathbf{v}\times\mathbf{w}) = \det[\mathbf{u};\mathbf{v};\mathbf{w}]",
     run=triple_scalar_product, verify=_recompute(triple_scalar_product, ["triple", "volume"]),
     reference=lambda u, v, w: (lambda d: {"triple": d, "volume": abs(d)})(float(__import__("numpy").linalg.det([u, v, w]))),
     wrong=lambda u, v, w: triple_scalar_product(u, w, v),
     random_inputs=lambda rng: {"u": _rvec(rng, 3), "v": _rvec(rng, 3), "w": _rvec(rng, 3)},
     parse=_safe_parse(lambda q: {"u": q.vecs[0]["text"], "v": q.vecs[1]["text"], "w": q.vecs[2]["text"]}
                       if len(q.vecs) == 3 and not q.mats and q.has(r"\btriple\b", r"\bparallelepiped\b") else None),
     templates=["Find the scalar triple product of (1, 0, 0), (0, 1, 0) and (0, 0, 1).",
                "What is the volume of the parallelepiped spanned by (1, 2, 3), (0, 1, 4) and (5, 6, 0)?"],
     examples=[{"inputs": {"u": "1, 0, 0", "v": "0, 1, 0", "w": "0, 0, 1"}, "outputs": {"triple": 1.0, "volume": 1.0}},
               {"inputs": {"u": [1, 2, 3], "v": [0, 1, 4], "w": [5, 6, 0]}, "outputs": {"triple": 1.0}}],
     questions=[{"q": "Find the scalar triple product of (1, 0, 0), (0, 1, 0) and (0, 0, 1).", "outputs": {"triple": 1.0}},
                {"q": "What is the volume of the parallelepiped spanned by (1, 2, 3), (0, 1, 4) and (5, 6, 0)?",
                 "outputs": {"volume": 1.0}}])


def parallelogram_area(u, v):
    u, v = _need_vec(u, "u", min_n=2, max_n=3), _need_vec(v, "v", min_n=2, max_n=3)
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries (2 or 3)")
    U, V = _FV(u), _FV(v)
    uu, vv, uv = (sum((a * b for a, b in zip(X, Y)), Fraction(0)) for X, Y in ((U, U), (V, V), (U, V)))
    area = math.sqrt(float(uu * vv - uv * uv))
    return {"area": area, "triangle_area": area / 2}


_add("parallelogram_area",
     name="Area of the parallelogram (and triangle) spanned by two vectors", topic="vectors / products of vectors",
     purpose="|u x v|: the area of the parallelogram with sides u and v (2-D or 3-D), and half of it for the triangle.",
     inputs=[_I("u", "vector", "first side (2 or 3 entries)", min_length=2, max_length=3),
             _I("v", "vector", "second side (2 or 3 entries)", min_length=2, max_length=3)],
     outputs=[_O("area", "parallelogram area |u x v|"), _O("triangle_area", "triangle area |u x v| / 2")], primary="area",
     conditions=["u and v have the same number of entries, 2 or 3"],
     method="Lagrange's identity |u x v|^2 = |u|^2 |v|^2 - (u . v)^2 (exact under the root)",
     latex=r"A = \|\mathbf{u}\times\mathbf{v}\| = \sqrt{\|\mathbf{u}\|^2\|\mathbf{v}\|^2 - (\mathbf{u}\cdot\mathbf{v})^2}",
     run=parallelogram_area,
     verify=lambda i, o: (lambda r: max(abs(o["area"] ** 2 - r["area"] ** 2) / max(1.0, r["area"] ** 2),
                                        _rel(o["triangle_area"], o["area"] / 2), 0.0 if o["area"] >= 0 else BAD))(parallelogram_area(i["u"], i["v"])),
     reference=lambda u, v: (lambda np: (lambda a: {"area": a, "triangle_area": a / 2})(
         float(np.linalg.norm(np.cross(list(u) + [0] * (3 - len(u)), list(v) + [0] * (3 - len(v)))))))(__import__("numpy")),
     wrong=lambda u, v: {"area": _norm(u) * _norm(v), "triangle_area": _norm(u) * _norm(v) / 2},
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 3)),
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\barea\b") and q.has(r"parallelogram", r"triangle", r"spanned")
                       else None),
     templates=["Find the area of the parallelogram spanned by (3, 0) and (1, 2).",
                "What is the area of the triangle with sides (1, 0, 0) and (0, 2, 0)?"],
     examples=[{"inputs": {"u": "3, 0", "v": "1, 2"}, "outputs": {"area": 6.0, "triangle_area": 3.0}},
               {"inputs": {"u": [1, 0, 0], "v": [0, 2, 0]}, "outputs": {"area": 2.0, "triangle_area": 1.0}}],
     questions=[{"q": "Find the area of the parallelogram spanned by (3, 0) and (1, 2).", "outputs": {"area": 6.0}},
                {"q": "What is the area of the triangle with sides (1, 0, 0) and (0, 2, 0)?", "outputs": {"triangle_area": 1.0}}])


def vector_projection(u, v):
    u, v = _need_vec(u, "u"), _need_vec(v, "v")
    if len(u) != len(v):
        raise ValueError("u and v must have the same number of entries")
    U, V = _FV(u), _FV(v)
    vv = sum((a * a for a in V), Fraction(0))
    if vv == 0:
        raise ValueError("cannot project onto the zero vector")
    t = sum((a * b for a, b in zip(U, V)), Fraction(0)) / vv
    return {"projection": [float(t * b) for b in V], "scalar_projection": float(t) * math.sqrt(float(vv))}


def _verify_projection(i, o):
    """(u - p) . v = 0 and p parallel to v; scalar projection = p . v / |v|."""
    u, v, p = i["u"], i["v"], o["projection"]
    if len(p) != len(v):
        return BAD
    nv = _norm(v)
    t = _dot(p, v) / (nv * nv)
    s = max(1.0, _norm(u))
    return max(abs(_dot([a - b for a, b in zip(u, p)], v)) / (s * nv), max(abs(a - t * b) for a, b in zip(p, v)) / s,
               abs(o["scalar_projection"] - _dot(p, v) / nv) / s)


_add("vector_projection",
     name="Projection of one vector onto another", topic="vectors / projections",
     purpose="The orthogonal projection of u onto v, (u.v / v.v) v, and the scalar projection (component) u.v / |v|.",
     inputs=[_I("u", "vector", "vector to project"), _I("v", "vector", "direction projected onto (not zero)")],
     outputs=[_O("projection", "vector projection of u onto v"), _O("scalar_projection", "scalar projection u.v / |v|")],
     primary="projection", conditions=["u and v have the same number of entries", "v is not the zero vector"],
     method="proj = (u.v / v.v) v in exact decimal arithmetic", latex=r"\mathrm{proj}_{\mathbf{v}}\mathbf{u} = \frac{\mathbf{u}\cdot\mathbf{v}}{\mathbf{v}\cdot\mathbf{v}}\,\mathbf{v}",
     run=vector_projection, verify=_verify_projection,
     reference=lambda u, v: (lambda np: (lambda t: {"projection": (t * np.asarray(v, float)).tolist(),
                                                    "scalar_projection": t * float(np.linalg.norm(v))})(
         float(np.linalg.lstsq(np.asarray(v, float)[:, None], np.asarray(u, float), rcond=None)[0][0])))(__import__("numpy")),
     wrong=lambda u, v: {"projection": vector_projection(v, u)["projection"] if any(u) else [0.0] * len(u),
                         "scalar_projection": _dot(u, v) / _norm(u) if any(u) else 0.0},
     random_inputs=lambda rng: (lambda n: {"u": _rvec(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 4)),
     parse=_safe_parse(lambda q: (lambda m: {"u": q.vecs[0]["text"], "v": q.vecs[1]["text"]} if m else None)(
         len(q.vecs) == 2 and not q.mats and q.has(r"\bproject(?:ion)?\b") and re.search(r"@\s+onto\s+(?:the\s+vector\s+)?@", q.rest))),
     templates=["Find the projection of (2, 3) onto (1, 0).", "Project the vector [1, 2, 3] onto [1, 1, 1]."],
     examples=[{"inputs": {"u": "2, 3", "v": "1, 0"}, "outputs": {"projection": [2.0, 0.0], "scalar_projection": 2.0}},
               {"inputs": {"u": [1, 2, 3], "v": [1, 1, 1]}, "outputs": {"projection": [2.0, 2.0, 2.0]}}],
     questions=[{"q": "Find the projection of (2, 3) onto (1, 0).", "outputs": {"projection": [2.0, 0.0]}},
                {"q": "Project the vector [1, 2, 3] onto [1, 1, 1].", "outputs": {"projection": [2.0, 2.0, 2.0]}},
                {"q": "Project the vector [1, 2] onto [0, 0].", "outcome": "INVALID_INPUT"}])


def _cos_sin_deg(t):
    r = t % 360
    exact = {0: (1.0, 0.0), 90: (0.0, 1.0), 180: (-1.0, 0.0), 270: (0.0, -1.0)}
    if r in exact:
        return exact[r]
    return math.cos(math.radians(r)), math.sin(math.radians(r))


def rotate_vector_2d(v, angle):
    v = _need_vec(v, "v", 2)
    _finite(angle, "angle")
    c, s = _cos_sin_deg(angle)
    return {"rotated": [c * v[0] - s * v[1], s * v[0] + c * v[1]]}


def _verify_rotation(i, o):
    """|w| = |v|, v . w = |v|^2 cos(theta), v x w = |v|^2 sin(theta) (counterclockwise)."""
    v, w = i["v"], o["rotated"]
    if len(w) != 2:
        return BAD
    c, s = _cos_sin_deg(i["angle"])
    n2 = max(1.0, _dot(v, v))
    return max(abs(_dot(w, w) - _dot(v, v)) / n2, abs(_dot(v, w) - _dot(v, v) * c) / n2,
               abs(v[0] * w[1] - v[1] * w[0] - _dot(v, v) * s) / n2)


_add("rotate_vector_2d",
     name="Rotation of a plane vector", topic="linear transformations / transformations of the plane",
     purpose="Rotate a 2-D vector counterclockwise about the origin by an angle given in degrees.",
     inputs=[_I("v", "vector", "the vector (2 entries)", min_length=2, max_length=2), _I("angle", "number", "rotation angle in degrees (counterclockwise)")],
     outputs=[_O("rotated", "the rotated vector R(angle) v")], primary="rotated",
     conditions=["v has 2 entries", "a positive angle turns counterclockwise"],
     method="multiply by the rotation matrix [[cos t, -sin t], [sin t, cos t]] (exact at multiples of 90 degrees)",
     latex=r"R(\theta)\mathbf{v} = \begin{pmatrix}\cos\theta & -\sin\theta\\ \sin\theta & \cos\theta\end{pmatrix}\mathbf{v}",
     run=rotate_vector_2d, verify=_verify_rotation,
     reference=lambda v, angle: {"rotated": __import__("scipy.spatial.transform", fromlist=["Rotation"]).Rotation.from_euler(
         "z", angle, degrees=True).apply([v[0], v[1], 0.0])[:2].tolist()},
     wrong=lambda v, angle: rotate_vector_2d(v, -angle),
     random_inputs=lambda rng: {"v": _rvec(rng, 2), "angle": rng.choice([rng.randint(-360, 360), round(rng.uniform(-180, 180), 1), 30, 45, 60, 120])},
     parse=_safe_parse(lambda q: (lambda a: {"v": q.vecs[0]["text"], "angle": -a if q.has(r"\bclockwise\b") and not q.has(r"counter", r"anti") else a}
                                  if a is not None else None)(
         (lambda m: float(m.group(1)) if m else None)(re.search(r"(?:by|through)\s+(" + _N + r")\s*(?:degrees|deg|°)", q.text)))
         if len(q.vecs) == 1 and not q.mats and q.has(r"\brotat") else None),
     templates=["Rotate the vector (1, 0) by 90 degrees.", "Rotate the vector (2, 1) counterclockwise by 30 degrees.",
                "Rotate the vector (1, 1) clockwise by 45 degrees."],
     examples=[{"inputs": {"v": "1, 0", "angle": 90}, "outputs": {"rotated": [0.0, 1.0]}},
               {"inputs": {"v": [1, 1], "angle": -45}, "outputs": {"rotated": [math.sqrt(2), 0.0]}},
               {"inputs": {"v": [3, 4], "angle": 180}, "outputs": {"rotated": [-3.0, -4.0]}}],
     questions=[{"q": "Rotate the vector (1, 0) by 90 degrees.", "outputs": {"rotated": [0.0, 1.0]}},
                {"q": "Rotate the vector (1, 1) clockwise by 45 degrees.", "outputs": {"rotated": [math.sqrt(2), 0.0]}},
                {"q": "Rotate the vector (1, 2, 3) by 90 degrees.", "outcome": "INVALID_INPUT"}])


# ============================================================================================================
# BATCH 1 — matrices
# ============================================================================================================
def _two_matrices(q):
    if len(q.mats) != 2 or q.vecs:
        return None
    a, b = _pick(q, "matrix", "A"), _pick(q, "matrix", "B")
    if a is None or b is None or a is b:
        a, b = q.mats
    return {"A": a["text"], "B": b["text"]}


def matrix_addition(A, B):
    A, B = _need_mat(A, "A", max_dim=0), _need_mat(B, "B", max_dim=0)
    if _shape(A) != _shape(B):
        raise ValueError("A and B must have the same shape")
    return {"sum": [[float(_F(a) + _F(b)) for a, b in zip(r, s)] for r, s in zip(A, B)]}


def _verify_entrywise(op, key):
    def v(i, o):
        A, B, C = i["A"], i["B"], o[key]
        if _shape(C) != _shape(A):
            return BAD
        return max(abs(c - op(a, b)) / max(1.0, abs(a) + abs(b)) for r, s, t in zip(A, B, C) for a, b, c in zip(r, s, t))
    return v


_add("matrix_addition",
     name="Sum of two matrices", topic="matrices / matrix arithmetic",
     purpose="Add two matrices of the same shape entry by entry.",
     inputs=[_I("A", "matrix", "first matrix"), _I("B", "matrix", "second matrix")],
     outputs=[_O("sum", "the matrix A + B")], primary="sum", conditions=["A and B have the same shape"],
     method="entry-by-entry addition (exact decimal arithmetic)", latex=r"(A+B)_{ij} = a_{ij} + b_{ij}",
     run=matrix_addition, verify=_verify_entrywise(lambda a, b: a + b, "sum"),
     reference=lambda A, B: {"sum": __import__("numpy").add(A, B).astype(float).tolist()},
     wrong=lambda A, B: {"sum": [[a - b for a, b in zip(r, s)] for r, s in zip(A, B)]},
     random_inputs=lambda rng: (lambda m, n: {"A": _rmat(rng, m, n), "B": _rmat(rng, m, n)})(rng.randint(1, 4), rng.randint(1, 4)),
     parse=_safe_parse(lambda q: _two_matrices(q) if q.has(r"\badd\b", r"\bsum of\b", r"\ba\s*\+\s*b\b", r"\bplus\b") else None),
     templates=["Add the matrices [[1, 2], [3, 4]] and [[5, 6], [7, 8]].", "What is the sum of the matrices [1 2; 3 4] and [0 1; 1 0]?",
                "Compute A + B for A = [[1, 0], [0, 1]] and B = [[2, 3], [4, 5]]."],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]", "B": "[[5, 6], [7, 8]]"}, "outputs": {"sum": [[6.0, 8.0], [10.0, 12.0]]}},
               {"inputs": {"A": [[0.1, 0.2, 0.3]], "B": [[0.2, 0.1, 0.0]]}, "outputs": {"sum": [[0.3, 0.3, 0.3]]}}],
     questions=[{"q": "Add the matrices [[1, 2], [3, 4]] and [[5, 6], [7, 8]].", "outputs": {"sum": [[6.0, 8.0], [10.0, 12.0]]}},
                {"q": "What is the sum of the matrices [1 2; 3 4] and [0 1; 1 0]?", "outputs": {"sum": [[1.0, 3.0], [4.0, 4.0]]}},
                {"q": "Add the matrices [[1, 2], [3, 4]] and [[1, 2, 3]].", "outcome": "INVALID_INPUT"}])


def matrix_subtraction(A, B):
    A, B = _need_mat(A, "A", max_dim=0), _need_mat(B, "B", max_dim=0)
    if _shape(A) != _shape(B):
        raise ValueError("A and B must have the same shape")
    return {"difference": [[float(_F(a) - _F(b)) for a, b in zip(r, s)] for r, s in zip(A, B)]}


def _parse_matrix_subtract(q):
    if len(q.mats) != 2 or q.vecs:
        return None
    if re.search(r"subtract\s+(?:the\s+matrix\s+)?@\s+from\s+(?:the\s+matrix\s+)?@", q.rest):
        return {"A": q.mats[1]["text"], "B": q.mats[0]["text"]}
    return _two_matrices(q) if q.has(r"\bdifference\b", r"\ba\s*-\s*b\b", r"\bminus\b", r"\bsubtract") else None


_add("matrix_subtraction",
     name="Difference of two matrices", topic="matrices / matrix arithmetic",
     purpose="Subtract the second matrix from the first, entry by entry.",
     inputs=[_I("A", "matrix", "first matrix"), _I("B", "matrix", "matrix subtracted")],
     outputs=[_O("difference", "the matrix A - B")], primary="difference", conditions=["A and B have the same shape"],
     method="entry-by-entry subtraction (exact decimal arithmetic)", latex=r"(A-B)_{ij} = a_{ij} - b_{ij}",
     run=matrix_subtraction, verify=_verify_entrywise(lambda a, b: a - b, "difference"),
     reference=lambda A, B: {"difference": __import__("numpy").subtract(A, B).astype(float).tolist()},
     wrong=lambda A, B: {"difference": [[b - a for a, b in zip(r, s)] for r, s in zip(A, B)]},
     random_inputs=lambda rng: (lambda m, n: {"A": _rmat(rng, m, n), "B": _rmat(rng, m, n)})(rng.randint(1, 4), rng.randint(1, 4)),
     parse=_safe_parse(_parse_matrix_subtract),
     templates=["Subtract the matrix [[1, 1], [1, 1]] from [[5, 6], [7, 8]].", "Compute A - B for A = [[4, 3], [2, 1]] and B = [[1, 2], [3, 4]]."],
     examples=[{"inputs": {"A": "[[5, 6], [7, 8]]", "B": "[[1, 1], [1, 1]]"}, "outputs": {"difference": [[4.0, 5.0], [6.0, 7.0]]}},
               {"inputs": {"A": [[4, 3], [2, 1]], "B": [[1, 2], [3, 4]]}, "outputs": {"difference": [[3.0, 1.0], [-1.0, -3.0]]}}],
     questions=[{"q": "Subtract the matrix [[1, 1], [1, 1]] from [[5, 6], [7, 8]].", "outputs": {"difference": [[4.0, 5.0], [6.0, 7.0]]}},
                {"q": "Compute A - B for A = [[4, 3], [2, 1]] and B = [[1, 2], [3, 4]].", "outputs": {"difference": [[3.0, 1.0], [-1.0, -3.0]]}}])


def scalar_matrix_multiplication(k, A):
    A = _need_mat(A, "A", max_dim=0)
    kk = _F(k, "k")
    return {"product": [[float(kk * _F(a)) for a in r] for r in A]}


def _parse_scalar_matrix(q):
    if len(q.mats) != 1 or q.vecs or not q.has(r"\bmultiply\b", r"\btimes\b", r"\bscale\b", r"\bscalar multiple\b"):
        return None
    if q.has(r"\bvector\b", r"\bpower\b", r"\bsquare\b", r"\binverse\b"):
        return None
    k = q.named("k")
    nums = q.numbers()
    if k is None:
        if len(nums) != 1:
            return None
        k = nums[0]
    return {"k": k, "A": q.mats[0]["text"]}


_add("scalar_matrix_multiplication",
     name="Scalar multiple of a matrix", topic="matrices / matrix arithmetic",
     purpose="Multiply every entry of a matrix by a number k.",
     inputs=[_I("k", "number", "the scalar"), _I("A", "matrix", "the matrix")],
     outputs=[_O("product", "the matrix k A")], primary="product",
     method="entry-by-entry multiplication (exact decimal arithmetic)", latex=r"(kA)_{ij} = k\,a_{ij}",
     run=scalar_matrix_multiplication,
     verify=lambda i, o: max(abs(p - i["k"] * a) / max(1.0, abs(i["k"] * a)) for r, s in zip(i["A"], o["product"]) for a, p in zip(r, s))
     if _shape(o["product"]) == _shape(i["A"]) else BAD,
     reference=lambda k, A: {"product": (__import__("numpy").asarray(A, dtype=float) * k).tolist()},
     wrong=lambda k, A: {"product": [[a + k for a in r] for r in A]},
     random_inputs=lambda rng: {"k": _rnum(rng) or 3, "A": _rmat(rng, rng.randint(1, 4), rng.randint(1, 4))},
     parse=_safe_parse(_parse_scalar_matrix),
     templates=["Multiply the matrix [[1, 2], [3, 4]] by 3.", "Compute 0.5 times the matrix [2 4; 6 8]."],
     examples=[{"inputs": {"k": 3, "A": "[[1, 2], [3, 4]]"}, "outputs": {"product": [[3.0, 6.0], [9.0, 12.0]]}},
               {"inputs": {"k": 0.5, "A": [[2, 4], [6, 8]]}, "outputs": {"product": [[1.0, 2.0], [3.0, 4.0]]}}],
     questions=[{"q": "Multiply the matrix [[1, 2], [3, 4]] by 3.", "outputs": {"product": [[3.0, 6.0], [9.0, 12.0]]}},
                {"q": "Compute 0.5 times the matrix [2 4; 6 8].", "outputs": {"product": [[1.0, 2.0], [3.0, 4.0]]}}])


def matrix_transpose(A):
    A = _need_mat(A, "A", max_dim=0)
    return {"transpose": [[float(x) for x in c] for c in zip(*A)]}


_add("matrix_transpose",
     name="Transpose of a matrix", topic="matrices / matrix arithmetic",
     purpose="Swap rows and columns: the transpose A^T with (A^T)_ij = a_ji.",
     inputs=[_I("A", "matrix", "the matrix")], outputs=[_O("transpose", "the matrix A^T")], primary="transpose",
     method="row i of A becomes column i", latex=r"(A^{T})_{ij} = a_{ji}",
     run=matrix_transpose,
     verify=lambda i, o: max(abs(o["transpose"][c][r] - i["A"][r][c]) / max(1.0, abs(i["A"][r][c]))
                             for r in range(len(i["A"])) for c in range(len(i["A"][0])))
     if _shape(o["transpose"]) == (len(i["A"][0]), len(i["A"])) else BAD,
     reference=lambda A: {"transpose": __import__("numpy").transpose(A).astype(float).tolist()},
     wrong=lambda A: {"transpose": [[float(x) for x in r] for r in A]},
     random_inputs=lambda rng: (lambda m, n: {"A": _rmat(rng, m, n)})(*rng.choice([(2, 3), (3, 2), (2, 2), (3, 3), (1, 4), (4, 2)])),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\btranspose\b")
                       and not q.has(r"\bmultiply\b", r"\bproduct\b", r"\binverse\b") else None),
     templates=["Find the transpose of [[1, 2, 3], [4, 5, 6]].", "Transpose the matrix [1 2; 3 4]."],
     examples=[{"inputs": {"A": "[[1, 2, 3], [4, 5, 6]]"}, "outputs": {"transpose": [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]}},
               {"inputs": {"A": [[1, 2], [3, 4]]}, "outputs": {"transpose": [[1.0, 3.0], [2.0, 4.0]]}}],
     questions=[{"q": "Find the transpose of [[1, 2, 3], [4, 5, 6]].", "outputs": {"transpose": [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]]}},
                {"q": "Transpose the matrix [1 2; 3 4].", "outputs": {"transpose": [[1.0, 3.0], [2.0, 4.0]]}}])


def matrix_trace(A):
    A = _need_mat(A, "A", square=True, max_dim=0)
    return {"trace": float(sum((_F(A[i][i]) for i in range(len(A))), Fraction(0)))}


_add("matrix_trace",
     name="Trace of a square matrix", topic="matrices / matrix arithmetic",
     purpose="The trace: the sum of the diagonal entries of a square matrix.",
     inputs=[_I("A", "matrix", "a square matrix")], outputs=[_O("trace", "tr(A) = sum of a_ii")], primary="trace",
     conditions=["A is square"], method="sum of the diagonal (exact)", latex=r"\operatorname{tr}A = \sum_i a_{ii}",
     run=matrix_trace,
     verify=lambda i, o: (lambda t: abs(o["trace"] - t) / max(1.0, abs(t)))(matrix_trace(i["A"])["trace"]),
     reference=lambda A: {"trace": float(__import__("numpy").trace(A))},
     wrong=lambda A: {"trace": math.fsum(A[i][len(A) - 1 - i] for i in range(len(A)))},
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\btrace\b") else None),
     templates=["Find the trace of [[1, 2], [3, 4]].", "What is the trace of the matrix [2 0 1; 3 5 0; 1 1 -4]?"],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]"}, "outputs": {"trace": 5.0}},
               {"inputs": {"A": [[2, 0, 1], [3, 5, 0], [1, 1, -4]]}, "outputs": {"trace": 3.0}}],
     questions=[{"q": "Find the trace of [[1, 2], [3, 4]].", "outputs": {"trace": 5.0}},
                {"q": "What is the trace of the matrix [2 0 1; 3 5 0; 1 1 -4]?", "outputs": {"trace": 3.0}},
                {"q": "Find the trace of [[1, 2, 3], [4, 5, 6]].", "outcome": "INVALID_INPUT"}])


def matrix_multiplication(A, B):
    A, B = _need_mat(A, "A"), _need_mat(B, "B")
    if len(A[0]) != len(B):
        raise ValueError("the number of columns of A must equal the number of rows of B")
    return {"product": _fl(_mm_F(_FM(A), _FM(B)))}


def _verify_matmul(i, o):
    """Freivalds-style check with fixed vectors: (AB)x = A(Bx) for x = ones and x = (1, 2, 3, ...), relative to |A||B||x|."""
    A, B, C = i["A"], i["B"], o["product"]
    if _shape(C) != (len(A), len(B[0])):
        return BAD
    worst = 0.0
    for x in ([1.0] * len(B[0]), [float(j + 1) for j in range(len(B[0]))], [(-1.0) ** j for j in range(len(B[0]))]):
        lhs, rhs = _mv(C, x), _mv(A, _mv(B, x))
        scale = max(1.0, _fro(A) * _fro(B) * _norm(x))
        worst = max(worst, max(abs(a - b) for a, b in zip(lhs, rhs)) / scale)
    return worst


_add("matrix_multiplication",
     name="Product of two matrices", topic="matrices / matrix arithmetic",
     purpose="The matrix product AB (rows of A times columns of B).",
     inputs=[_I("A", "matrix", "left matrix"), _I("B", "matrix", "right matrix")],
     outputs=[_O("product", "the matrix AB")], primary="product",
     conditions=["columns of A = rows of B", "at most 10 rows and columns"],
     method="row-by-column sums in exact decimal arithmetic", latex=r"(AB)_{ij} = \sum_k a_{ik} b_{kj}",
     run=matrix_multiplication, verify=_verify_matmul,
     reference=lambda A, B: {"product": __import__("numpy").matmul(A, B).astype(float).tolist()},
     wrong=lambda A, B: {"product": [[math.fsum(A[r][k] * B[k][c] for k in range(len(B) - 1)) for c in range(len(B[0]))] for r in range(len(A))]},
     random_inputs=lambda rng: (lambda m, n, p: {"A": _rmat(rng, m, n), "B": _rmat(rng, n, p)})(rng.randint(1, 4), rng.randint(2, 4), rng.randint(1, 4)),
     # two matrices; a list in square brackets is a one-row matrix when the question calls both operands matrices
     parse=_safe_parse(lambda q: _operands(q, rows=_matrix_words(q) >= 2)
                       if q.has(r"\bmultiply\b", r"\bmultiplying\b", r"\bproduct\b", r"\bab\b", r"\ba\s*\*\s*b\b", r"\btimes\b")
                       and not q.has(r"\bkronecker\b", r"\btensor\b", r"\bhadamard\b", r"\bentry-?wise\b", r"element-?wise",
                                     r"\bouter product\b", r"\bdot product\b", r"\binner product\b", r"\bba\b", r"\bb\s*\*\s*a\b",
                                     r"\btranspose", r"\binverse\b", r"\bdeterminant\b", r"\btrace\b", r"\beigen", r"\brank\b",
                                     r"\bnorms?\b", r"\bpower\b", r"\^") else None),
     templates=["Multiply the matrices [[1, 2], [3, 4]] and [[5, 6], [7, 8]].", "Find the product AB for A = [[1, 0, 2]] and B = [[1], [2], [3]].",
                "What is the product of [1 2; 3 4] and [0 1; 1 0]?"],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]", "B": "[[5, 6], [7, 8]]"}, "outputs": {"product": [[19.0, 22.0], [43.0, 50.0]]}},
               {"inputs": {"A": [[1, 0, 2]], "B": [[1], [2], [3]]}, "outputs": {"product": [[7.0]]}}],
     questions=[{"q": "Multiply the matrices [[1, 2], [3, 4]] and [[5, 6], [7, 8]].", "outputs": {"product": [[19.0, 22.0], [43.0, 50.0]]}},
                {"q": "What is the product of [1 2; 3 4] and [0 1; 1 0]?", "outputs": {"product": [[2.0, 1.0], [4.0, 3.0]]}},
                {"q": "Multiply the matrices [[1, 2], [3, 4]] and [[1, 2, 3]].", "outcome": "INVALID_INPUT"}])


def matrix_vector_product(A, x):
    A, x = _need_mat(A, "A"), _need_vec(x, "x")
    if len(A[0]) != len(x):
        raise ValueError("the number of columns of A must equal the length of x")
    X = _FV(x)
    return {"image": [float(sum((_F(a) * b for a, b in zip(r, X)), Fraction(0))) for r in A]}


def _parse_matvec(q):
    if len(q.mats) != 1 or len(q.vecs) != 1 or not q.has(r"\bmultiply\b", r"\bproduct\b", r"\bimage\b", r"\bapply\b", r"\btimes\b",
                                                          r"\btransform", r"\bax\b"):
        return None
    if q.has(r"\bsolve\b", r"\bsystem\b", r"\bleast[- ]squares\b", r"\bpower\b", r"\beigen", r"\bbasis\b", r"\bcoordinates?\b",
             r"\bsolutions?\b", r"\bconsistent\b", r"\binconsistent\b", r"\bclassify\b"):
        return None
    # a product of two MATRICES (the list is called a matrix: 'the first/left matrix [1 2 3]'), a Kronecker or outer
    # product, or a product written AB / BA is not a matrix times a vector
    if _matrix_words(q) >= 2 or q.has(r"\bkronecker\b", r"\btensor\b", r"\bouter\b", r"\bhadamard\b", r"\bab\b", r"\bba\b",
                                      r"\brow vector\b", r"\bvector-matrix\b", r"\bx\s*\^?\s*t\s*a\b", r"\bleft\b"):
        return None
    if _loose(q):
        return None
    return {"A": q.mats[0]["text"], "x": q.vecs[0]["text"]}


_add("matrix_vector_product",
     name="Matrix times vector (image under a linear map)", topic="linear transformations / matrix-vector product",
     purpose="The vector Ax: the image of x under the linear transformation with matrix A.",
     inputs=[_I("A", "matrix", "the matrix"), _I("x", "vector", "the vector")],
     outputs=[_O("image", "the vector Ax")], primary="image", conditions=["columns of A = length of x"],
     method="dot product of each row with x (exact decimal arithmetic)", latex=r"(A\mathbf{x})_i = \sum_j a_{ij} x_j",
     run=matrix_vector_product,
     verify=lambda i, o: max(abs(y - _dot(r, i["x"])) / max(1.0, _norm(r) * _norm(i["x"])) for r, y in zip(i["A"], o["image"]))
     if len(o["image"]) == len(i["A"]) else BAD,
     reference=lambda A, x: {"image": __import__("numpy").dot(A, x).astype(float).tolist()},
     wrong=lambda A, x: {"image": [math.fsum(a * b for a, b in zip(r[:-1], x[:-1])) for r in A]},
     random_inputs=lambda rng: (lambda m, n: {"A": _rmat(rng, m, n), "x": _rvec(rng, n)})(rng.randint(1, 4), rng.randint(2, 4)),
     parse=_safe_parse(_parse_matvec),
     templates=["Multiply the matrix [[1, 2], [3, 4]] by the vector (5, 6).", "Find the image of the vector [1, 0, -1] under the matrix [1 2 3; 4 5 6]."],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]", "x": "5, 6"}, "outputs": {"image": [17.0, 39.0]}},
               {"inputs": {"A": [[1, 2, 3], [4, 5, 6]], "x": [1, 0, -1]}, "outputs": {"image": [-2.0, -2.0]}}],
     questions=[{"q": "Multiply the matrix [[1, 2], [3, 4]] by the vector (5, 6).", "outputs": {"image": [17.0, 39.0]}},
                {"q": "Find the image of the vector [1, 0, -1] under the matrix [1 2 3; 4 5 6].", "outputs": {"image": [-2.0, -2.0]}},
                {"q": "Multiply the matrix [[1, 2], [3, 4]] by the vector (5, 6, 7).", "outcome": "INVALID_INPUT"}])


def outer_product(u, v):
    u, v = _need_vec(u, "u", max_n=MAX_DIM), _need_vec(v, "v", max_n=MAX_DIM)
    U, V = _FV(u), _FV(v)
    return {"outer": [[float(a * b) for b in V] for a in U]}


_add("outer_product",
     name="Outer product of two vectors", topic="vectors / products of vectors",
     purpose="The matrix u v^T with entries u_i v_j.",
     inputs=[_I("u", "vector", "first vector (rows)"), _I("v", "vector", "second vector (columns)")],
     outputs=[_O("outer", "the matrix u v^T")], primary="outer",
     method="entry (i, j) = u_i v_j (exact)", latex=r"(\mathbf{u}\mathbf{v}^{T})_{ij} = u_i v_j",
     run=outer_product,
     verify=lambda i, o: max(abs(o["outer"][r][c] - i["u"][r] * i["v"][c]) / max(1.0, abs(i["u"][r] * i["v"][c]))
                             for r in range(len(i["u"])) for c in range(len(i["v"])))
     if _shape(o["outer"]) == (len(i["u"]), len(i["v"])) else BAD,
     reference=lambda u, v: {"outer": __import__("numpy").outer(u, v).astype(float).tolist()},
     wrong=lambda u, v: {"outer": [[float(a * b) for a in u] for b in v]},
     random_inputs=lambda rng: {"u": _rvec(rng, rng.randint(2, 4)), "v": _rvec(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: _two_vectors(q) if q.has(r"\bouter product\b") else None),
     templates=["Find the outer product of (1, 2) and (3, 4, 5).", "Compute the outer product of u = [1, -1] and v = [2, 3]."],
     examples=[{"inputs": {"u": "1, 2", "v": "3, 4, 5"}, "outputs": {"outer": [[3.0, 4.0, 5.0], [6.0, 8.0, 10.0]]}},
               {"inputs": {"u": [1, -1], "v": [2, 3]}, "outputs": {"outer": [[2.0, 3.0], [-2.0, -3.0]]}}],
     questions=[{"q": "Find the outer product of (1, 2) and (3, 4, 5).", "outputs": {"outer": [[3.0, 4.0, 5.0], [6.0, 8.0, 10.0]]}}])


def kronecker_product(A, B):
    A, B = _need_mat(A, "A", max_dim=6), _need_mat(B, "B", max_dim=6)
    FA, FB = _FM(A), _FM(B)
    m, n, p, r = len(A), len(A[0]), len(B), len(B[0])
    return {"kron": [[float(FA[i // p][j // r] * FB[i % p][j % r]) for j in range(n * r)] for i in range(m * p)]}


_add("kronecker_product",
     name="Kronecker (tensor) product of two matrices", topic="matrices / matrix arithmetic",
     purpose="The block matrix A (x) B whose (i, j) block is a_ij B.",
     inputs=[_I("A", "matrix", "first matrix"), _I("B", "matrix", "second matrix")],
     outputs=[_O("kron", "the Kronecker product A (x) B")], primary="kron",
     conditions=["each matrix at most 6 x 6"], method="block (i, j) = a_ij B (exact)", latex=r"A\otimes B = [a_{ij}B]",
     run=kronecker_product, verify=_recompute(kronecker_product, ["kron"]),
     reference=lambda A, B: {"kron": __import__("numpy").kron(A, B).astype(float).tolist()},
     wrong=lambda A, B: kronecker_product(B, A),
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(1, 3), rng.randint(2, 3)), "B": _rmat(rng, rng.randint(2, 3), rng.randint(1, 3))},
     # a list written in square brackets, [1 2 3], is the one-row matrix it is written as
     parse=_safe_parse(lambda q: _operands(q, rows=True) if q.has(r"\bkronecker\b", r"\btensor product\b")
                       and not q.has(r"\bsum\b", r"\beigen", r"\bdeterminant\b", r"\btrace\b", r"\brank\b", r"\binverse\b",
                                     r"\bnorms?\b") else None),
     templates=["Find the Kronecker product of [[1, 2], [3, 4]] and [[0, 1], [1, 0]].", "Compute the tensor product of [[1, 2]] and [[1], [1]]."],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]", "B": "[[0, 1], [1, 0]]"},
                "outputs": {"kron": [[0.0, 1.0, 0.0, 2.0], [1.0, 0.0, 2.0, 0.0], [0.0, 3.0, 0.0, 4.0], [3.0, 0.0, 4.0, 0.0]]}},
               {"inputs": {"A": [[1, 2]], "B": [[1], [1]]}, "outputs": {"kron": [[1.0, 2.0], [1.0, 2.0]]}}],
     questions=[{"q": "Find the Kronecker product of [[1, 2], [3, 4]] and [[0, 1], [1, 0]].",
                 "outputs": {"kron": [[0.0, 1.0, 0.0, 2.0], [1.0, 0.0, 2.0, 0.0], [0.0, 3.0, 0.0, 4.0], [3.0, 0.0, 4.0, 0.0]]}}])


def matrix_power(A, k):
    A = _need_mat(A, "A", square=True, max_dim=6)
    if isinstance(k, bool) or not isinstance(k, (int, float)) or k != int(k) or k < 0 or k > 64:
        raise ValueError("k must be a whole number from 0 to 64")
    k, n = int(k), len(A)
    R = [[Fraction(int(i == j)) for j in range(n)] for i in range(n)]
    P = _FM(A)
    while k:                                           # binary powering, exact
        if k & 1:
            R = _mm_F(R, P)
        k >>= 1
        if k:
            P = _mm_F(P, P)
    out = _fl(R)
    if any(not math.isfinite(x) for r in out for x in r):
        raise ValueError("the entries of A^k are too large for floating-point numbers")
    return {"power": out}


def _verify_power(i, o):
    """Sequential products A, A^2, ..., A^k (exact), a different route from binary powering."""
    A, k = _FM(i["A"]), int(i["k"])
    n = len(A)
    R = [[Fraction(int(r == c)) for c in range(n)] for r in range(n)]
    for _ in range(k):
        R = _mm_F(R, A)
    return _diff(o["power"], _fl(R))


_add("matrix_power",
     name="Power of a square matrix", topic="matrices / matrix arithmetic",
     purpose="A^k for a square matrix A and a whole number k (A^0 = I).",
     inputs=[_I("A", "matrix", "a square matrix (at most 6 x 6)"), _I("k", "integer", "the exponent (0 to 64)", min=0, max=64)],
     outputs=[_O("power", "the matrix A^k")], primary="power", conditions=["A is square", "k is a whole number, 0 <= k <= 64"],
     method="binary powering (repeated squaring) in exact decimal arithmetic", latex=r"A^k = \underbrace{A A \cdots A}_{k}",
     run=matrix_power, verify=_safe(_verify_power),
     reference=lambda A, k: {"power": __import__("numpy").linalg.matrix_power(__import__("numpy").asarray(A, dtype=float), int(k)).tolist()},
     wrong=lambda A, k: {"power": [[float(x) ** int(k) for x in r] for r in A]},
     random_inputs=lambda rng: {"A": [[rng.randint(-5, 5) for _ in range(n)] for n in [rng.randint(2, 3)] for _ in range(n)], "k": rng.randint(2, 8)},
     parse=_safe_parse(lambda q: (lambda k: {"A": q.mats[0]["text"], "k": k} if k is not None else None)(
         (lambda m: float(m.group(1)) if m else (2.0 if q.has(r"\bsquare of\b", r"\bsquare the matrix\b") else q.named("k")))(
             re.search(r"(?:power|\^)\s*(?:of\s*)?(\d+)", q.rest) or re.search(r"\bto the (\d+)(?:st|nd|rd|th)? power\b", q.rest)))
         if len(q.mats) == 1 and not q.vecs and q.has(r"\bpower\b", r"\^\s*\d", r"\bsquare of\b", r"\bsquare the matrix\b")
         and not q.has(r"\bpower (?:method|iteration)\b", r"\beigen",
                       # a power of an unknown X (X^2 = A asks for a square root), or a further task done on the power
                       r"\bx\s*(?:\^|\*\*)", r"\bsquare root\b", r"\bsqrt", r"\bsuch that\b", r"\bsatisf", r"\btrace\b",
                       r"\bdeterminant\b", r"\bdet\b", r"\binverse\b", r"\brank\b", r"\bnorms?\b", r"\bexponential\b") else None),
     templates=["Compute A^3 for A = [[1, 1], [0, 1]].", "Raise the matrix [[2, 0], [1, 1]] to the 5th power.",
                "Find the square of the matrix [1 2; 3 4]."],
     examples=[{"inputs": {"A": "[[1, 1], [0, 1]]", "k": 3}, "outputs": {"power": [[1.0, 3.0], [0.0, 1.0]]}},
               {"inputs": {"A": [[1, 2], [3, 4]], "k": 2}, "outputs": {"power": [[7.0, 10.0], [15.0, 22.0]]}},
               {"inputs": {"A": [[1, 2], [3, 4]], "k": 0}, "outputs": {"power": [[1.0, 0.0], [0.0, 1.0]]}}],
     questions=[{"q": "Compute A^3 for A = [[1, 1], [0, 1]].", "outputs": {"power": [[1.0, 3.0], [0.0, 1.0]]}},
                {"q": "Find the square of the matrix [1 2; 3 4].", "outputs": {"power": [[7.0, 10.0], [15.0, 22.0]]}},
                {"q": "Raise the matrix [[2, 0], [1, 1]] to the 5th power.", "outputs": {"power": [[32.0, 0.0], [31.0, 1.0]]}},
                {"q": "Compute A^2 for A = [[1, 2, 3], [4, 5, 6]].", "outcome": "INVALID_INPUT"}])


def matrix_norms(A):
    A = _need_mat(A, "A")
    F = _FM(A)
    fro = math.sqrt(float(sum((x * x for r in F for x in r), Fraction(0))))
    one = float(max(sum((abs(r[j]) for r in F), Fraction(0)) for j in range(len(A[0]))))
    inf = float(max(sum((abs(x) for x in r), Fraction(0)) for r in F))
    return {"frobenius": fro, "norm1": one, "norm_inf": inf, "norm2": singular_values(A)["singular_values"][0]}


# ============================================================================================================
# BATCH 1 — determinants and inverses
# ============================================================================================================
def _hadamard(A):
    """Hadamard's bound on |det A|: the product of the row lengths (a scale for determinant residuals)."""
    p = 1.0
    for r in A:
        p *= max(_norm(r), 1e-300)
    return p


def _verify_det_value(i, o, key="det"):
    want = float(_det_F(_FM(i["A"])))
    return abs(o[key] - want) / max(1.0, abs(want))


def determinant_2x2(A):
    A = _need_mat(A, "A", square=True, rows=2)
    F = _FM(A)
    return {"det": float(F[0][0] * F[1][1] - F[0][1] * F[1][0])}


def _parse_det(size_ok):
    def p(q):
        if len(q.mats) != 1 or q.vecs or not q.has(r"\bdeterminant\b", r"\bdet\b"):
            return None
        if q.has(r"\bminor\b", r"\bcofactor\b", r"\bcharacteristic\b", r"\bleading\b", r"\beigen", r"\binverse\b", r"\badjugate\b"):
            return None
        o = q.mats[0]
        d = _dims(o)
        if d is None:
            return {"A": o["text"]} if size_ok(None) else None
        if d[0] != d[1]:
            return {"A": o["text"]} if size_ok(0) else None     # not square: the general reader reports it
        return {"A": o["text"]} if size_ok(d[0]) else None
    return p


_add("determinant_2x2",
     name="Determinant of a 2 x 2 matrix", topic="determinants / small determinants",
     purpose="det [[a, b], [c, d]] = ad - bc.",
     inputs=[_I("A", "matrix", "a 2 x 2 matrix")], outputs=[_O("det", "the determinant ad - bc")], primary="det",
     conditions=["A is 2 x 2"], method="ad - bc (exact decimal arithmetic)",
     latex=r"\det\begin{pmatrix}a&b\\c&d\end{pmatrix} = ad - bc",
     run=determinant_2x2, verify=_verify_det_value,
     reference=lambda A: {"det": float(__import__("numpy").linalg.det(A))},
     wrong=lambda A: {"det": A[0][0] * A[1][1] + A[0][1] * A[1][0]},
     random_inputs=lambda rng: {"A": _rmat(rng, 2)},
     parse=_safe_parse(_parse_det(lambda n: n == 2)),
     templates=["Find the determinant of [[1, 2], [3, 4]].", "What is det [3 8; 4 6]?"],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]"}, "outputs": {"det": -2.0}},
               {"inputs": {"A": [[3, 8], [4, 6]]}, "outputs": {"det": -14.0}},
               {"inputs": {"A": [[0.1, 0.2], [0.3, 0.4]]}, "outputs": {"det": -0.02}}],
     questions=[{"q": "Find the determinant of [[1, 2], [3, 4]].", "outputs": {"det": -2.0}},
                {"q": "What is det [3 8; 4 6]?", "outputs": {"det": -14.0}}])


def determinant_3x3(A):
    A = _need_mat(A, "A", square=True, rows=3)
    a = _FM(A)
    return {"det": float(a[0][0] * a[1][1] * a[2][2] + a[0][1] * a[1][2] * a[2][0] + a[0][2] * a[1][0] * a[2][1]
                         - a[0][2] * a[1][1] * a[2][0] - a[0][0] * a[1][2] * a[2][1] - a[0][1] * a[1][0] * a[2][2])}


_add("determinant_3x3",
     name="Determinant of a 3 x 3 matrix", topic="determinants / small determinants",
     purpose="The determinant of a 3 x 3 matrix by the rule of Sarrus.",
     inputs=[_I("A", "matrix", "a 3 x 3 matrix")], outputs=[_O("det", "the determinant")], primary="det",
     conditions=["A is 3 x 3"], method="rule of Sarrus: three down-diagonal products minus three up-diagonal products (exact)",
     latex=r"\det A = a_{11}a_{22}a_{33} + a_{12}a_{23}a_{31} + a_{13}a_{21}a_{32} - a_{13}a_{22}a_{31} - a_{11}a_{23}a_{32} - a_{12}a_{21}a_{33}",
     run=determinant_3x3, verify=_verify_det_value,
     reference=lambda A: {"det": float(__import__("numpy").linalg.det(A))},
     wrong=lambda A: {"det": A[0][0] * A[1][1] * A[2][2] + A[0][1] * A[1][2] * A[2][0] + A[0][2] * A[1][0] * A[2][1]
                      + A[0][2] * A[1][1] * A[2][0] + A[0][0] * A[1][2] * A[2][1] + A[0][1] * A[1][0] * A[2][2]},
     random_inputs=lambda rng: {"A": _rmat(rng, 3)},
     parse=_safe_parse(_parse_det(lambda n: n == 3)),
     templates=["Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].", "Compute det [1 2 3; 4 5 6; 7 8 10]."],
     examples=[{"inputs": {"A": "[[2, 0, 1], [1, 3, 2], [1, 1, 2]]"}, "outputs": {"det": 6.0}},
               {"inputs": {"A": [[1, 2, 3], [4, 5, 6], [7, 8, 10]]}, "outputs": {"det": -3.0}},
               {"inputs": {"A": [[1, 2, 3], [4, 5, 6], [7, 8, 9]]}, "outputs": {"det": 0.0}}],
     questions=[{"q": "Find the determinant of [[2, 0, 1], [1, 3, 2], [1, 1, 2]].", "outputs": {"det": 6.0}},
                {"q": "Compute det [1 2 3; 4 5 6; 7 8 10].", "outputs": {"det": -3.0}}])


def determinant(A):
    A = _need_mat(A, "A", square=True)
    return {"det": float(_det_F(_FM(A)))}


def _reference_det(A):
    import sympy as sp
    return {"det": float(sp.Matrix([[sp.Rational(repr(float(x))) for x in r] for r in A]).det())}


_add("determinant",
     name="Determinant of an n x n matrix", topic="determinants / general determinants",
     purpose="The determinant of a square matrix of any size up to 10 x 10, by row reduction.",
     inputs=[_I("A", "matrix", "a square matrix (up to 10 x 10)")], outputs=[_O("det", "the determinant")], primary="det",
     conditions=["A is square", "at most 10 x 10"],
     method="Gaussian elimination with partial pivoting in exact rational arithmetic: det = (-1)^swaps x product of pivots",
     latex=r"\det A = (-1)^{s}\prod_i u_{ii}",
     run=determinant, verify=_verify_det_value, reference=_reference_det,
     wrong=lambda A: {"det": math.prod(A[i][i] for i in range(len(A)))},
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(4, 6))},
     parse=_safe_parse(_parse_det(lambda n: n is None or n == 0 or n == 1 or n >= 4)),
     templates=["Find the determinant of [[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 5, 6], [0, 0, 7, 8]].",
                "Compute det [2 1 0 0; 1 2 1 0; 0 1 2 1; 0 0 1 2]."],
     examples=[{"inputs": {"A": "[[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 5, 6], [0, 0, 7, 8]]"}, "outputs": {"det": 4.0}},
               {"inputs": {"A": [[2, 1, 0, 0], [1, 2, 1, 0], [0, 1, 2, 1], [0, 0, 1, 2]]}, "outputs": {"det": 5.0}},
               {"inputs": {"A": [[1, 2], [3, 4]]}, "outputs": {"det": -2.0}}],
     questions=[{"q": "Find the determinant of [[1, 2, 0, 0], [3, 4, 0, 0], [0, 0, 5, 6], [0, 0, 7, 8]].", "outputs": {"det": 4.0}},
                {"q": "Compute det [2 1 0 0; 1 2 1 0; 0 1 2 1; 0 0 1 2].", "outputs": {"det": 5.0}},
                {"q": "Find the determinant of [[1, 2, 3], [4, 5, 6]].", "outcome": "INVALID_INPUT"}])


def matrix_inverse(A):
    A = _need_mat(A, "A", square=True)
    return {"inverse": _fl(_inv_F(_FM(A)))}


def _verify_inverse(i, o):
    """A X = I and X A = I, relative to |A| |X| (a correctly rounded inverse leaves a residual near machine precision)."""
    A, X = i["A"], o["inverse"]
    n = len(A)
    if _shape(X) != (n, n):
        return BAD
    s = max(1.0, _fro(A) * _fro(X))
    I = _eye(n)
    r1 = max(abs(a - b) for ra, rb in zip(_mm(A, X), I) for a, b in zip(ra, rb))
    r2 = max(abs(a - b) for ra, rb in zip(_mm(X, A), I) for a, b in zip(ra, rb))
    return max(r1, r2) / s


_add("matrix_inverse",
     name="Inverse of a square matrix", topic="matrices / inverses", loose_ok=(r"\^\s*\(?\s*-\s*1\s*\)?", r"\*\*\s*\(?\s*-\s*1\s*\)?"),
     purpose="The inverse A^-1 of a nonsingular square matrix (any size up to 10 x 10).",
     inputs=[_I("A", "matrix", "a nonsingular square matrix")], outputs=[_O("inverse", "the matrix A^-1")], primary="inverse",
     conditions=["A is square", "det A != 0 (a singular matrix has no inverse)", "at most 10 x 10"],
     method="Gauss-Jordan elimination on [A | I] in exact rational arithmetic", latex=r"A A^{-1} = A^{-1} A = I",
     run=matrix_inverse, verify=_verify_inverse,
     reference=lambda A: {"inverse": __import__("numpy").linalg.inv(__import__("numpy").asarray(A, dtype=float)).tolist()},
     wrong=lambda A: {"inverse": _T(matrix_inverse(A)["inverse"])},
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\binverse\b", r"\binvert\b", r"\^\s*-\s*1")
                       and not q.has(r"\bpseudo", r"\bmoore", r"\biteration\b", r"\bpower\b") else None),
     templates=["Find the inverse of [[4, 7], [2, 6]].", "Invert the matrix [1 2 3; 0 1 4; 5 6 0].", "Compute A^-1 for A = [[2, 1], [1, 1]]."],
     examples=[{"inputs": {"A": "[[4, 7], [2, 6]]"}, "outputs": {"inverse": [[0.6, -0.7], [-0.2, 0.4]]}},
               {"inputs": {"A": [[1, 2, 3], [0, 1, 4], [5, 6, 0]]}, "outputs": {"inverse": [[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]]}}],
     questions=[{"q": "Find the inverse of [[4, 7], [2, 6]].", "outputs": {"inverse": [[0.6, -0.7], [-0.2, 0.4]]}},
                {"q": "Compute A^-1 for A = [[2, 1], [1, 1]].", "outputs": {"inverse": [[1.0, -1.0], [-1.0, 2.0]]}},
                {"q": "Find the inverse of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


# ============================================================================================================
# BATCH 2 — linear systems, rank and the fundamental subspaces
# ============================================================================================================
_TERM = r"(?:\d+(?:\.\d+)?\s*\*?\s*)?[xyz]"
_LHS = r"[-+]?\s*" + _TERM + r"(?:\s*[-+]\s*" + _TERM + r")*"
_EQ = re.compile(r"(?<![\w.^*/])(" + _LHS + r")\s*=\s*([-+]?\s*\d+(?:\.\d+)?)(?!\w|\.\d)")


def _equations(q):
    """Linear equations in x, y, z written in the question: (A rows, b, variables) or None. Every '=' of the question
    must belong to a read equation (never read part of the text)."""
    text = q.text
    eqs = _EQ.findall(text)
    if not eqs or text.count("=") != len(eqs):
        return None
    rows, rhs, names = [], [], set()
    for lhs, r in eqs:
        co = {}
        for sign, num, var in re.findall(r"([-+]?)\s*(\d+(?:\.\d+)?)?\s*\*?\s*([xyz])", lhs):
            co[var] = co.get(var, 0.0) + (-1.0 if sign == "-" else 1.0) * (float(num) if num else 1.0)
            names.add(var)
        rows.append(co)
        rhs.append(float(r.replace(" ", "")))
    var = sorted(names)
    A = [[row.get(v, 0.0) for v in var] for row in rows]
    return A, rhs, var


def _mat_text(A):
    return "; ".join(" ".join(repr(float(x)) for x in r) for r in A)


def _vec_text(v):
    return ", ".join(repr(float(x)) for x in v)


_NOT_SOLVE = (r"\bhow many solutions\b", r"\bconsistent\b", r"\binconsistent\b", r"\bclassify\b", r"\bnumber of solutions\b",
              r"\bleast[- ]squares\b", r"\bjacobi\b", r"\bgauss-?seidel\b", r"\bconjugate gradients?\b", r"\btridiagonal\b",
              r"\biterat", r"\bcoordinates?\b")


def _system_from_equations(q, nvars):
    e = _equations(q)
    if e is None or q.objs:
        return None
    A, b, var = e
    if var != ["x", "y", "z"][:nvars] or len(A) != nvars:
        return None
    return {"A": _mat_text(A), "b": _vec_text(b)}


def _verify_solution(i, o, key="solution"):
    """|A x - b| relative to |A| |x| + |b|: a correctly rounded solution leaves a residual near machine precision."""
    A, b, x = i["A"], i["b"], o[key]
    if len(x) != len(A[0]):
        return BAD
    r = [a - c for a, c in zip(_mv(A, x), b)]
    return _norm(r) / max(1e-300, _fro(A) * _norm(x) + _norm(b))


def _cramer(A, b, n):
    A, b = _need_mat(A, "A", square=True, rows=n), _need_vec(b, "b", n)
    FA, Fb = _FM(A), _FV(b)
    d = _det_F(FA)
    if d == 0:
        raise ValueError("the determinant of the coefficient matrix is 0: the system has no unique solution")
    sol = []
    for j in range(n):
        Aj = [[Fb[r] if c == j else FA[r][c] for c in range(n)] for r in range(n)]
        sol.append(float(_det_F(Aj) / d))
    return {"solution": sol, "det": float(d)}


def solve_2x2_system(A, b):
    return _cramer(A, b, 2)


def solve_3x3_system(A, b):
    return _cramer(A, b, 3)


def _parse_cramer(n):
    def p(q):
        if q.has(*_NOT_SOLVE):
            return None
        if q.objs:
            if not q.has(r"\bcramer"):
                return None
            A, b = _pick(q, "matrix", "A"), _pick(q, "vector", "b")
            if len(q.mats) != 1 or len(q.vecs) != 1 or not _is_square(A, n):
                return None
            return {"A": A["text"], "b": b["text"]}
        if not q.has(r"\bsolve\b", r"\bsolution\b", r"\bfind x\b", r"\bcramer"):
            return None
        return _system_from_equations(q, n)
    return p


_SYS_IN = [_I("A", "matrix", "coefficient matrix"), _I("b", "vector", "right-hand side")]
_add("solve_2x2_system",
     name="Two linear equations in two unknowns", topic="linear systems / small systems",
     purpose="Solve a x + b y = e, c x + d y = f for (x, y) by Cramer's rule.",
     inputs=_SYS_IN, outputs=[_O("solution", "the solution (x, y)"), _O("det", "determinant of the coefficient matrix")],
     primary="solution", conditions=["A is 2 x 2", "det A != 0 (otherwise there is no unique solution)"],
     method="Cramer's rule x_j = det(A_j)/det(A) in exact decimal arithmetic",
     latex=r"x = \frac{\begin{vmatrix} e & b\\ f & d\end{vmatrix}}{\begin{vmatrix} a & b\\ c & d\end{vmatrix}},\quad y = \frac{\begin{vmatrix} a & e\\ c & f\end{vmatrix}}{\begin{vmatrix} a & b\\ c & d\end{vmatrix}}",
     run=solve_2x2_system, verify=lambda i, o: max(_verify_solution(i, o), _verify_det_value(i, o)),
     reference=lambda A, b: {"solution": __import__("numpy").linalg.solve(__import__("numpy").asarray(A, float), b).tolist()},
     wrong=lambda A, b: (lambda r: {"solution": r["solution"][::-1], "det": r["det"]})(solve_2x2_system(A, b)),
     random_inputs=lambda rng: {"A": _rmat(rng, 2), "b": _rvec(rng, 2)},
     parse=_safe_parse(_parse_cramer(2)),
     templates=["Solve the system 2x + 3y = 7, x - y = 1.", "Solve 3x - y = 5 and x + 2y = 4.",
                "Use Cramer's rule to solve A = [[2, 3], [1, -1]], b = (7, 1)."],
     examples=[{"inputs": {"A": "2 3; 1 -1", "b": "7, 1"}, "outputs": {"solution": [2.0, 1.0], "det": -5.0}},
               {"inputs": {"A": [[3, -1], [1, 2]], "b": [5, 4]}, "outputs": {"solution": [2.0, 1.0]}}],
     questions=[{"q": "Solve the system 2x + 3y = 7, x - y = 1.", "outputs": {"solution": [2.0, 1.0]}},
                {"q": "Solve 3x - y = 5 and x + 2y = 4.", "outputs": {"solution": [2.0, 1.0]}},
                {"q": "Use Cramer's rule to solve A = [[2, 3], [1, -1]], b = (7, 1).", "outputs": {"solution": [2.0, 1.0]}},
                {"q": "Solve the system x + y = 2, 2x + 2y = 4.", "outcome": "INVALID_INPUT"}])

_add("solve_3x3_system",
     name="Three linear equations in three unknowns", topic="linear systems / small systems",
     purpose="Solve three linear equations in x, y, z by Cramer's rule.",
     inputs=_SYS_IN, outputs=[_O("solution", "the solution (x, y, z)"), _O("det", "determinant of the coefficient matrix")],
     primary="solution", conditions=["A is 3 x 3", "det A != 0 (otherwise there is no unique solution)"],
     method="Cramer's rule x_j = det(A_j)/det(A) in exact decimal arithmetic", latex=r"x_j = \frac{\det A_j}{\det A}",
     run=solve_3x3_system, verify=lambda i, o: max(_verify_solution(i, o), _verify_det_value(i, o)),
     reference=lambda A, b: {"solution": __import__("numpy").linalg.solve(__import__("numpy").asarray(A, float), b).tolist()},
     wrong=lambda A, b: (lambda r: {"solution": r["solution"][::-1], "det": r["det"]})(solve_3x3_system(A, b)),
     random_inputs=lambda rng: {"A": _rmat(rng, 3), "b": _rvec(rng, 3)},
     parse=_safe_parse(_parse_cramer(3)),
     templates=["Solve the system x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2.",
                "Use Cramer's rule to solve A = [[1, 1, 1], [0, 2, 5], [2, 5, -1]], b = (6, -4, 27)."],
     examples=[{"inputs": {"A": "1 1 1; 2 -1 1; 1 2 -1", "b": "6, 3, 2"}, "outputs": {"solution": [1.0, 2.0, 3.0]}},
               {"inputs": {"A": [[1, 1, 1], [0, 2, 5], [2, 5, -1]], "b": [6, -4, 27]}, "outputs": {"solution": [5.0, 3.0, -2.0]}}],
     questions=[{"q": "Solve the system x + y + z = 6, 2x - y + z = 3, x + 2y - z = 2.", "outputs": {"solution": [1.0, 2.0, 3.0]}},
                {"q": "Use Cramer's rule to solve A = [[1, 1, 1], [0, 2, 5], [2, 5, -1]], b = (6, -4, 27).",
                 "outputs": {"solution": [5.0, 3.0, -2.0]}},
                {"q": "Solve the system x + y + z = 1, x + y + z = 2, x - y = 0.", "outcome": "INVALID_INPUT"}])


def solve_linear_system(A, b):
    A = _need_mat(A, "A", square=True)
    b = _need_vec(b, "b", len(A))
    return {"solution": _fl(_solve_F(_FM(A), _FV(b)))}


def _parse_solve(q):
    if q.has(*_NOT_SOLVE) or q.has(r"\bcramer") or not q.has(r"\bsolve\b", r"\bsolution\b"):
        return None
    if len(q.mats) == 1 and len(q.vecs) == 1:
        return {"A": (_pick(q, "matrix", "A") or q.mats[0])["text"], "b": (_pick(q, "vector", "b") or q.vecs[0])["text"]}
    return None


_add("solve_linear_system",
     name="Solve a square linear system Ax = b", topic="linear systems / Gaussian elimination",
     purpose="The unique solution of Ax = b for a nonsingular n x n matrix A (n up to 10).",
     inputs=_SYS_IN, outputs=[_O("solution", "the solution vector x")], primary="solution",
     conditions=["A is square and nonsingular", "b has one entry per row of A", "at most 10 x 10"],
     method="Gaussian elimination with partial pivoting and back substitution, in exact rational arithmetic",
     latex=r"A\mathbf{x} = \mathbf{b}",
     run=solve_linear_system, verify=_verify_solution,
     reference=lambda A, b: {"solution": __import__("scipy.linalg", fromlist=["solve"]).solve(__import__("numpy").asarray(A, float), b).tolist()},
     wrong=lambda A, b: {"solution": solve_linear_system(_T(A), b)["solution"]},
     random_inputs=lambda rng: (lambda n: {"A": _rmat(rng, n), "b": _rvec(rng, n)})(rng.randint(2, 6)),
     parse=_safe_parse(_parse_solve),
     templates=["Solve Ax = b for A = [[2, 1], [1, 3]] and b = (3, 5).",
                "Solve the linear system with A = [[1, 2, 0, 0], [0, 1, 2, 0], [0, 0, 1, 2], [2, 0, 0, 1]] and b = [3, 3, 3, 3]."],
     examples=[{"inputs": {"A": "[[2, 1], [1, 3]]", "b": "3, 5"}, "outputs": {"solution": [0.8, 1.4]}},
               {"inputs": {"A": [[1, 2, 0, 0], [0, 1, 2, 0], [0, 0, 1, 2], [2, 0, 0, 1]], "b": [3, 3, 3, 3]},
                "outputs": {"solution": [1.0, 1.0, 1.0, 1.0]}}],
     questions=[{"q": "Solve Ax = b for A = [[2, 1], [1, 3]] and b = (3, 5).", "outputs": {"solution": [0.8, 1.4]}},
                {"q": "Solve Ax = b for A = [[1, 2], [2, 4]] and b = (3, 6).", "outcome": "INVALID_INPUT"},
                {"q": "Solve Ax = b for A = [[1, 2], [3, 4]] and b = (3, 6, 9).", "outcome": "INVALID_INPUT"}])


def _rank_F(M):
    return len(_rref_F(M)[1])


def matrix_rank(A):
    A = _need_mat(A, "A")
    return {"rank": float(_rank_F(_FM(A)))}


def _low_rank(rng, m, n, r):
    L = [[rng.randint(-3, 3) for _ in range(r)] for _ in range(m)]
    R = [[rng.randint(-3, 3) for _ in range(n)] for _ in range(r)]
    return [[sum(L[i][k] * R[k][j] for k in range(r)) for j in range(n)] for i in range(m)]


def _random_rank_case(rng):
    m, n = rng.randint(2, 5), rng.randint(2, 5)
    r = rng.randint(1, max(1, min(m, n) - 1)) if rng.random() < 0.85 else min(m, n)
    return _low_rank(rng, m, n, r)


def _sympy_matrix(A):
    import sympy as sp
    return sp.Matrix([[sp.Rational(repr(float(x))) for x in r] for r in A])


_add("matrix_rank",
     name="Rank of a matrix", topic="matrices / rank",
     purpose="The rank: the number of linearly independent rows (equivalently columns) of a matrix.",
     inputs=[_I("A", "matrix", "the matrix (up to 10 x 10)")], outputs=[_O("rank", "rank of A")], primary="rank",
     conditions=["at most 10 x 10"], method="count of pivots of the reduced row echelon form, exact rational arithmetic",
     latex=r"\operatorname{rank}A = \dim\operatorname{col}A", run=matrix_rank, verify=_recompute(matrix_rank, ["rank"]),
     reference=lambda A: {"rank": float(_sympy_matrix(A).rank())},
     wrong=lambda A: {"rank": float(min(len(A), len(A[0])))},
     random_inputs=lambda rng: {"A": _random_rank_case(rng)},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\brank\b")
                       and not q.has(r"\bnullity\b", r"\bnull ?space\b", r"\bkernel\b", r"\brow echelon\b", r"\brref\b", r"\blow-?rank\b",
                                     r"\brank-\d", r"\bapproxima(?:t|nt)", r"\bcolumn space\b", r"\btruncat", r"\bsvd\b",
                                     r"\bsingular values?\b", r"\bsigma", r"\breconstruct", r"\btarget rank\b") else None),
     templates=["Find the rank of [[1, 2, 3], [2, 4, 6], [1, 0, 1]].", "What is the rank of the matrix [1 2; 2 4]?"],
     examples=[{"inputs": {"A": "[[1, 2, 3], [2, 4, 6], [1, 0, 1]]"}, "outputs": {"rank": 2.0}},
               {"inputs": {"A": [[1, 2], [2, 4]]}, "outputs": {"rank": 1.0}},
               {"inputs": {"A": [[0, 0], [0, 0]]}, "outputs": {"rank": 0.0}}],
     questions=[{"q": "Find the rank of [[1, 2, 3], [2, 4, 6], [1, 0, 1]].", "outputs": {"rank": 2.0}},
                {"q": "What is the rank of the matrix [1 2; 2 4]?", "outputs": {"rank": 1.0}}])


def rref(A):
    A = _need_mat(A, "A")
    R, piv = _rref_F(_FM(A))
    return {"rref": _fl(R), "pivot_columns": [float(p + 1) for p in piv], "rank": float(len(piv))}


def _wrong_rref(A):
    """Row echelon form with unit pivots but without clearing the entries ABOVE each pivot."""
    M = _FM(A)
    rows, cols, r, piv = len(M), len(M[0]), 0, []
    for c in range(cols):
        if r == rows:
            break
        p = max(range(r, rows), key=lambda i: abs(M[i][c]))
        if M[p][c] == 0:
            continue
        M[r], M[p] = M[p], M[r]
        M[r] = [x / M[r][c] for x in M[r]]
        for i in range(r + 1, rows):
            M[i] = [a - M[i][c] * b for a, b in zip(M[i], M[r])]
        piv.append(c)
        r += 1
    return {"rref": _fl(M), "pivot_columns": [float(p + 1) for p in piv], "rank": float(len(piv))}


def _reference_rref(A):
    R, piv = _sympy_matrix(A).rref()
    return {"rref": [[float(x) for x in R.row(i)] for i in range(R.rows)], "pivot_columns": [float(p + 1) for p in piv],
            "rank": float(len(piv))}


_add("rref",
     name="Reduced row echelon form", topic="linear systems / row reduction",
     purpose="The reduced row echelon form (RREF) of a matrix, its pivot columns (1-based) and its rank.",
     inputs=[_I("A", "matrix", "the matrix (up to 10 x 10)")],
     outputs=[_O("rref", "the reduced row echelon form"), _O("pivot_columns", "pivot columns, 1-based"), _O("rank", "number of pivots")],
     primary="rref", conditions=["at most 10 x 10"],
     method="Gauss-Jordan elimination in exact rational arithmetic (the RREF is unique)", latex=r"A \sim \operatorname{rref}(A)",
     run=rref, verify=_recompute(rref, ["rref", "pivot_columns", "rank"]), reference=_reference_rref, wrong=_wrong_rref,
     random_inputs=lambda rng: {"A": _random_rank_case(rng) if rng.random() < 0.5 else _rmat(rng, rng.randint(2, 4), rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\breduced row echelon\b", r"\brref\b", r"\brow reduce\b", r"\brow-reduce\b", r"\bgauss-jordan\b") else None),
     templates=["Find the reduced row echelon form of [[1, 2, 3], [4, 5, 6], [7, 8, 9]].", "Row reduce the matrix [2 4 6; 1 3 5]."],
     examples=[{"inputs": {"A": "[[1, 2, 3], [4, 5, 6], [7, 8, 9]]"},
                "outputs": {"rref": [[1.0, 0.0, -1.0], [0.0, 1.0, 2.0], [0.0, 0.0, 0.0]], "pivot_columns": [1.0, 2.0], "rank": 2.0}},
               {"inputs": {"A": [[2, 4, 6], [1, 3, 5]]}, "outputs": {"rref": [[1.0, 0.0, -1.0], [0.0, 1.0, 2.0]]}}],
     questions=[{"q": "Find the reduced row echelon form of [[1, 2, 3], [4, 5, 6], [7, 8, 9]].",
                 "outputs": {"rref": [[1.0, 0.0, -1.0], [0.0, 1.0, 2.0], [0.0, 0.0, 0.0]]}},
                {"q": "Row reduce the matrix [2 4 6; 1 3 5].", "outputs": {"rref": [[1.0, 0.0, -1.0], [0.0, 1.0, 2.0]]}}])


def null_space(A):
    A = _need_mat(A, "A")
    R, piv = _rref_F(_FM(A))
    n = len(A[0])
    basis = []
    for f in (c for c in range(n) if c not in piv):
        v = [Fraction(0)] * n
        v[f] = Fraction(1)
        for row, p in enumerate(piv):
            v[p] = -R[row][f]
        basis.append(_fl(v))
    return {"nullity": float(len(basis)), "basis": basis}


def _verify_null(i, o):
    want = null_space(i["A"])
    return max(_diff(o["nullity"], want["nullity"]), _diff(o["basis"], want["basis"]))


def _reference_null(A):
    vs = _sympy_matrix(A).nullspace()
    return {"nullity": float(len(vs)), "basis": [[float(x) for x in v] for v in vs]}


_add("null_space",
     name="Null space (kernel) of a matrix", topic="vector spaces / fundamental subspaces",
     purpose="The dimension of the null space {x : Ax = 0} (the nullity) and a basis of it.",
     inputs=[_I("A", "matrix", "the matrix (up to 10 x 10)")],
     outputs=[_O("nullity", "dimension of the null space = columns - rank"),
              _O("basis", "basis vectors: one per free variable, that variable 1, the other free variables 0")],
     primary="nullity", conditions=["at most 10 x 10"],
     method="solve Ax = 0 from the exact reduced row echelon form; each free variable gives one basis vector",
     latex=r"\operatorname{null}A = \{\mathbf{x} : A\mathbf{x} = \mathbf{0}\},\quad \dim = n - \operatorname{rank}A",
     run=null_space, verify=_verify_null, reference=_reference_null,
     wrong=lambda A: dict(null_space(A), nullity=float(len(A) - _rank_F(_FM(A)))),
     random_inputs=lambda rng: (lambda m, n: {"A": _low_rank(rng, m, n, rng.randint(1, min(m, n)))})(*rng.choice([(2, 3), (2, 4), (3, 4), (3, 5), (4, 2), (3, 2), (4, 3), (2, 5)])),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\bnull ?space\b", r"\bnullity\b", r"\bkernel\b") else None),
     templates=["Find the null space of [[1, 2, 3], [2, 4, 6]].", "What is the nullity of the matrix [1 0 -1; 0 1 2]?",
                "Find a basis for the kernel of [[1, 1], [1, 1]]."],
     examples=[{"inputs": {"A": "[[1, 2, 3], [2, 4, 6]]"}, "outputs": {"nullity": 2.0, "basis": [[-2.0, 1.0, 0.0], [-3.0, 0.0, 1.0]]}},
               {"inputs": {"A": [[1, 0, -1], [0, 1, 2]]}, "outputs": {"nullity": 1.0, "basis": [[1.0, -2.0, 1.0]]}},
               {"inputs": {"A": [[1, 0], [0, 1]]}, "outputs": {"nullity": 0.0, "basis": []}}],
     questions=[{"q": "Find the null space of [[1, 2, 3], [2, 4, 6]].", "outputs": {"nullity": 2.0, "basis": [[-2.0, 1.0, 0.0], [-3.0, 0.0, 1.0]]}},
                {"q": "What is the nullity of the matrix [1 0 -1; 0 1 2]?", "outputs": {"nullity": 1.0}},
                {"q": "Find a basis for the kernel of [[1, 1], [1, 1]].", "outputs": {"basis": [[-1.0, 1.0]]}}])


def column_space(A):
    A = _need_mat(A, "A")
    _, piv = _rref_F(_FM(A))
    return {"dimension": float(len(piv)), "basis": [[float(A[r][c]) for r in range(len(A))] for c in piv]}


def _reference_colspace(A):
    vs = _sympy_matrix(A).columnspace()
    return {"dimension": float(len(vs)), "basis": [[float(x) for x in v] for v in vs]}


def _wrong_colspace(A):
    R, piv = _rref_F(_FM(A))
    return {"dimension": float(len(piv)), "basis": [[float(R[r][c]) for r in range(len(A))] for c in piv]}


_add("column_space",
     name="Column space of a matrix", topic="vector spaces / fundamental subspaces",
     purpose="A basis of the column space (range) of A: the pivot columns of A itself, and its dimension.",
     inputs=[_I("A", "matrix", "the matrix (up to 10 x 10)")],
     outputs=[_O("dimension", "dimension of the column space (the rank)"), _O("basis", "the pivot columns of A, left to right")],
     primary="dimension", conditions=["at most 10 x 10"],
     method="the pivot positions of the exact reduced row echelon form pick the columns of the ORIGINAL matrix",
     latex=r"\operatorname{col}A = \operatorname{span}\{\mathbf{a}_j : j \text{ a pivot column}\}",
     run=column_space, verify=_recompute(column_space, ["dimension", "basis"]), reference=_reference_colspace, wrong=_wrong_colspace,
     random_inputs=lambda rng: (lambda m, n: {"A": _low_rank(rng, m, n, rng.randint(1, min(m, n)))})(rng.randint(2, 4), rng.randint(2, 5)),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\bcolumn space\b", r"\brange of\b", r"\bimage of the matrix\b") and not q.has(r"\bprojection\b", r"\bprojector\b") else None),
     templates=["Find a basis for the column space of [[1, 2, 3], [2, 4, 7]].", "What is the column space of [1 2; 2 4]?"],
     examples=[{"inputs": {"A": "[[1, 2, 3], [2, 4, 7]]"}, "outputs": {"dimension": 2.0, "basis": [[1.0, 2.0], [3.0, 7.0]]}},
               {"inputs": {"A": [[1, 2], [2, 4]]}, "outputs": {"dimension": 1.0, "basis": [[1.0, 2.0]]}}],
     questions=[{"q": "Find a basis for the column space of [[1, 2, 3], [2, 4, 7]].", "outputs": {"basis": [[1.0, 2.0], [3.0, 7.0]]}},
                {"q": "What is the column space of [1 2; 2 4]?", "outputs": {"dimension": 1.0}}])


def linear_independence(vectors):
    V = _need_mat(vectors, "vectors")
    r = _rank_F(_FM(V))
    return {"independent": r == len(V), "rank": float(r)}


def _parse_independence(q):
    if not q.has(r"\blinearly (?:in)?dependent\b", r"\blinear (?:in)?dependence\b"):
        return None
    if q.mats and len(q.mats) == 1 and not q.vecs:
        return {"vectors": q.mats[0]["text"]}
    if len(q.vecs) >= 2 and not q.mats:
        return {"vectors": "; ".join(v["text"] for v in q.vecs)}
    return None


def _random_independence(rng):
    k, n = rng.randint(2, 4), rng.randint(2, 4)
    if k > n:
        k = n
    if rng.random() < 0.85:
        return {"vectors": _low_rank(rng, k, n, rng.randint(1, k - 1))}
    return {"vectors": [[rng.randint(-5, 5) for _ in range(n)] for _ in range(k)]}


_add("linear_independence",
     name="Linear independence of vectors", topic="vector spaces / independence and span",
     purpose="Whether a list of vectors is linearly independent, with the rank of the list.",
     inputs=[_I("vectors", "matrix", "the vectors, one per row (equal lengths)")],
     outputs=[_O("independent", "true when no vector is a combination of the others", "text"), _O("rank", "dimension of their span")],
     primary="independent", conditions=["all vectors have the same length", "at most 10 vectors of at most 10 entries"],
     method="the vectors are independent exactly when the rank of the matrix of rows equals their number (exact)",
     latex=r"c_1\mathbf{v}_1 + \cdots + c_k\mathbf{v}_k = \mathbf{0} \Rightarrow c_1 = \cdots = c_k = 0",
     run=linear_independence, verify=_recompute(linear_independence, ["independent", "rank"]),
     reference=lambda vectors: (lambda r: {"independent": r == len(vectors), "rank": float(r)})(_sympy_matrix(vectors).rank()),
     wrong=lambda vectors: {"independent": len(vectors) <= len(vectors[0]), "rank": float(_rank_F(_FM(vectors)))},
     random_inputs=_random_independence, parse=_safe_parse(_parse_independence),
     templates=["Are the vectors (1, 2, 3), (4, 5, 6) and (7, 8, 9) linearly independent?",
                "Are the vectors [1, 0] and [1, 1] linearly independent?"],
     examples=[{"inputs": {"vectors": "1 2 3; 4 5 6; 7 8 9"}, "outputs": {"independent": False, "rank": 2.0}},
               {"inputs": {"vectors": [[1, 0], [1, 1]]}, "outputs": {"independent": True, "rank": 2.0}}],
     questions=[{"q": "Are the vectors (1, 2, 3), (4, 5, 6) and (7, 8, 9) linearly independent?", "outputs": {"independent": False}},
                {"q": "Are the vectors [1, 0] and [1, 1] linearly independent?", "outputs": {"independent": True}},
                {"q": "Are the vectors (1, 2) and (1, 2, 3) linearly independent?", "outcome": "INVALID_INPUT"}])


def linear_system_classification(A, b):
    A = _need_mat(A, "A")
    b = _need_vec(b, "b", len(A))
    FA = _FM(A)
    ra = _rank_F(FA)
    rb = _rank_F([r + [x] for r, x in zip(FA, _FV(b))])
    kind = "none" if rb > ra else ("unique" if ra == len(A[0]) else "infinitely many")
    return {"solutions": kind, "rank_A": float(ra), "rank_augmented": float(rb)}


def _reference_classify(A, b):
    import sympy as sp
    M = _sympy_matrix(A)
    aug = M.row_join(sp.Matrix([sp.Rational(repr(float(x))) for x in b]))
    ra, rb = M.rank(), aug.rank()
    return {"solutions": "none" if rb > ra else ("unique" if ra == M.cols else "infinitely many"),
            "rank_A": float(ra), "rank_augmented": float(rb)}


def _random_classify(rng):
    u = rng.random()
    if u < 0.2:
        n = rng.randint(2, 4)
        return {"A": _rmat(rng, n), "b": _rvec(rng, n)}
    if u < 0.6:                                          # fewer equations than unknowns, consistent
        m, n = rng.randint(1, 3), rng.randint(2, 5)
        n = max(n, m + 1)
        A = [[rng.randint(-5, 5) for _ in range(n)] for _ in range(m)]
        x = [rng.randint(-3, 3) for _ in range(n)]
        return {"A": A, "b": [sum(a * c for a, c in zip(r, x)) for r in A]}
    m, n = rng.randint(2, 4), rng.randint(2, 4)          # singular / rank-deficient, a random right side
    return {"A": _low_rank(rng, m, n, rng.randint(1, min(m, n) - 1) if min(m, n) > 1 else 1),
            "b": [rng.randint(-9, 9) for _ in range(m)]}


def _parse_classify(q):
    if not q.has(r"\bhow many solutions\b", r"\bconsistent\b", r"\binconsistent\b", r"\bclassify\b", r"\bnumber of solutions\b"):
        return None
    if q.objs:
        if len(q.mats) == 1 and len(q.vecs) == 1:
            return {"A": (_pick(q, "matrix", "A") or q.mats[0])["text"], "b": (_pick(q, "vector", "b") or q.vecs[0])["text"]}
        return None
    e = _equations(q)
    if e is None:
        return None
    return {"A": _mat_text(e[0]), "b": _vec_text(e[1])}


_add("linear_system_classification",
     name="Number of solutions of a linear system", topic="linear systems / existence and uniqueness",
     purpose="Whether Ax = b has a unique solution, infinitely many, or none (Rouche-Capelli), with the two ranks.",
     inputs=_SYS_IN,
     outputs=[_O("solutions", "unique / infinitely many / none", "text"), _O("rank_A", "rank of A"),
              _O("rank_augmented", "rank of the augmented matrix [A | b]")],
     primary="solutions", conditions=["b has one entry per row of A", "at most 10 x 10"],
     method="compare rank A, rank [A | b] and the number of unknowns (exact row reduction)",
     latex=r"\operatorname{rank}A < \operatorname{rank}[A\,|\,\mathbf{b}] \Rightarrow \text{none};\ \operatorname{rank}A = \operatorname{rank}[A\,|\,\mathbf{b}] = n \Rightarrow \text{unique}",
     run=linear_system_classification, verify=_recompute(linear_system_classification, ["solutions", "rank_A", "rank_augmented"]),
     reference=_reference_classify,
     wrong=lambda A, b: (lambda r: dict(r, solutions="unique" if r["rank_A"] == len(A) else "infinitely many"))(linear_system_classification(A, b)),
     random_inputs=_random_classify, parse=_safe_parse(_parse_classify),
     templates=["How many solutions does the system x + y = 2, 2x + 2y = 5 have?", "Is the system x + y + z = 1, x - y = 0 consistent?",
                "How many solutions does Ax = b have for A = [[1, 2], [2, 4]] and b = (3, 6)?"],
     examples=[{"inputs": {"A": "1 1; 2 2", "b": "2, 5"}, "outputs": {"solutions": "none", "rank_A": 1.0, "rank_augmented": 2.0}},
               {"inputs": {"A": [[1, 1, 1], [1, -1, 0]], "b": [1, 0]}, "outputs": {"solutions": "infinitely many"}},
               {"inputs": {"A": [[2, 1], [1, 3]], "b": [3, 5]}, "outputs": {"solutions": "unique"}}],
     questions=[{"q": "How many solutions does the system x + y = 2, 2x + 2y = 5 have?", "outputs": {"solutions": "none"}},
                {"q": "Is the system x + y + z = 1, x - y = 0 consistent?", "outputs": {"solutions": "infinitely many"}},
                {"q": "How many solutions does Ax = b have for A = [[1, 2], [2, 4]] and b = (3, 6)?", "outputs": {"solutions": "infinitely many"}}])


# --- minors, cofactors, adjugate, characteristic polynomial, 2 x 2 eigenvalues ---------------------------------
def _minor_F(FA, i, j):
    return _det_F([[x for c, x in enumerate(r) if c != j] for k, r in enumerate(FA) if k != i])


def minor_cofactor(A, i, j):
    A = _need_mat(A, "A", square=True)
    n = len(A)
    if len(A) < 2:
        raise ValueError("a minor needs a matrix of at least 2 x 2")
    for name, v in (("i", i), ("j", j)):
        if isinstance(v, bool) or v != int(v) or not 1 <= v <= n:
            raise ValueError(f"{name} must be a whole number from 1 to {n}")
    m = _minor_F(_FM(A), int(i) - 1, int(j) - 1)
    return {"minor": float(m), "cofactor": float(m if (int(i) + int(j)) % 2 == 0 else -m)}


def _parse_minor(q):
    if len(q.mats) != 1 or q.vecs or not q.has(r"\bminor\b", r"\bcofactor\b") or q.has(r"\bleading\b", r"\bprincipal\b", r"\bmatrix of cofactors\b",
                                                                                         r"\bcofactor matrix\b", r"\badjugate\b"):
        return None
    m = re.search(r"\b[mc]_?\{?\s*(\d)\s*,?\s*(\d)\s*\}?(?!\d|\.\d)", q.rest) or \
        re.search(r"\brow\s+(\d+)\s*,?\s*(?:and\s+)?column\s+(\d+)", q.rest) or re.search(r"\bentry\s*a_?\{?(\d)\s*,?\s*(\d)\}?", q.rest)
    if not m:
        return None
    return {"A": q.mats[0]["text"], "i": float(m.group(1)), "j": float(m.group(2))}


_add("minor_cofactor",
     name="Minor and cofactor of an entry", topic="determinants / cofactor expansion",
     purpose="The minor M_ij (determinant with row i and column j deleted) and the cofactor C_ij = (-1)^(i+j) M_ij.",
     inputs=[_I("A", "matrix", "a square matrix"), _I("i", "integer", "row of the entry (1-based)", min=1),
             _I("j", "integer", "column of the entry (1-based)", min=1)],
     outputs=[_O("minor", "the minor M_ij"), _O("cofactor", "the cofactor C_ij")], primary="cofactor",
     conditions=["A is square, at least 2 x 2", "1 <= i, j <= n"],
     method="delete row i and column j, take the determinant exactly; attach the sign (-1)^(i+j)",
     latex=r"C_{ij} = (-1)^{i+j} M_{ij}",
     run=minor_cofactor, verify=_recompute(minor_cofactor, ["minor", "cofactor"]),
     reference=lambda A, i, j: (lambda M: {"minor": float(M.minor(int(i) - 1, int(j) - 1)), "cofactor": float(M.cofactor(int(i) - 1, int(j) - 1))})(_sympy_matrix(A)),
     wrong=lambda A, i, j: minor_cofactor(_T(A), i, j) if True else None,
     random_inputs=lambda rng: (lambda n: {"A": _rmat(rng, n), "i": rng.randint(1, n), "j": rng.randint(1, n)})(rng.randint(3, 4)),
     parse=_safe_parse(_parse_minor),
     templates=["Find the cofactor C23 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]].",
                "Find the minor of the entry in row 1, column 2 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]]."],
     examples=[{"inputs": {"A": "[[1, 2, 3], [4, 5, 6], [7, 8, 10]]", "i": 2, "j": 3}, "outputs": {"minor": -6.0, "cofactor": 6.0}},
               {"inputs": {"A": [[1, 2, 3], [4, 5, 6], [7, 8, 10]], "i": 1, "j": 2}, "outputs": {"minor": -2.0, "cofactor": 2.0}}],
     questions=[{"q": "Find the cofactor C23 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]].", "outputs": {"cofactor": 6.0}},
                {"q": "Find the minor of the entry in row 1, column 2 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]].", "outputs": {"minor": -2.0}},
                {"q": "Find the cofactor C35 of [[1, 2, 3], [4, 5, 6], [7, 8, 10]].", "outcome": "INVALID_INPUT"}])


def adjugate(A):
    A = _need_mat(A, "A", square=True, max_dim=7)
    F, n = _FM(A), len(A)
    if n == 1:
        return {"adjugate": [[1.0]]}
    return {"adjugate": [[float((-1) ** (i + j) * _minor_F(F, j, i)) for j in range(n)] for i in range(n)]}


def _verify_adjugate(i, o):
    """A adj(A) = det(A) I (exact check on the floats when A is nonsingular), else a fresh recomputation."""
    want = adjugate(i["A"])["adjugate"]
    return _diff(o["adjugate"], want)


_add("adjugate",
     name="Adjugate (classical adjoint) of a matrix", topic="determinants / cofactor expansion",
     purpose="adj(A): the transpose of the matrix of cofactors; A adj(A) = det(A) I.",
     inputs=[_I("A", "matrix", "a square matrix (up to 7 x 7)")], outputs=[_O("adjugate", "the adjugate matrix adj(A)")],
     primary="adjugate", conditions=["A is square", "at most 7 x 7"],
     method="entry (i, j) = cofactor C_ji, every minor computed exactly", latex=r"\operatorname{adj}(A)_{ij} = C_{ji}",
     run=adjugate, verify=_verify_adjugate,
     reference=lambda A: {"adjugate": [[float(x) for x in r] for r in _sympy_matrix(A).adjugate().tolist()]},
     wrong=lambda A: {"adjugate": _T(adjugate(A)["adjugate"])},
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\badjugate\b", r"\badjoint\b", r"\bcofactor matrix\b", r"\bmatrix of cofactors\b")
                       and not q.has(r"\bself-adjoint\b") else None),
     templates=["Find the adjugate of [[1, 2], [3, 4]].", "Compute the adjugate of the matrix [1 2 3; 0 1 4; 5 6 0]."],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]"}, "outputs": {"adjugate": [[4.0, -2.0], [-3.0, 1.0]]}},
               {"inputs": {"A": [[1, 2, 3], [0, 1, 4], [5, 6, 0]]}, "outputs": {"adjugate": [[-24.0, 18.0, 5.0], [20.0, -15.0, -4.0], [-5.0, 4.0, 1.0]]}}],
     questions=[{"q": "Find the adjugate of [[1, 2], [3, 4]].", "outputs": {"adjugate": [[4.0, -2.0], [-3.0, 1.0]]}}])


def characteristic_polynomial(A):
    A = _need_mat(A, "A", square=True)
    F, n = _FM(A), len(A)
    M = [[Fraction(0)] * n for _ in range(n)]
    c = [Fraction(1)]
    I = [[Fraction(int(i == j)) for j in range(n)] for i in range(n)]
    for k in range(1, n + 1):                         # Faddeev-LeVerrier: M_k = A M_{k-1} + c_{k-1} I, c_k = -tr(A M_k)/k
        M = [[a + c[-1] * b for a, b in zip(r, s)] for r, s in zip(_mm_F(F, M), I)] if k > 1 else [r[:] for r in I]
        AM = _mm_F(F, M)
        c.append(-sum((AM[i][i] for i in range(n)), Fraction(0)) / k)
    return {"coefficients": [float(x) for x in c]}


def _verify_charpoly(i, o):
    """Newton's identities with the exact power traces p_k = tr(A^k): k c_k = -(p_k + c_1 p_{k-1} + ... + c_{k-1} p_1)."""
    F, n = _FM(i["A"]), len(i["A"])
    co = o["coefficients"]
    if len(co) != n + 1:
        return BAD
    P, p = [r[:] for r in F], []
    for _ in range(n):
        p.append(sum((P[j][j] for j in range(n)), Fraction(0)))
        P = _mm_F(P, F)
    c = [Fraction(1)]
    for k in range(1, n + 1):
        c.append(-(p[k - 1] + sum((c[m] * p[k - 1 - m] for m in range(1, k)), Fraction(0))) / k)
    return _diff(co, [float(x) for x in c])


def _reference_charpoly(A):
    import sympy as sp
    lam = sp.Symbol("lambda")
    return {"coefficients": [float(x) for x in _sympy_matrix(A).charpoly(lam).all_coeffs()]}


_add("characteristic_polynomial",
     name="Characteristic polynomial of a matrix", topic="eigenvalues / characteristic polynomial",
     purpose="The coefficients of det(lambda I - A) = lambda^n + c1 lambda^(n-1) + ... + cn, highest power first.",
     inputs=[_I("A", "matrix", "a square matrix (up to 10 x 10)")],
     outputs=[_O("coefficients", "[1, c1, ..., cn] of det(lambda I - A), highest power first")], primary="coefficients",
     conditions=["A is square", "at most 10 x 10"],
     method="Faddeev-LeVerrier recursion in exact rational arithmetic", latex=r"p(\lambda) = \det(\lambda I - A)",
     run=characteristic_polynomial, verify=_verify_charpoly, reference=_reference_charpoly,
     wrong=lambda A: (lambda c: {"coefficients": [c[0], -c[1]] + c[2:]})(characteristic_polynomial(A)["coefficients"]),
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\bcharacteristic (?:polynomial|equation)\b") else None),
     templates=["Find the characteristic polynomial of [[2, 1], [1, 2]].", "What is the characteristic polynomial of [1 2 0; 0 3 0; 2 -4 2]?"],
     examples=[{"inputs": {"A": "[[2, 1], [1, 2]]"}, "outputs": {"coefficients": [1.0, -4.0, 3.0]}},
               {"inputs": {"A": [[1, 2, 0], [0, 3, 0], [2, -4, 2]]}, "outputs": {"coefficients": [1.0, -6.0, 11.0, -6.0]}}],
     questions=[{"q": "Find the characteristic polynomial of [[2, 1], [1, 2]].", "outputs": {"coefficients": [1.0, -4.0, 3.0]}},
                {"q": "Find the characteristic polynomial of [[1, 2, 3], [4, 5, 6]].", "outcome": "INVALID_INPUT"}])


def _canon_eigs(vals, scale=1.0):
    """Real eigenvalues ascending, then complex ones by real part with the positive imaginary part first."""
    tol = 1e-12 * max(1.0, scale)
    real = sorted(float(z.real) for z in vals if abs(complex(z).imag) <= tol)
    cplx = sorted((complex(z) for z in vals if abs(complex(z).imag) > tol), key=lambda z: (z.real, -z.imag))
    return real + cplx


def eigenvalues_2x2(A):
    A = _need_mat(A, "A", square=True, rows=2)
    F = _FM(A)
    tr, det = F[0][0] + F[1][1], F[0][0] * F[1][1] - F[0][1] * F[1][0]
    disc = tr * tr - 4 * det
    if disc >= 0:
        s = math.sqrt(float(disc))
        if disc == 0:
            l1 = l2 = float(tr / 2)
        else:
            qq = -(-float(tr) + math.copysign(s, -float(tr))) / 2          # stable: no cancellation
            l1, l2 = (qq, float(det) / qq) if qq != 0 else (s / 2, -s / 2)
        lo, hi = sorted([l1, l2])
        return {"lambda1": lo, "lambda2": hi}
    re_, im = float(tr) / 2 + 0.0, math.sqrt(float(-disc)) / 2
    return {"lambda1": complex(re_, im), "lambda2": complex(re_, -im)}


def _verify_eig2(i, o):
    """lambda1 + lambda2 = trace and lambda1 lambda2 = det, relative to the size of the eigenvalues; canonical order."""
    A = i["A"]
    l1, l2 = complex(o["lambda1"]), complex(o["lambda2"])
    tr, det = A[0][0] + A[1][1], A[0][0] * A[1][1] - A[0][1] * A[1][0]
    s = max(1.0, abs(l1) + abs(l2), _fro(A))
    order = 0.0
    if abs(l1.imag) > 0 or abs(l2.imag) > 0:
        order = 0.0 if l1.imag > 0 and abs(l1 - l2.conjugate()) <= 1e-12 * s else BAD
    elif l1.real > l2.real:
        order = BAD
    return max(abs(l1 + l2 - tr) / s, abs(l1 * l2 - det) / (s * s), order)


def _random_eig2(rng):
    while True:
        A = _rmat(rng, 2)
        F = _FM(A)
        disc = (F[0][0] - F[1][1]) ** 2 + 4 * F[0][1] * F[1][0]
        if disc != 0 or (A[0][1] == 0 and A[1][0] == 0):          # no defective (non-diagonalizable) cases
            return {"A": A}


def _reference_eig2(A):
    import numpy as np
    v = _canon_eigs(list(np.linalg.eigvals(np.asarray(A, float))), float(np.abs(A).max()))
    return {"lambda1": v[0], "lambda2": v[1]}


_add("eigenvalues_2x2",
     name="Eigenvalues of a 2 x 2 matrix", topic="eigenvalues / small matrices",
     purpose="Both eigenvalues of a 2 x 2 matrix from its characteristic equation lambda^2 - tr(A) lambda + det(A) = 0.",
     inputs=[_I("A", "matrix", "a 2 x 2 matrix")],
     outputs=[_O("lambda1", "first eigenvalue (the smaller real one, or the one with positive imaginary part)"),
              _O("lambda2", "second eigenvalue")], primary="lambda1",
     conditions=["A is 2 x 2"], method="quadratic formula on the characteristic polynomial (trace and determinant exact, stable form)",
     latex=r"\lambda = \frac{\operatorname{tr}A \pm \sqrt{(\operatorname{tr}A)^2 - 4\det A}}{2}",
     run=eigenvalues_2x2, verify=_verify_eig2, reference=_reference_eig2,
     wrong=lambda A: (lambda tr, det: (lambda d: {"lambda1": min(tr / 2 - d / 2, tr / 2 + d / 2), "lambda2": max(tr / 2 - d / 2, tr / 2 + d / 2)})(
         math.sqrt(tr * tr + 4 * det)) if tr * tr + 4 * det >= 0 else
         {"lambda1": complex(tr / 2, math.sqrt(-(tr * tr + 4 * det)) / 2), "lambda2": complex(tr / 2, -math.sqrt(-(tr * tr + 4 * det)) / 2)})(
         A[0][0] + A[1][1], A[0][0] * A[1][1] - A[0][1] * A[1][0]),
     random_inputs=_random_eig2,
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and _is_square(q.mats[0], 2)
                       and q.has(r"\beigenvalues?\b") and not q.has(r"\beigenvectors?\b", r"\bdominant\b", r"\blargest\b", r"\bsmallest\b",
                                                                   r"\bpower\b", r"\biteration\b", r"\bspectral radius\b", r"\bsingular\b",
                                                                   r"\bcharacteristic polynomial\b", *_ASKS_MODULUS, *_LIST_ORDER,
                                                                   *_LIST_DESC) else None),
     templates=["Find the eigenvalues of [[2, 1], [1, 2]].", "What are the eigenvalues of the matrix [0 -1; 1 0]?"],
     examples=[{"inputs": {"A": "[[2, 1], [1, 2]]"}, "outputs": {"lambda1": 1.0, "lambda2": 3.0}},
               {"inputs": {"A": [[0, -1], [1, 0]]}, "outputs": {"lambda1": complex(0, 1), "lambda2": complex(0, -1)}},
               {"inputs": {"A": [[4, 1], [2, 3]]}, "outputs": {"lambda1": 2.0, "lambda2": 5.0}}],
     questions=[{"q": "Find the eigenvalues of [[2, 1], [1, 2]].", "outputs": {"lambda1": 1.0, "lambda2": 3.0}},
                {"q": "What are the eigenvalues of the matrix [0 -1; 1 0]?", "outputs": {"lambda1": complex(0, 1)}}])


def change_of_basis(basis, v):
    B = _need_mat(basis, "basis", square=True)
    v = _need_vec(v, "v", len(B))
    try:
        c = _solve_F(_T(_FM(B)), _FV(v))
    except ValueError:
        raise ValueError("the basis vectors are linearly dependent, so they are not a basis") from None
    return {"coordinates": _fl(c)}


def _parse_change_basis(q):
    if not q.has(r"\bcoordinates?\b") or not q.has(r"\bbasis\b") or q.mats or len(q.vecs) < 3:
        return None
    v, rest = q.vecs[0], q.vecs[1:]
    if len(rest) != len(v["value"] or []):
        return None
    return {"basis": "; ".join(o["text"] for o in rest), "v": v["text"]}


_add("change_of_basis",
     name="Coordinates of a vector in a basis", topic="vector spaces / bases and coordinates",
     purpose="The coordinates c of v with respect to a basis b1, ..., bn: v = c1 b1 + ... + cn bn.",
     inputs=[_I("basis", "matrix", "the basis vectors, one per row"), _I("v", "vector", "the vector")],
     outputs=[_O("coordinates", "the coordinate vector [v]_B")], primary="coordinates",
     conditions=["n basis vectors of length n that are linearly independent", "v has n entries"],
     method="solve B c = v with the basis vectors as the columns of B (exact)", latex=r"\mathbf{v} = \sum_i c_i \mathbf{b}_i",
     run=change_of_basis,
     verify=lambda i, o: (lambda B, c, v: max(abs(a - b) for a, b in zip(_mv(_T(B), c), v)) / max(1.0, _fro(B) * _norm(c) + _norm(v))
                          if len(c) == len(v) else BAD)(i["basis"], o["coordinates"], i["v"]),
     reference=lambda basis, v: {"coordinates": __import__("numpy").linalg.solve(__import__("numpy").asarray(basis, float).T, v).tolist()},
     wrong=lambda basis, v: {"coordinates": _fl(_solve_F(_FM(basis), _FV(v)))},
     random_inputs=lambda rng: (lambda n: {"basis": _rmat(rng, n), "v": _rvec(rng, n)})(rng.randint(2, 4)),
     parse=_safe_parse(_parse_change_basis),
     templates=["Find the coordinates of (3, 1) in the basis (1, 1), (1, -1).",
                "What are the coordinates of the vector [2, 3, 4] with respect to the basis [1, 0, 0], [1, 1, 0], [1, 1, 1]?"],
     examples=[{"inputs": {"basis": "1 1; 1 -1", "v": "3, 1"}, "outputs": {"coordinates": [2.0, 1.0]}},
               {"inputs": {"basis": [[1, 0, 0], [1, 1, 0], [1, 1, 1]], "v": [2, 3, 4]}, "outputs": {"coordinates": [-1.0, -1.0, 4.0]}}],
     questions=[{"q": "Find the coordinates of (3, 1) in the basis (1, 1), (1, -1).", "outputs": {"coordinates": [2.0, 1.0]}},
                {"q": "Find the coordinates of (3, 1) in the basis (1, 2), (2, 4).", "outcome": "INVALID_INPUT"}])


# --- least squares ---------------------------------------------------------------------------------------------
def _points(q):
    """Data as x = (...), y = (...) or as a list of points (x1, y1), (x2, y2), ..."""
    xo, yo = _pick(q, "vector", "x"), _pick(q, "vector", "y")
    if xo is not None and yo is not None and xo is not yo and xo["name"] == "x" and yo["name"] == "y":
        return {"x": xo["text"], "y": yo["text"]}
    if len(q.vecs) >= 2 and all(v["value"] and len(v["value"]) == 2 for v in q.vecs) and not q.mats \
            and q.has(r"\bpoints?\b", r"\bdata\b"):
        return {"x": ", ".join(repr(v["value"][0]) for v in q.vecs), "y": ", ".join(repr(v["value"][1]) for v in q.vecs)}
    return None


def least_squares_line(x, y):
    x, y = _need_vec(x, "x", min_n=2, max_n=1000), _need_vec(y, "y", min_n=2, max_n=1000)
    if len(x) != len(y):
        raise ValueError("x and y must have the same number of values")
    X, Y = _FV(x), _FV(y)
    n = len(X)
    mx, my = sum(X, Fraction(0)) / n, sum(Y, Fraction(0)) / n
    sxx = sum(((a - mx) ** 2 for a in X), Fraction(0))
    syy = sum(((b - my) ** 2 for b in Y), Fraction(0))
    sxy = sum(((a - mx) * (b - my) for a, b in zip(X, Y)), Fraction(0))
    if sxx == 0:
        raise ValueError("all x values are equal: the line is vertical and has no slope")
    if syy == 0:
        raise ValueError("all y values are equal: the fitted line is y = constant and R^2 is undefined")
    slope = sxy / sxx
    return {"slope": float(slope), "intercept": float(my - slope * mx), "r_squared": float(sxy * sxy / (sxx * syy))}


def _verify_line(i, o):
    """Normal equations: sum of residuals = 0 and sum of x * residual = 0; R^2 = 1 - SS_res / SS_tot."""
    x, y = i["x"], i["y"]
    r = [b - o["intercept"] - o["slope"] * a for a, b in zip(x, y)]
    sc = max(1.0, math.fsum(abs(b) for b in y) + math.fsum(abs(o["slope"] * a) for a in x) + len(x) * abs(o["intercept"]))
    sx = max(1.0, max(abs(a) for a in x))
    my = math.fsum(y) / len(y)
    sst = math.fsum((b - my) ** 2 for b in y)
    r2 = 1 - math.fsum(e * e for e in r) / sst
    return max(abs(math.fsum(r)) / sc, abs(math.fsum(a * e for a, e in zip(x, r))) / (sc * sx), abs(o["r_squared"] - r2))


def _reference_line(x, y):
    from scipy import stats
    res = stats.linregress(x, y)
    return {"slope": float(res.slope), "intercept": float(res.intercept), "r_squared": float(res.rvalue) ** 2}


def _random_line(rng):
    n = rng.randint(3, 10)
    x = sorted(rng.sample(range(-10, 15), n)) if rng.random() < 0.6 else [round(rng.uniform(0, 20), 2) for _ in range(n)]
    a, b = round(rng.uniform(-5, 5), 2), round(rng.uniform(-3, 3), 2)
    return {"x": x, "y": [round(a + b * t + rng.uniform(-2, 2), 2) for t in x]}


_add("least_squares_line",
     name="Least-squares regression line", topic="least squares / line fitting",
     purpose="The straight line y = a + b x that best fits data points in the least-squares sense, with R^2.",
     inputs=[_I("x", "vector", "x values"), _I("y", "vector", "y values")],
     outputs=[_O("slope", "slope b"), _O("intercept", "intercept a"), _O("r_squared", "coefficient of determination R^2")],
     primary="slope", conditions=["x and y have the same number of values, at least 2", "the x values are not all equal",
                                  "the y values are not all equal (otherwise R^2 is undefined)"],
     method="b = Sxy / Sxx, a = mean(y) - b mean(x), R^2 = Sxy^2 / (Sxx Syy), exact sums",
     latex=r"b = \frac{\sum (x_i-\bar x)(y_i-\bar y)}{\sum (x_i-\bar x)^2},\quad a = \bar y - b\bar x",
     run=least_squares_line, verify=_verify_line, reference=_reference_line,
     wrong=lambda x, y: (lambda b: {"slope": b, "intercept": 0.0, "r_squared": least_squares_line(x, y)["r_squared"]})(
         math.fsum(a * c for a, c in zip(x, y)) / math.fsum(a * a for a in x)),
     random_inputs=_random_line,
     parse=_safe_parse(lambda q: _points(q) if q.has(r"\bline\b", r"\blinear regression\b", r"\bregression line\b")
                       and q.has(r"\bfit\b", r"\bleast[- ]squares\b", r"\bregression\b", r"\bbest\b") and not q.has(r"\binterpolat", r"\bpolynomial\b", r"\bquadratic\b") else None),
     templates=["Find the least squares line for the points (1, 2), (2, 3), (3, 5), (4, 4).",
                "Fit a line to the data x = (0, 1, 2, 3), y = (1, 3, 5, 7) by least squares."],
     examples=[{"inputs": {"x": "1, 2, 3, 4", "y": "2, 3, 5, 4"}, "outputs": {"slope": 0.8, "intercept": 1.5, "r_squared": 0.64}},
               {"inputs": {"x": [0, 1, 2, 3], "y": [1, 3, 5, 7]}, "outputs": {"slope": 2.0, "intercept": 1.0, "r_squared": 1.0}}],
     questions=[{"q": "Find the least squares line for the points (1, 2), (2, 3), (3, 5), (4, 4).", "outputs": {"slope": 0.8, "intercept": 1.5}},
                {"q": "Fit a line to the data x = (0, 1, 2, 3), y = (1, 3, 5, 7) by least squares.", "outputs": {"slope": 2.0, "intercept": 1.0}},
                {"q": "Find the least squares line for the points (1, 2), (1, 3), (1, 5).", "outcome": "INVALID_INPUT"}])


def _normal_solve(A, b):
    """Exact least-squares solution of A x ~ b by the normal equations A^T A x = A^T b."""
    At = _T(A)
    try:
        return _solve_F(_mm_F(At, A), [sum((a * c for a, c in zip(r, b)), Fraction(0)) for r in At])
    except ValueError:
        raise ValueError("the columns are linearly dependent: the least-squares solution is not unique") from None


def least_squares_polynomial(x, y, degree):
    x, y = _need_vec(x, "x", min_n=2, max_n=200), _need_vec(y, "y", min_n=2, max_n=200)
    if len(x) != len(y):
        raise ValueError("x and y must have the same number of values")
    if isinstance(degree, bool) or degree != int(degree) or not 0 <= degree <= 8:
        raise ValueError("the degree must be a whole number from 0 to 8")
    d = int(degree)
    X = _FV(x)
    if len(set(X)) <= d:
        raise ValueError(f"a degree-{d} fit needs at least {d + 1} distinct x values")
    V = [[t ** k for k in range(d + 1)] for t in X]
    return {"coefficients": _fl(_normal_solve(V, _FV(y)))}


def _verify_polyfit(i, o):
    """Normal equations V^T (y - V c) = 0, relative to |V| (|y| + |V||c|)."""
    x, y, c = i["x"], i["y"], o["coefficients"]
    d = int(i["degree"])
    if len(c) != d + 1:
        return BAD
    V = [[t ** k for k in range(d + 1)] for t in x]
    r = [b - math.fsum(v * cc for v, cc in zip(row, c)) for row, b in zip(V, y)]
    g = [math.fsum(row[k] * e for row, e in zip(V, r)) for k in range(d + 1)]
    nv = _fro(V)
    return _norm(g) / max(1e-300, nv * (_norm(y) + nv * _norm(c)))


def _parse_degree(q):
    m = re.search(r"\bdegree\s*(?:=\s*)?(\d+)", q.rest) or re.search(r"\b(\d+)(?:st|nd|rd|th)[- ](?:degree|order)\b", q.rest)
    if m:
        return float(m.group(1))
    for w, d in (("quadratic", 2), ("parabola", 2), ("cubic", 3), ("quartic", 4)):
        if w in q.text:
            return float(d)
    return None


_add("least_squares_polynomial",
     name="Least-squares polynomial fit", topic="least squares / polynomial fitting",
     purpose="The polynomial c0 + c1 x + ... + cd x^d of a given degree that best fits data in the least-squares sense.",
     inputs=[_I("x", "vector", "x values"), _I("y", "vector", "y values"), _I("degree", "integer", "degree d of the polynomial (0 to 8)", min=0, max=8)],
     outputs=[_O("coefficients", "[c0, c1, ..., cd], lowest power first")], primary="coefficients",
     conditions=["x and y have the same number of values", "more distinct x values than the degree", "degree at most 8"],
     method="normal equations V^T V c = V^T y of the Vandermonde matrix, solved in exact rational arithmetic",
     latex=r"\min_{\mathbf{c}} \sum_i \Big(y_i - \sum_{k=0}^{d} c_k x_i^k\Big)^2",
     run=least_squares_polynomial, verify=_verify_polyfit,
     reference=lambda x, y, degree: {"coefficients": __import__("numpy").polynomial.polynomial.polyfit(x, y, int(degree)).tolist()},
     wrong=lambda x, y, degree: {"coefficients": least_squares_polynomial(x, y, degree)["coefficients"][::-1]},
     random_inputs=lambda rng: (lambda d: (lambda x: {"x": x, "y": [round(rng.uniform(-10, 10), 2) for _ in x], "degree": d})(
         sorted(rng.sample(range(-6, 7), rng.randint(d + 2, min(9, d + 5))))))(rng.randint(1, 3)),
     parse=_safe_parse(lambda q: (lambda p, d: dict(p, degree=d) if p and d is not None else None)(_points(q), _parse_degree(q))
                       if q.has(r"\bpolynomial\b", r"\bquadratic\b", r"\bcubic\b", r"\bparabola\b", r"\bquartic\b")
                       and q.has(r"\bfit\b", r"\bleast[- ]squares\b", r"\bregression\b", r"\bbest\b") and not q.has(r"\binterpolat") else None),
     templates=["Fit a quadratic to the points (0, 1), (1, 0), (2, 3), (3, 10) by least squares.",
                "Find the least squares polynomial of degree 2 for x = (-1, 0, 1, 2), y = (2, 1, 2, 5)."],
     examples=[{"inputs": {"x": "0, 1, 2, 3", "y": "1, 0, 3, 10", "degree": 2}, "outputs": {"coefficients": [1.0, -3.0, 2.0]}},
               {"inputs": {"x": [-1, 0, 1, 2], "y": [2, 1, 2, 5], "degree": 2}, "outputs": {"coefficients": [1.0, 0.0, 1.0]}},
               {"inputs": {"x": [1, 2, 3, 4], "y": [2, 3, 5, 4], "degree": 1}, "outputs": {"coefficients": [1.5, 0.8]}}],
     questions=[{"q": "Fit a quadratic to the points (0, 1), (1, 0), (2, 3), (3, 10) by least squares.", "outputs": {"coefficients": [1.0, -3.0, 2.0]}},
                {"q": "Find the least squares polynomial of degree 2 for x = (-1, 0, 1, 2), y = (2, 1, 2, 5).", "outputs": {"coefficients": [1.0, 0.0, 1.0]}},
                {"q": "Fit a cubic to the points (0, 1), (1, 0), (2, 3) by least squares.", "outcome": "INVALID_INPUT"}])


def least_squares_solution(A, b):
    A = _need_mat(A, "A")
    b = _need_vec(b, "b", len(A))
    if len(A) < len(A[0]):
        raise ValueError("fewer equations than unknowns: the least-squares solution is not unique")
    FA, Fb = _FM(A), _FV(b)
    x = _normal_solve(FA, Fb)
    r = [bi - sum((a * c for a, c in zip(row, x)), Fraction(0)) for row, bi in zip(FA, Fb)]
    return {"solution": _fl(x), "residual_norm": math.sqrt(float(sum((e * e for e in r), Fraction(0))))}


def _verify_lstsq(i, o):
    A, b, x = i["A"], i["b"], o["solution"]
    if len(x) != len(A[0]):
        return BAD
    r = [bi - v for v, bi in zip(_mv(A, x), b)]
    g = _mv(_T(A), r)
    s = _fro(A) * (_norm(b) + _fro(A) * _norm(x))
    return max(_norm(g) / max(1e-300, s), abs(o["residual_norm"] - _norm(r)) / max(1.0, _norm(b)))


_add("least_squares_solution",
     name="Least-squares solution of an overdetermined system", topic="least squares / overdetermined systems",
     purpose="The x minimizing |Ax - b| for a tall matrix A with independent columns, and the residual length.",
     inputs=_SYS_IN, outputs=[_O("solution", "least-squares solution x"), _O("residual_norm", "|Ax - b| at the solution")],
     primary="solution", conditions=["at least as many rows as columns", "the columns of A are linearly independent", "at most 10 x 10"],
     method="normal equations A^T A x = A^T b in exact rational arithmetic", latex=r"A^{T}A\,\hat{\mathbf{x}} = A^{T}\mathbf{b}",
     run=least_squares_solution, verify=_verify_lstsq,
     reference=lambda A, b: (lambda np: (lambda x: {"solution": x.tolist(), "residual_norm": float(np.linalg.norm(np.asarray(A, float) @ x - b))})(
         np.linalg.lstsq(np.asarray(A, float), np.asarray(b, float), rcond=None)[0]))(__import__("numpy")),
     wrong=lambda A, b: (lambda x: {"solution": x, "residual_norm": _norm([bi - v for v, bi in zip(_mv(A, x), b)])})(
         _fl(_solve_F(_FM(A[:len(A[0])]), _FV(b[:len(A[0])]))) if _det_F(_FM(A[:len(A[0])])) != 0 else [0.0] * len(A[0])),
     random_inputs=lambda rng: (lambda n: (lambda m: {"A": _rmat(rng, m, n), "b": _rvec(rng, m)})(rng.randint(n + 1, n + 4)))(rng.randint(1, 3)),
     parse=_safe_parse(lambda q: {"A": (_pick(q, "matrix", "A") or q.mats[0])["text"], "b": (_pick(q, "vector", "b") or q.vecs[0])["text"]}
                       if len(q.mats) == 1 and len(q.vecs) == 1 and q.has(r"\bleast[- ]squares\b")
                       and not q.has(r"\bline\b", r"\bpolynomial\b") else None),
     templates=["Find the least squares solution of Ax = b for A = [[1, 0], [0, 1], [1, 1]] and b = (1, 1, 0).",
                "Solve Ax = b in the least-squares sense with A = [[1, 1], [1, 2], [1, 3]] and b = [1, 2, 2]."],
     examples=[{"inputs": {"A": "[[1, 0], [0, 1], [1, 1]]", "b": "1, 1, 0"}, "outputs": {"solution": [1 / 3, 1 / 3], "residual_norm": math.sqrt(4 / 3)}},
               {"inputs": {"A": [[1, 1], [1, 2], [1, 3]], "b": [1, 2, 2]}, "outputs": {"solution": [2 / 3, 0.5]}}],
     questions=[{"q": "Find the least squares solution of Ax = b for A = [[1, 0], [0, 1], [1, 1]] and b = (1, 1, 0).", "outputs": {"solution": [1 / 3, 1 / 3]}},
                {"q": "Find the least squares solution of Ax = b for A = [[1, 2], [2, 4], [3, 6]] and b = (1, 1, 0).", "outcome": "INVALID_INPUT"}])


def pseudoinverse(A):
    A = _need_mat(A, "A")
    F = _FM(A)
    m, n = len(A), len(A[0])
    r = _rank_F(F)
    Ft = _T(F)
    if r == n:
        P = _mm_F(_inv_F(_mm_F(Ft, F)), Ft)
    elif r == m:
        P = _mm_F(Ft, _inv_F(_mm_F(F, Ft)))
    else:
        raise ValueError("this version computes the pseudoinverse of a matrix of full rank (independent rows or columns) only")
    return {"pseudoinverse": _fl(P)}


def _verify_pinv(i, o):
    """The four Penrose conditions A X A = A, X A X = X, (A X)^T = A X, (X A)^T = X A (they determine X uniquely)."""
    A, X = i["A"], o["pseudoinverse"]
    if _shape(X) != (len(A[0]), len(A)):
        return BAD
    AX, XA = _mm(A, X), _mm(X, A)
    s = max(1.0, _fro(A) * _fro(X))
    e1 = _fro([[a - b for a, b in zip(r, t)] for r, t in zip(_mm(AX, A), A)]) / max(1e-300, _fro(A) * s)
    e2 = _fro([[a - b for a, b in zip(r, t)] for r, t in zip(_mm(XA, X), X)]) / max(1e-300, _fro(X) * s)
    e3 = _fro([[a - b for a, b in zip(r, t)] for r, t in zip(AX, _T(AX))]) / s
    e4 = _fro([[a - b for a, b in zip(r, t)] for r, t in zip(XA, _T(XA))]) / s
    return max(e1, e2, e3, e4)


_add("pseudoinverse",
     name="Moore-Penrose pseudoinverse (full rank)", topic="least squares / pseudoinverse",
     purpose="The pseudoinverse A+ of a matrix with independent columns, (A^T A)^-1 A^T, or independent rows, A^T (A A^T)^-1.",
     inputs=[_I("A", "matrix", "a matrix of full rank (up to 10 x 10)")], outputs=[_O("pseudoinverse", "the matrix A+")],
     primary="pseudoinverse", conditions=["A has full rank (independent rows or independent columns)", "at most 10 x 10"],
     method="the full-rank formulas in exact rational arithmetic", latex=r"A^{+} = (A^{T}A)^{-1}A^{T}\ \text{or}\ A^{T}(AA^{T})^{-1}",
     run=pseudoinverse, verify=_verify_pinv,
     reference=lambda A: {"pseudoinverse": __import__("numpy").linalg.pinv(__import__("numpy").asarray(A, float)).tolist()},
     wrong=lambda A: {"pseudoinverse": _T(pseudoinverse(A)["pseudoinverse"])},
     random_inputs=lambda rng: (lambda m, n: {"A": _rmat(rng, m, n)})(*rng.choice([(3, 2), (4, 2), (2, 3), (2, 4), (4, 3), (3, 4), (3, 3)])),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\bpseudo-?inverse\b", r"\bmoore-penrose\b", r"\bgeneralized inverse\b") else None),
     templates=["Find the pseudoinverse of [[1, 0], [0, 1], [1, 1]].", "Compute the Moore-Penrose inverse of [[1, 2, 3]]."],
     examples=[{"inputs": {"A": "[[1, 0], [0, 1], [1, 1]]"}, "outputs": {"pseudoinverse": [[2 / 3, -1 / 3, 1 / 3], [-1 / 3, 2 / 3, 1 / 3]]}},
               {"inputs": {"A": [[1, 2, 3]]}, "outputs": {"pseudoinverse": [[1 / 14], [2 / 14], [3 / 14]]}}],
     questions=[{"q": "Find the pseudoinverse of [[1, 0], [0, 1], [1, 1]].", "outputs": {"pseudoinverse": [[2 / 3, -1 / 3, 1 / 3], [-1 / 3, 2 / 3, 1 / 3]]}},
                {"q": "Find the pseudoinverse of [[1, 2], [2, 4], [3, 6]].", "outcome": "INVALID_INPUT"}])


def projection_matrix(A):
    A = _need_mat(A, "A")
    F = _FM(A)
    if _rank_F(F) != len(A[0]):
        raise ValueError("the columns of A must be linearly independent")
    Ft = _T(F)
    return {"projection": _fl(_mm_F(_mm_F(F, _inv_F(_mm_F(Ft, F))), Ft))}


def _verify_projmat(i, o):
    """P symmetric, P^2 = P, P A = A and trace P = number of columns: the orthogonal projector onto col A."""
    A, P = i["A"], o["projection"]
    m = len(A)
    if _shape(P) != (m, m):
        return BAD
    PP, PA = _mm(P, P), _mm(P, A)
    e1 = max(abs(P[r][c] - P[c][r]) for r in range(m) for c in range(m))
    e2 = max(abs(a - b) for r, t in zip(PP, P) for a, b in zip(r, t))
    e3 = max(abs(a - b) for r, t in zip(PA, A) for a, b in zip(r, t)) / max(1.0, _fro(A))
    e4 = abs(math.fsum(P[k][k] for k in range(m)) - len(A[0]))
    return max(e1, e2, e3, e4)


_add("projection_matrix",
     name="Orthogonal projection matrix onto a column space", topic="least squares / orthogonal projections",
     purpose="The matrix P = A (A^T A)^-1 A^T that projects orthogonally onto the column space of A.",
     inputs=[_I("A", "matrix", "a matrix with independent columns")], outputs=[_O("projection", "the projection matrix P")],
     primary="projection", conditions=["the columns of A are linearly independent", "at most 10 x 10"],
     method="P = A (A^T A)^-1 A^T in exact rational arithmetic", latex=r"P = A(A^{T}A)^{-1}A^{T}",
     run=projection_matrix, verify=_verify_projmat,
     reference=lambda A: (lambda Q: {"projection": (Q @ Q.T).tolist()})(__import__("scipy.linalg", fromlist=["orth"]).orth(__import__("numpy").asarray(A, float))),
     wrong=lambda A: {"projection": _mm(A, _T(A))},
     random_inputs=lambda rng: (lambda m: {"A": _rmat(rng, m, rng.randint(1, m - 1))})(rng.randint(2, 4)),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bprojection matrix\b", r"\bprojector\b")
                       else None),
     templates=["Find the projection matrix onto the column space of [[1, 0], [1, 1], [0, 1]].",
                "What is the projection matrix onto the column space of [1; 2]?"],
     examples=[{"inputs": {"A": "1; 2"}, "outputs": {"projection": [[0.2, 0.4], [0.4, 0.8]]}},
               {"inputs": {"A": [[1, 0], [0, 1], [0, 0]]}, "outputs": {"projection": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.0]]}}],
     questions=[{"q": "What is the projection matrix onto the column space of [1; 2]?", "outputs": {"projection": [[0.2, 0.4], [0.4, 0.8]]}},
                {"q": "Find the projection matrix onto the column space of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


# ============================================================================================================
# BATCH 3 — decompositions and eigenproblems (floating-point algorithms, plain Python)
# ============================================================================================================
def _sign_fix(v, tol=1e-9):
    """Unit vector convention: the first entry whose magnitude is within tol of the largest is made positive."""
    if not v:
        return v
    big = max(abs(complex(x)) for x in v)
    for x in v:
        if abs(complex(x)) >= big - tol * max(1.0, big):
            return [-y for y in v] if (complex(x).real < 0) else list(v)
    return list(v)


def _sign_ok(v, tol=1e-9):
    return _sign_fix(v, tol) == list(v)


def _rnd_float_mat(rng, m, n=None, lo=-10.0, hi=10.0):
    return [[round(rng.uniform(lo, hi), 6) for _ in range(n or m)] for _ in range(m)]


def _jacobi_eig(A):
    """Cyclic Jacobi rotations for a symmetric matrix: (eigenvalues, eigenvector columns), unsorted."""
    n = len(A)
    a = [[float(x) for x in r] for r in A]
    V = _eye(n)
    fro = _fro(a)
    for _ in range(100):
        off = math.sqrt(math.fsum(a[i][j] ** 2 for i in range(n) for j in range(i + 1, n)))
        if off == 0 or off <= 1e-17 * fro:
            break
        for p in range(n - 1):
            for q in range(p + 1, n):
                if a[p][q] == 0:
                    continue
                theta = (a[q][q] - a[p][p]) / (2 * a[p][q])
                t = math.copysign(1.0, theta) / (abs(theta) + math.hypot(1.0, theta))
                c = 1 / math.hypot(1.0, t)
                s = t * c
                for k in range(n):
                    akp, akq = a[k][p], a[k][q]
                    a[k][p], a[k][q] = c * akp - s * akq, s * akp + c * akq
                for k in range(n):
                    apk, aqk = a[p][k], a[q][k]
                    a[p][k], a[q][k] = c * apk - s * aqk, s * apk + c * aqk
                for k in range(n):
                    vkp, vkq = V[k][p], V[k][q]
                    V[k][p], V[k][q] = c * vkp - s * vkq, s * vkp + c * vkq
    else:
        raise ValueError("the Jacobi iteration did not converge")
    return [a[i][i] for i in range(n)], V


def _need_symmetric(A, name="A"):
    A = _need_mat(A, name, square=True)
    if any(_F(A[i][j]) != _F(A[j][i]) for i in range(len(A)) for j in range(i)):
        raise ValueError(f"{name} must be symmetric")
    return A


def _svd_jacobi(A):
    """One-sided Jacobi (Hestenes) SVD: (U columns, singular values descending, V columns) of an m x n matrix, m >= n."""
    m, n = len(A), len(A[0])
    cols = [[float(A[i][j]) for i in range(m)] for j in range(n)]
    V = [[1.0 if i == j else 0.0 for i in range(n)] for j in range(n)]      # V stored by columns
    for _ in range(80):
        rotated = False
        for i in range(n - 1):
            for j in range(i + 1, n):
                ci, cj = cols[i], cols[j]
                a, b, g = _dot(ci, ci), _dot(cj, cj), _dot(ci, cj)
                if g == 0 or abs(g) <= max(1e-15, 2.3e-16 * m) * math.sqrt(a * b):
                    continue
                rotated = True
                zeta = (b - a) / (2 * g)
                t = math.copysign(1.0, zeta) / (abs(zeta) + math.hypot(1.0, zeta))
                c = 1 / math.hypot(1.0, t)
                s = c * t
                cols[i], cols[j] = [c * x - s * y for x, y in zip(ci, cj)], [s * x + c * y for x, y in zip(ci, cj)]
                vi, vj = V[i], V[j]
                V[i], V[j] = [c * x - s * y for x, y in zip(vi, vj)], [s * x + c * y for x, y in zip(vi, vj)]
        if not rotated:
            break
    else:
        raise ValueError("the Jacobi SVD did not converge")
    sv = [_norm(c) for c in cols]
    order = sorted(range(n), key=lambda k: -sv[k])
    U = [[x / sv[k] for x in cols[k]] if sv[k] > 0 else [0.0] * m for k in order]
    return U, [sv[k] for k in order], [V[k] for k in order]


def singular_values(A):
    A = _need_mat(A, "A")
    M = A if len(A) >= len(A[0]) else _T(A)
    return {"singular_values": _svd_jacobi(M)[1]}


def _gram_power_sums(A, kmax):
    """Exact tr((G)^k), k = 1..kmax, for the smaller Gram matrix G of A (A^T A or A A^T)."""
    F = _FM(A)
    G = _mm_F(_T(F), F) if len(A) >= len(A[0]) else _mm_F(F, _T(F))
    P, out = [r[:] for r in G], []
    for _ in range(kmax):
        out.append(float(sum((P[i][i] for i in range(len(G))), Fraction(0))))
        P = _mm_F(P, G)
    return out


def _verify_singular(i, o):
    """sum sigma^(2k) = tr((A^T A)^k) for k = 1..n (exact traces) and descending order."""
    A, s = i["A"], o["singular_values"]
    n = min(len(A), len(A[0]))
    if len(s) != n or any(x < 0 for x in s) or any(s[k] < s[k + 1] for k in range(n - 1)):
        return BAD
    p = _gram_power_sums(A, n)
    sc = max(1.0, _fro(A)) ** 2
    return max(abs(math.fsum(x ** (2 * k) for x in s) - p[k - 1]) / sc ** k for k in range(1, n + 1))


_add("singular_values",
     name="Singular values of a matrix", topic="decompositions / singular value decomposition",
     purpose="The singular values of a matrix (any shape up to 10 x 10), largest first.",
     inputs=[_I("A", "matrix", "the matrix")], outputs=[_O("singular_values", "singular values, descending")], primary="singular_values",
     conditions=["at most 10 x 10"], method="one-sided Jacobi (Hestenes) orthogonalization of the columns; the column lengths are the singular values",
     latex=r"A = U\Sigma V^{T},\quad \sigma_i = \sqrt{\lambda_i(A^{T}A)}",
     run=singular_values, verify=_verify_singular,
     reference=lambda A: {"singular_values": __import__("numpy").linalg.svd(__import__("numpy").asarray(A, float), compute_uv=False).tolist()},
     wrong=lambda A: {"singular_values": sorted((_norm(c) for c in (_T(A) if len(A) >= len(A[0]) else A)), reverse=True)},
     random_inputs=lambda rng: {"A": _rnd_float_mat(rng, rng.randint(2, 5), rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bsingular values?\b")
                       and not q.has(r"\bcondition\b", r"\blow-?rank\b", r"\bapproximation\b") else None),
     templates=["Find the singular values of [[3, 0], [4, 5]].", "What are the singular values of the matrix [1 0 0; 0 2 0]?"],
     examples=[{"inputs": {"A": "[[3, 0], [4, 5]]"}, "outputs": {"singular_values": [math.sqrt(45), math.sqrt(5)]}},
               {"inputs": {"A": [[1, 0, 0], [0, 2, 0]]}, "outputs": {"singular_values": [2.0, 1.0]}}],
     questions=[{"q": "Find the singular values of [[3, 0], [4, 5]].", "outputs": {"singular_values": [math.sqrt(45), math.sqrt(5)]}},
                {"q": "What are the singular values of the matrix [1 0 0; 0 2 0]?", "outputs": {"singular_values": [2.0, 1.0]}}])

_add("matrix_norms",
     name="Norms of a matrix", topic="matrices / norms and conditioning", loose_ok=_NORM_NAMES,
     purpose="The Frobenius norm, the 1-norm (largest column sum), the infinity-norm (largest row sum) and the spectral 2-norm.",
     inputs=[_I("A", "matrix", "the matrix")],
     outputs=[_O("frobenius", "Frobenius norm"), _O("norm1", "1-norm: largest absolute column sum"),
              _O("norm_inf", "infinity-norm: largest absolute row sum"), _O("norm2", "spectral norm: the largest singular value")],
     primary="frobenius", conditions=["at most 10 x 10"],
     method="exact sums for the Frobenius, 1- and infinity-norms; one-sided Jacobi SVD for the 2-norm",
     latex=r"\|A\|_F = \sqrt{\textstyle\sum a_{ij}^2},\ \|A\|_1 = \max_j \textstyle\sum_i |a_{ij}|,\ \|A\|_\infty = \max_i \textstyle\sum_j |a_{ij}|,\ \|A\|_2 = \sigma_{\max}",
     run=matrix_norms,
     verify=lambda i, o: (lambda w, s2: max(abs(o["frobenius"] ** 2 - w["frobenius"] ** 2) / max(1.0, w["frobenius"] ** 2),
                                            _rel(o["norm1"], w["norm1"]), _rel(o["norm_inf"], w["norm_inf"]), _rel(o["norm2"], s2)))(
         matrix_norms(i["A"]), _svd_jacobi(_T(i["A"]) if len(i["A"]) < len(i["A"][0]) else [r[::-1] for r in i["A"]])[1][0]),
     reference=lambda A: (lambda np, M: {"frobenius": float(np.linalg.norm(M, "fro")), "norm1": float(np.linalg.norm(M, 1)),
                                         "norm_inf": float(np.linalg.norm(M, np.inf)), "norm2": float(np.linalg.norm(M, 2))})(
         __import__("numpy"), __import__("numpy").asarray(A, float)),
     wrong=lambda A: (lambda r: dict(r, norm1=r["norm_inf"], norm_inf=r["norm1"]))(matrix_norms(A)),
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 4), rng.randint(2, 4))},
     # the norms OF the given matrix: not of an error, residual or difference (a low-rank or other derived matrix)
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bnorms?\b")
                       and not q.has(r"\bcondition\b", r"\bnormali[sz]e", r"\berrors?\b", r"\bresidual", r"\bdifference\b",
                                     r"\bapproxima(?:t|nt)", r"\brank\b", r"\btruncat", r"\breconstruct", r"\bsubspace\b", r"\bproject",
                                     r"\binverse\b", r"\bexponential\b", r"\bsquare root\b", r"\bpower\b") else None),
     templates=["Find the norms of the matrix [[1, -2], [3, 4]].", "What is the Frobenius norm of [1 2; 3 4]?"],
     examples=[{"inputs": {"A": "[[1, -2], [3, 4]]"}, "outputs": {"frobenius": math.sqrt(30), "norm1": 6.0, "norm_inf": 7.0}},
               {"inputs": {"A": [[3, 0], [0, -4]]}, "outputs": {"frobenius": 5.0, "norm1": 4.0, "norm_inf": 4.0, "norm2": 4.0}}],
     questions=[{"q": "Find the norms of the matrix [[1, -2], [3, 4]].", "outputs": {"norm1": 6.0, "norm_inf": 7.0}},
                {"q": "What is the Frobenius norm of [1 2; 3 4]?", "outputs": {"frobenius": math.sqrt(30)}}])


def condition_number(A):
    A = _need_mat(A, "A", square=True)
    F = _FM(A)
    if _det_F(F) == 0:
        raise ValueError("the matrix is singular: its condition number is infinite")
    s = _svd_jacobi(A)[1]
    inv = _inv_F(F)
    n1 = lambda M: max(sum((abs(r[j]) for r in M), Fraction(0)) for j in range(len(M)))
    return {"cond2": s[0] / s[-1], "cond1": float(n1(F) * n1(inv))}


def _verify_cond(i, o):
    A = i["A"]
    s = _svd_jacobi([r[::-1] for r in _T(A)])[1]          # the SVD of a permuted transpose: another computation
    F = _FM(A)
    n1 = lambda M: max(sum((abs(r[j]) for r in M), Fraction(0)) for j in range(len(M)))
    return max(_rel(o["cond2"], s[0] / s[-1]) if o["cond2"] > 0 else BAD,
               _rel(o["cond1"], float(n1(F) * n1(_inv_F(F)))))


_add("condition_number",
     name="Condition number of a matrix", topic="matrices / norms and conditioning", loose_ok=_NORM_NAMES,
     purpose="How sensitive Ax = b is to errors: the 2-norm condition number sigma_max/sigma_min and the 1-norm one.",
     inputs=[_I("A", "matrix", "a nonsingular square matrix")],
     outputs=[_O("cond2", "2-norm condition number sigma_max / sigma_min"), _O("cond1", "1-norm condition number |A|_1 |A^-1|_1")],
     primary="cond2", conditions=["A is square and nonsingular", "at most 10 x 10"],
     method="singular values by one-sided Jacobi; the 1-norm one from the exact inverse",
     latex=r"\kappa_2(A) = \frac{\sigma_{\max}}{\sigma_{\min}},\quad \kappa_1(A) = \|A\|_1\|A^{-1}\|_1",
     run=condition_number, verify=_verify_cond,
     reference=lambda A: (lambda np, M: {"cond2": float(np.linalg.cond(M)), "cond1": float(np.linalg.cond(M, 1))})(__import__("numpy"), __import__("numpy").asarray(A, float)),
     wrong=lambda A: (lambda r: dict(r, cond2=_fro(A) * _fro(matrix_inverse(A)["inverse"])))(condition_number(A)),
     random_inputs=lambda rng: {"A": _rmat(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bcondition number\b") else None),
     templates=["Find the condition number of [[1, 2], [3, 4]].", "What is the condition number of the matrix [2 0; 0 0.5]?"],
     examples=[{"inputs": {"A": "[[2, 0], [0, 0.5]]"}, "outputs": {"cond2": 4.0, "cond1": 4.0}},
               {"inputs": {"A": [[1, 2], [3, 4]]}, "outputs": {"cond1": 21.0}}],
     questions=[{"q": "What is the condition number of the matrix [2 0; 0 0.5]?", "outputs": {"cond2": 4.0}},
                {"q": "Find the condition number of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


def low_rank_approximation(A, k):
    A = _need_mat(A, "A")
    m, n = len(A), len(A[0])
    r = min(m, n)
    if isinstance(k, bool) or k != int(k) or not 1 <= k <= r:
        raise ValueError(f"k must be a whole number from 1 to {r}")
    k = int(k)
    tall = m >= n
    U, s, V = _svd_jacobi(A if tall else _T(A))
    if k < r and s[k - 1] - s[k] <= 1e-9 * max(1.0, s[0]):
        raise ValueError("sigma_k = sigma_(k+1): the best rank-k approximation is not unique")
    B = [[math.fsum(s[t] * U[t][i] * V[t][j] for t in range(k)) for j in range(len(V[0]))] for i in range(len(U[0]))]
    if not tall:
        B = _T(B)
    return {"approximation": B, "error2": s[k] if k < r else 0.0, "errorF": math.sqrt(math.fsum(x * x for x in s[k:]))}


def _verify_lowrank(i, o):
    A, k = i["A"], int(i["k"])
    w = low_rank_approximation([r[::-1] for r in A], k)              # columns reversed: another Jacobi ordering
    B = [r[::-1] for r in w["approximation"]]
    sc = max(1.0, _fro(A))
    return max(max(abs(a - b) for r, t in zip(o["approximation"], B) for a, b in zip(r, t)) / sc,
               abs(o["error2"] - w["error2"]) / sc, abs(o["errorF"] - w["errorF"]) / sc) if _shape(o["approximation"]) == _shape(A) else BAD


def _reference_lowrank(A, k):
    import numpy as np
    U, s, Vt = np.linalg.svd(np.asarray(A, float), full_matrices=False)
    k = int(k)
    return {"approximation": (U[:, :k] * s[:k] @ Vt[:k]).tolist(), "error2": float(s[k]) if k < len(s) else 0.0,
            "errorF": float(np.sqrt(np.sum(s[k:] ** 2)))}


def _wrong_lowrank(A, k):
    """Keeps the k SMALLEST singular values (a plausible ordering mistake)."""
    m, n = len(A), len(A[0])
    tall = m >= n
    U, s, V = _svd_jacobi(A if tall else _T(A))
    idx = list(range(len(s)))[-int(k):]
    B = [[math.fsum(s[t] * U[t][i] * V[t][j] for t in idx) for j in range(len(V[0]))] for i in range(len(U[0]))]
    rest = [s[t] for t in range(len(s)) if t not in idx]
    return {"approximation": B if tall else _T(B), "error2": max(rest, default=0.0), "errorF": math.sqrt(math.fsum(x * x for x in rest))}


def _parse_low_rank(q):
    """A rank-k approximation (approximation / approximant / approximating / low-rank / truncated SVD /
    reconstruction) with its rank stated ('rank-1', 'rank 2', 'target rank k of 1', 'k = 2'), and no other number."""
    if len(q.mats) != 1 or q.vecs or not q.has(r"\bapproxima(?:t|nt)",r"\blow-?rank\b", r"\btruncated (?:svd|singular value decomposition)\b",
                                                r"\breconstruct"):
        return None
    if q.has(r"\bnuclear\b", r"\btrace norm\b", r"\bmax(?:imum)? norm\b", r"\binfinity[- ]norm\b", r"\b1-norm\b"):
        return None                                          # an error norm this capability does not compute
    ks ={float(m.group(1) or m.group(2)) for m in re.finditer(
        r"\brank[- ](\d+)\b|\brank\s*k?\s*(?:=|:|of|is(?: set to)?|equal to|equals)\s*(\d+)\b", q.rest)}
    if q.named("k") is not None:
        ks.add(q.named("k"))
    if len(ks) != 1:
        return None                                          # no rank, or two different ranks: not one reading
    k = ks.pop()
    if any(x != k for x in _loose(q)):
        return None                                          # every number stated is the rank k
    return {"A": q.mats[0]["text"], "k": k}


_add("low_rank_approximation",
     name="Best rank-k approximation (Eckart-Young)", topic="decompositions / singular value decomposition",
     purpose="The rank-k matrix closest to A in the 2- and Frobenius norms (truncated SVD), with both approximation errors.",
     inputs=[_I("A", "matrix", "the matrix"), _I("k", "integer", "target rank k", min=1)],
     outputs=[_O("approximation", "the best rank-k approximation A_k"), _O("error2", "|A - A_k|_2 = sigma_(k+1)"),
              _O("errorF", "|A - A_k|_F = sqrt(sigma_(k+1)^2 + ...)")], primary="error2",
     conditions=["1 <= k <= min(rows, columns)", "sigma_k > sigma_(k+1) (otherwise A_k is not unique)", "at most 10 x 10"],
     method="one-sided Jacobi SVD, keep the k largest singular triplets", latex=r"A_k = \sum_{i=1}^{k} \sigma_i \mathbf{u}_i \mathbf{v}_i^{T}",
     run=low_rank_approximation, verify=_verify_lowrank, reference=_reference_lowrank, wrong=_wrong_lowrank,
     random_inputs=lambda rng: (lambda m, n: {"A": _rnd_float_mat(rng, m, n), "k": rng.randint(1, min(m, n) - 1)})(rng.randint(2, 5), rng.randint(2, 5)),
     parse=_safe_parse(_parse_low_rank),
     templates=["Find the best rank-1 approximation of [[3, 0], [0, 1]].", "Compute the best rank 1 approximation of the matrix [2 0 0; 0 3 0; 0 0 1]."],
     examples=[{"inputs": {"A": "[[3, 0], [0, 1]]", "k": 1}, "outputs": {"approximation": [[3.0, 0.0], [0.0, 0.0]], "error2": 1.0, "errorF": 1.0}},
               {"inputs": {"A": [[2, 0, 0], [0, 3, 0], [0, 0, 1]], "k": 2}, "outputs": {"error2": 1.0}}],
     questions=[{"q": "Find the best rank-1 approximation of [[3, 0], [0, 1]].", "outputs": {"approximation": [[3.0, 0.0], [0.0, 0.0]], "error2": 1.0}},
                {"q": "Find the best rank-1 approximation of [[1, 0], [0, 1]].", "outcome": "INVALID_INPUT"}])


# --- LU, QR, Cholesky, Gram-Schmidt ------------------------------------------------------------------------------
def lu_decomposition(A):
    A = _need_mat(A, "A", square=True)
    if _det_F(_FM(A)) == 0:
        raise ValueError("A is singular: this capability factors nonsingular matrices")
    n = len(A)
    U = [[float(x) for x in r] for r in A]
    L = [[0.0] * n for _ in range(n)]
    perm = list(range(n))
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(U[i][k]))      # the first row of largest magnitude
        if p != k:
            U[k], U[p] = U[p], U[k]
            L[k], L[p] = L[p], L[k]
            perm[k], perm[p] = perm[p], perm[k]
        for i in range(k + 1, n):
            f = U[i][k] / U[k][k]
            L[i][k] = f
            U[i] = [a - f * b if j >= k else a for j, (a, b) in enumerate(zip(U[i], U[k]))]
            U[i][k] = 0.0
    for i in range(n):
        L[i][i] = 1.0
    P = [[0.0] * n for _ in range(n)]                  # A = P L U: row i of P A' ... P[perm[i]][i] = 1
    for i, r in enumerate(perm):
        P[r][i] = 1.0
    return {"P": P, "L": L, "U": U}


def _verify_lu(i, o):
    """P a permutation, L unit lower triangular with |l_ij| <= 1 (partial pivoting), U upper triangular, P L U = A."""
    A, P, L, U = i["A"], o["P"], o["L"], o["U"]
    n = len(A)
    if any(_shape(M) != (n, n) for M in (P, L, U)):
        return BAD
    if sorted(tuple(r) for r in P) != sorted(tuple(r) for r in _eye(n)) or any(sum(c) != 1 for c in zip(*P)):
        return BAD
    if any(L[r][r] != 1.0 for r in range(n)) or any(L[r][c] != 0 for r in range(n) for c in range(r + 1, n)):
        return BAD
    if any(abs(L[r][c]) > 1 + 1e-12 for r in range(n) for c in range(r)) or any(U[r][c] != 0 for r in range(n) for c in range(r)):
        return BAD
    PLU = _mm(P, _mm(L, U))
    return max(abs(a - b) for r, t in zip(PLU, A) for a, b in zip(r, t)) / max(1.0, _fro(A))


def _wrong_lu(A):
    """Doolittle without pivoting: P = I."""
    n = len(A)
    U = [[float(x) for x in r] for r in A]
    L = _eye(n)
    for k in range(n):
        if U[k][k] == 0:
            return lu_decomposition(A)
        for i in range(k + 1, n):
            f = U[i][k] / U[k][k]
            L[i][k] = f
            U[i] = [a - f * b for a, b in zip(U[i], U[k])]
            U[i][k] = 0.0
    return {"P": _eye(n), "L": L, "U": U}


_add("lu_decomposition",
     name="LU decomposition with partial pivoting", topic="decompositions / LU",
     purpose="Factor a nonsingular square matrix as A = P L U (permutation, unit lower triangular, upper triangular).",
     inputs=[_I("A", "matrix", "a nonsingular square matrix")],
     outputs=[_O("P", "permutation matrix (A = P L U)"), _O("L", "unit lower triangular factor, |l_ij| <= 1"), _O("U", "upper triangular factor")],
     primary="U", conditions=["A is square and nonsingular", "at most 10 x 10",
                              "pivot: the first row of largest magnitude in the column (the scipy.linalg.lu convention A = P L U)"],
     method="Gaussian elimination with partial pivoting (Doolittle form) in floating point",
     latex=r"A = PLU", run=lu_decomposition, verify=_verify_lu,
     reference=lambda A: (lambda r: {"P": r[0].tolist(), "L": r[1].tolist(), "U": r[2].tolist()})(
         __import__("scipy.linalg", fromlist=["lu"]).lu(__import__("numpy").asarray(A, float))),
     wrong=_wrong_lu,
     random_inputs=lambda rng: {"A": _rnd_float_mat(rng, rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\blu\b", r"\blu decomposition\b", r"\blu factori[sz]ation\b")
                       and not q.has(r"\bsolve\b") else None),
     templates=["Find the LU decomposition of [[1, 2], [3, 4]].", "Compute the LU factorization of the matrix [2 1 1; 4 3 3; 8 7 9]."],
     examples=[{"inputs": {"A": "[[1, 2], [3, 4]]"}, "outputs": {"P": [[0.0, 1.0], [1.0, 0.0]], "L": [[1.0, 0.0], [1 / 3, 1.0]],
                                                                 "U": [[3.0, 4.0], [0.0, 2 / 3]]}},
               {"inputs": {"A": [[2, 1, 1], [4, 3, 3], [8, 7, 9]]},
                "outputs": {"P": [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]], "U": [[8.0, 7.0, 9.0], [0.0, -0.75, -1.25], [0.0, 0.0, -2 / 3]]}}],
     questions=[{"q": "Find the LU decomposition of [[1, 2], [3, 4]].", "outputs": {"L": [[1.0, 0.0], [1 / 3, 1.0]], "U": [[3.0, 4.0], [0.0, 2 / 3]]}},
                {"q": "Find the LU decomposition of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


def _qr_house(A):
    """Householder QR of an m x n matrix (m >= n, independent columns): reduced Q (m x n), R (n x n), diag(R) > 0."""
    m, n = len(A), len(A[0])
    R = [[float(x) for x in r] for r in A]
    vs = []
    for k in range(n):
        x = [R[i][k] for i in range(k, m)]
        nx = _norm(x)
        alpha = -math.copysign(nx, x[0])
        v = x[:]
        v[0] -= alpha
        nv = _norm(v)
        if nv == 0:
            vs.append(None)
            continue
        v = [t / nv for t in v]
        vs.append(v)
        for j in range(k, n):
            d = 2 * math.fsum(v[i - k] * R[i][j] for i in range(k, m))
            for i in range(k, m):
                R[i][j] -= d * v[i - k]
    Q = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(m)]
    for k in reversed(range(n)):
        v = vs[k]
        if v is None:
            continue
        for j in range(n):
            d = 2 * math.fsum(v[i - k] * Q[i][j] for i in range(k, m))
            for i in range(k, m):
                Q[i][j] -= d * v[i - k]
    R = [[R[i][j] if j >= i else 0.0 for j in range(n)] for i in range(n)]
    for i in range(n):
        if R[i][i] < 0:
            R[i] = [-x for x in R[i]]
            for r in range(m):
                Q[r][i] = -Q[r][i]
    return Q, R


def qr_decomposition(A):
    A = _need_mat(A, "A")
    if len(A) < len(A[0]):
        raise ValueError("A needs at least as many rows as columns")
    if _rank_F(_FM(A)) < len(A[0]):
        raise ValueError("the columns of A are linearly dependent")
    Q, R = _qr_house(A)
    return {"Q": Q, "R": R}


def _verify_qr(i, o):
    """Q^T Q = I, R upper triangular with a positive diagonal, Q R = A (this determines Q and R uniquely)."""
    A, Q, R = i["A"], o["Q"], o["R"]
    m, n = len(A), len(A[0])
    if _shape(Q) != (m, n) or _shape(R) != (n, n):
        return BAD
    if any(R[r][c] != 0 for r in range(n) for c in range(r)) or any(R[r][r] <= 0 for r in range(n)):
        return BAD
    QtQ = _mm(_T(Q), Q)
    e1 = max(abs(a - b) for r, t in zip(QtQ, _eye(n)) for a, b in zip(r, t))
    QR = _mm(Q, R)
    e2 = max(abs(a - b) for r, t in zip(QR, A) for a, b in zip(r, t)) / max(1.0, _fro(A))
    return max(e1, e2)


def _reference_qr(A):
    import numpy as np
    Q, R = np.linalg.qr(np.asarray(A, float))
    d = np.sign(np.diag(R))
    return {"Q": (Q * d).tolist(), "R": (R * d[:, None]).tolist()}


_add("qr_decomposition",
     name="QR decomposition (Householder)", topic="decompositions / QR",
     purpose="Factor A = Q R with orthonormal columns Q and upper triangular R (reduced form, positive diagonal of R).",
     inputs=[_I("A", "matrix", "a matrix with at least as many rows as columns, independent columns")],
     outputs=[_O("Q", "m x n matrix with orthonormal columns"), _O("R", "n x n upper triangular, positive diagonal")], primary="R",
     conditions=["rows >= columns", "independent columns", "at most 10 x 10", "convention: the diagonal of R is positive (unique)"],
     method="Householder reflections, then signs fixed so that diag(R) > 0", latex=r"A = QR,\ Q^{T}Q = I",
     run=qr_decomposition, verify=_verify_qr, reference=_reference_qr,
     wrong=lambda A: (lambda Q: {"Q": Q, "R": [[(_dot([r[i] for r in Q], [r[j] for r in A]) if j >= i else 0.0) for j in range(len(A[0]))] for i in range(len(A[0]))]})(
         _T([[x / _norm(c) for x in c] for c in _T(A)])),
     random_inputs=lambda rng: (lambda n: {"A": _rnd_float_mat(rng, rng.randint(n, n + 2), n)})(rng.randint(2, 4)),
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bqr\b") and not q.has(r"\beigen") else None),
     templates=["Find the QR decomposition of [[3, 1], [4, 2]].", "Compute the QR factorization of the matrix [1 1; 1 0; 0 1]."],
     examples=[{"inputs": {"A": "[[3, 1], [4, 2]]"}, "outputs": {"Q": [[0.6, -0.8], [0.8, 0.6]], "R": [[5.0, 2.2], [0.0, 0.4]]}},
               {"inputs": {"A": [[1, 0], [0, 1], [0, 0]]}, "outputs": {"Q": [[1.0, 0.0], [0.0, 1.0], [0.0, 0.0]], "R": [[1.0, 0.0], [0.0, 1.0]]}}],
     questions=[{"q": "Find the QR decomposition of [[3, 1], [4, 2]].", "outputs": {"Q": [[0.6, -0.8], [0.8, 0.6]], "R": [[5.0, 2.2], [0.0, 0.4]]}},
                {"q": "Find the QR decomposition of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


def gram_schmidt(vectors):
    V = _need_mat(vectors, "vectors")
    k, n = len(V), len(V[0])
    if k > n or _rank_F(_FM(V)) < k:
        raise ValueError("the vectors must be linearly independent")
    E = []
    for v in V:                                          # modified Gram-Schmidt with one re-orthogonalization pass
        w = [float(x) for x in v]
        for _ in range(2):
            for e in E:
                d = _dot(e, w)
                w = [a - d * b for a, b in zip(w, e)]
        nw = _norm(w)
        E.append([x / nw for x in w])
    return {"orthonormal": E}


def _verify_gs(i, o):
    """Rows orthonormal; R = E V^T is upper triangular with a positive diagonal and V = R^T E (span and order kept)."""
    V, E = i["vectors"], o["orthonormal"]
    k = len(V)
    if _shape(E) != _shape(V):
        return BAD
    G = _mm(E, _T(E))
    e1 = max(abs(a - b) for r, t in zip(G, _eye(k)) for a, b in zip(r, t))
    R = _mm(E, _T(V))                                     # R[a][b] = e_a . v_b
    sc = max(1.0, _fro(V))
    e2 = max((abs(R[a][b]) for a in range(k) for b in range(a)), default=0.0) / sc
    if any(R[a][a] <= 0 for a in range(k)):
        return BAD
    back = _mm(_T(R), E)
    e3 = max(abs(x - y) for r, t in zip(back, V) for x, y in zip(r, t)) / sc
    return max(e1, e2, e3)


_add("gram_schmidt",
     name="Gram-Schmidt orthonormalization", topic="vector spaces / orthogonality",
     purpose="Turn linearly independent vectors v1, ..., vk into orthonormal vectors e1, ..., ek spanning the same nested subspaces.",
     inputs=[_I("vectors", "matrix", "the vectors, one per row (independent)")], outputs=[_O("orthonormal", "orthonormal vectors e1, ..., ek, one per row")],
     primary="orthonormal", conditions=["the vectors are linearly independent", "at most 10 vectors of at most 10 entries"],
     method="modified Gram-Schmidt with re-orthogonalization (e_j . v_j > 0 by construction)",
     latex=r"\mathbf{w}_j = \mathbf{v}_j - \sum_{i<j} (\mathbf{e}_i\cdot\mathbf{v}_j)\,\mathbf{e}_i,\quad \mathbf{e}_j = \mathbf{w}_j/\|\mathbf{w}_j\|",
     run=gram_schmidt, verify=_verify_gs,
     reference=lambda vectors: (lambda r: {"orthonormal": _T(r["Q"])})(_reference_qr(_T(vectors))),
     wrong=lambda vectors: (lambda E: {"orthonormal": [[x * _norm(v) for x in e] for e, v in zip(E, vectors)]})(gram_schmidt(vectors)["orthonormal"]),
     random_inputs=lambda rng: (lambda n: {"vectors": _rnd_float_mat(rng, rng.randint(2, n), n)})(rng.randint(2, 4)),
     parse=_safe_parse(lambda q: ({"vectors": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs else
                                  {"vectors": "; ".join(v["text"] for v in q.vecs)} if len(q.vecs) >= 2 and not q.mats else None)
                       if q.has(r"\bgram-?schmidt\b", r"\borthonormali[sz]e\b", r"\borthonormal basis\b") else None),
     templates=["Apply Gram-Schmidt to the vectors (3, 4) and (1, 0).", "Orthonormalize the vectors [1, 1, 0], [1, 0, 1] and [0, 1, 1]."],
     examples=[{"inputs": {"vectors": "3 4; 1 0"}, "outputs": {"orthonormal": [[0.6, 0.8], [0.8, -0.6]]}},
               {"inputs": {"vectors": [[1, 0, 0], [1, 1, 0]]}, "outputs": {"orthonormal": [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]]}}],
     questions=[{"q": "Apply Gram-Schmidt to the vectors (3, 4) and (1, 0).", "outputs": {"orthonormal": [[0.6, 0.8], [0.8, -0.6]]}},
                {"q": "Apply Gram-Schmidt to the vectors (1, 2) and (2, 4).", "outcome": "INVALID_INPUT"}])


def cholesky(A):
    A = _need_symmetric(A)
    n = len(A)
    F = _FM(A)
    for k in range(1, n + 1):                            # exact test: every leading principal minor positive
        if _det_F([r[:k] for r in F[:k]]) <= 0:
            raise ValueError("A is not positive definite (a leading principal minor is not positive)")
    L = [[0.0] * n for _ in range(n)]
    for j in range(n):
        s = float(A[j][j]) - math.fsum(L[j][k] ** 2 for k in range(j))
        L[j][j] = math.sqrt(s)
        for i in range(j + 1, n):
            L[i][j] = (float(A[i][j]) - math.fsum(L[i][k] * L[j][k] for k in range(j))) / L[j][j]
    return {"L": L}


def _verify_chol(i, o):
    A, L = i["A"], o["L"]
    n = len(A)
    if _shape(L) != (n, n) or any(L[r][c] != 0 for r in range(n) for c in range(r + 1, n)) or any(L[r][r] <= 0 for r in range(n)):
        return BAD
    LLt = _mm(L, _T(L))
    return max(abs(a - b) for r, t in zip(LLt, A) for a, b in zip(r, t)) / max(1.0, _fro(A))


def _random_spd(rng, n=None):
    n = n or rng.randint(2, 5)
    B = [[rng.randint(-4, 4) for _ in range(n)] for _ in range(n)]
    return [[sum(B[k][i] * B[k][j] for k in range(n)) + (n if i == j else 0) for j in range(n)] for i in range(n)]


_add("cholesky",
     name="Cholesky decomposition", topic="decompositions / Cholesky",
     purpose="Factor a symmetric positive definite matrix as A = L L^T with L lower triangular, positive diagonal.",
     inputs=[_I("A", "matrix", "a symmetric positive definite matrix")], outputs=[_O("L", "lower triangular factor with positive diagonal")],
     primary="L", conditions=["A is symmetric", "A is positive definite (checked exactly by its leading minors)", "at most 10 x 10"],
     method="the Cholesky-Banachiewicz recursion l_jj = sqrt(a_jj - sum l_jk^2), l_ij = (a_ij - sum l_ik l_jk)/l_jj",
     latex=r"A = LL^{T}", run=cholesky, verify=_verify_chol,
     reference=lambda A: {"L": __import__("numpy").linalg.cholesky(__import__("numpy").asarray(A, float)).tolist()},
     wrong=lambda A: {"L": _T(cholesky(A)["L"])},
     random_inputs=lambda rng: {"A": _random_spd(rng)},
     # 'Cholesky', or the factor described by its defining properties: lower triangular with a positive diagonal, of a
     # positive definite matrix (unique). Not an LU factor (unit diagonal, pivoting), not an upper factor.
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and (
         q.has(r"\bcholesky\b") or (q.has(r"\blower[- ]triangular\b") and q.has(r"\bpositive diagonal\b", r"\bpositive diagonal entries\b")
                                    and q.has(r"\bpositive definite\b")))
         and not q.has(r"\blu\b", r"\bunit diagonal\b", r"\bpivot", r"\bupper[- ]triangular\b", r"\bsquare root\b",
                       r"\bsolutions?\b", r"\bsolve (?:the |a )?(?:system|equation)", r"\bax\s*=\s*b\b",r"\binverse\b", r"\bdeterminant\b") else None),
     templates=["Find the Cholesky decomposition of [[4, 2], [2, 3]].", "Compute the Cholesky factor of the matrix [4 12 -16; 12 37 -43; -16 -43 98]."],
     examples=[{"inputs": {"A": "[[4, 2], [2, 3]]"}, "outputs": {"L": [[2.0, 0.0], [1.0, math.sqrt(2)]]}},
               {"inputs": {"A": [[4, 12, -16], [12, 37, -43], [-16, -43, 98]]}, "outputs": {"L": [[2.0, 0.0, 0.0], [6.0, 1.0, 0.0], [-8.0, 5.0, 3.0]]}}],
     questions=[{"q": "Compute the Cholesky factor of the matrix [4 12 -16; 12 37 -43; -16 -43 98].", "outputs": {"L": [[2.0, 0.0, 0.0], [6.0, 1.0, 0.0], [-8.0, 5.0, 3.0]]}},
                {"q": "Find the Cholesky decomposition of [[1, 2], [2, 1]].", "outcome": "INVALID_INPUT"},
                {"q": "Find the Cholesky decomposition of [[4, 1], [2, 3]].", "outcome": "INVALID_INPUT"}])


def positive_definiteness(A):
    A = _need_symmetric(A)
    F = _FM(A)
    minors = [_det_F([r[:k] for r in F[:k]]) for k in range(1, len(A) + 1)]
    return {"positive_definite": all(m > 0 for m in minors), "leading_minors": [float(m) for m in minors]}


# words that name another task done ON a positive definite matrix (a factor, a solution, a square root, eigenvalues ...):
# "symmetric positive definite" then only describes the input, it is not the question
_NOT_DEFINITENESS = (r"\bfactor", r"\bdecompos", r"\bdefinite (?:component|part)\b", r"\bpart\b", r"\bcholesky\b", r"\btriangular\b",
                     r"\bsquare root\b", r"\bsquared?\b", r"\bsqrt", r"\^\s*2\b", r"\bsolve\b", r"\bsolutions?\b",
                     r"\bright-hand\b", r"\bgradient", r"\biterat", r"\beigen", r"\binverse\b", r"\bpolar\b",
                     r"\bexponential\b", r"\bnorms?\b", r"\brank\b", r"\bcondition number\b", r"\bsingular values?\b")


def _parse_definiteness(q):
    """'Is A positive definite?', 'Check whether / Determine if A is positive definite', 'positive definiteness',
    'leading principal minors', 'Sylvester's criterion' — the definiteness test is the question asked."""
    if len(q.mats) != 1 or q.vecs or q.has(*_NOT_DEFINITENESS):
        return None
    asked = (re.search(r"\bpositive definiteness\b|\bleading principal minors\b|\bsylvester", q.rest)
             or re.search(r"\b(?:whether|if)\b[^.;?]*\bpositive definite\b", q.rest)
             or re.search(r"(?:^|[.;?!]\s*)(?:is|are|does|do)\b[^.;?!]*\bpositive definite\b[^.;!]*\?", q.rest)
             or re.search(r"\bbeing positive definite\b", q.rest))
    return {"A": q.mats[0]["text"]} if asked else None


def _random_pd_case(rng):
    n = rng.randint(2, 4)
    A = [[0] * n for _ in range(n)]
    for i in range(n):
        A[i][i] = rng.randint(1, 9)
        for j in range(i):
            A[i][j] = A[j][i] = rng.randint(-9, 9)
    return {"A": A}


_add("positive_definiteness",
     name="Positive definiteness test (Sylvester's criterion)", topic="eigenvalues / definiteness",
     purpose="Whether a symmetric matrix is positive definite, with its leading principal minors.",
     inputs=[_I("A", "matrix", "a symmetric matrix")],
     outputs=[_O("positive_definite", "true when x^T A x > 0 for every x != 0", "text"), _O("leading_minors", "the leading principal minors D1, ..., Dn")],
     primary="positive_definite", conditions=["A is symmetric", "at most 10 x 10"],
     method="Sylvester's criterion: positive definite exactly when every leading principal minor is positive (exact determinants)",
     latex=r"A \succ 0 \iff D_k = \det A_{[1..k],[1..k]} > 0,\ k = 1..n",
     run=positive_definiteness, verify=_recompute(positive_definiteness, ["positive_definite", "leading_minors"]),
     reference=lambda A: (lambda M: {"positive_definite": bool(M.is_positive_definite),
                                     "leading_minors": [float(M[:k, :k].det()) for k in range(1, M.rows + 1)]})(_sympy_matrix(A)),
     wrong=lambda A: dict(positive_definiteness(A), positive_definite=all(A[i][i] > 0 for i in range(len(A)))),
     random_inputs=_random_pd_case,
     parse=_safe_parse(_parse_definiteness),
     templates=["Is the matrix [[2, -1], [-1, 2]] positive definite?", "Is [1 2; 2 1] positive definite?"],
     examples=[{"inputs": {"A": "[[2, -1], [-1, 2]]"}, "outputs": {"positive_definite": True, "leading_minors": [2.0, 3.0]}},
               {"inputs": {"A": [[1, 2], [2, 1]]}, "outputs": {"positive_definite": False, "leading_minors": [1.0, -3.0]}}],
     questions=[{"q": "Is the matrix [[2, -1], [-1, 2]] positive definite?", "outputs": {"positive_definite": True}},
                {"q": "Is [1 2; 2 1] positive definite?", "outputs": {"positive_definite": False}},
                {"q": "Is the matrix [[2, 1], [0, 2]] positive definite?", "outcome": "INVALID_INPUT"}])


# --- eigenvalues: symmetric (Jacobi), general (QR algorithm), power and inverse iteration -------------------------
def _power_sums_exact(A, kmax):
    F = _FM(A)
    P, out = [r[:] for r in F], []
    for _ in range(kmax):
        out.append(float(sum((P[i][i] for i in range(len(F))), Fraction(0))))
        P = _mm_F(P, F)
    return out


def _verify_spectrum(A, vals):
    """sum lambda_i^k = tr(A^k) for k = 1..n (exact traces): the multiset of eigenvalues is determined by these."""
    n = len(A)
    if len(vals) != n:
        return BAD
    p = _power_sums_exact(A, n)
    sc = max(1.0, _fro(A))
    return max(abs(sum(complex(x) ** k for x in vals) - p[k - 1]) / sc ** k for k in range(1, n + 1))


def symmetric_eigenvalues(A):
    A = _need_symmetric(A)
    return {"eigenvalues": sorted(_jacobi_eig(A)[0])}


_add("symmetric_eigenvalues",
     name="Eigenvalues of a symmetric matrix (Jacobi method)", topic="eigenvalues / symmetric matrices",
     purpose="All eigenvalues of a real symmetric matrix (they are real), in ascending order.",
     inputs=[_I("A", "matrix", "a symmetric matrix (up to 10 x 10)")], outputs=[_O("eigenvalues", "eigenvalues, ascending")], primary="eigenvalues",
     conditions=["A is symmetric", "at most 10 x 10"], method="cyclic Jacobi rotations until the off-diagonal part vanishes",
     latex=r"A = Q\Lambda Q^{T}", run=symmetric_eigenvalues,
     verify=lambda i, o: max(_verify_spectrum(i["A"], o["eigenvalues"]), 0.0 if o["eigenvalues"] == sorted(o["eigenvalues"]) else BAD),
     reference=lambda A: {"eigenvalues": __import__("numpy").linalg.eigvalsh(__import__("numpy").asarray(A, float)).tolist()},
     wrong=lambda A: {"eigenvalues": sorted(float(A[i][i]) for i in range(len(A)))},
     random_inputs=lambda rng: {"A": _random_sym(rng, rng.randint(3, 6))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\beigenvalues?\b")
                       # 2 x 2 belongs to eigenvalues_2x2 unless the ascending list (this capability's output) is asked
                       and (lambda o: _dims(o) is not None and _is_square(o) and (_dims(o)[0] >= 3 or q.has(*_LIST_ORDER)) and all(
                           o["value"][i][j] == o["value"][j][i] for i in range(len(o["value"])) for j in range(i)))(q.mats[0])
                       and not q.has(r"\beigenvectors?\b", r"\bdominant\b", r"\blargest\b", r"\bsmallest\b", r"\bpower\b", r"\biteration\b",
                                     r"\bspectral radius\b", r"\bsingular\b", r"\bcharacteristic polynomial\b", *_ASKS_MODULUS,
                                     *_LIST_DESC) else None),
     templates=["Find the eigenvalues of [[2, 1, 0], [1, 2, 1], [0, 1, 2]].", "What are the eigenvalues of the symmetric matrix [4 1 0 0; 1 4 1 0; 0 1 4 1; 0 0 1 4]?"],
     examples=[{"inputs": {"A": "[[2, 1, 0], [1, 2, 1], [0, 1, 2]]"}, "outputs": {"eigenvalues": [2 - math.sqrt(2), 2.0, 2 + math.sqrt(2)]}},
               {"inputs": {"A": [[2, 0, 0], [0, 3, 4], [0, 4, 9]]}, "outputs": {"eigenvalues": [1.0, 2.0, 11.0]}}],
     questions=[{"q": "Find the eigenvalues of [[2, 1, 0], [1, 2, 1], [0, 1, 2]].", "outputs": {"eigenvalues": [2 - math.sqrt(2), 2.0, 2 + math.sqrt(2)]}}])


def symmetric_eigenvectors(A):
    A = _need_symmetric(A)
    vals, V = _jacobi_eig(A)
    order = sorted(range(len(vals)), key=lambda k: vals[k])
    vals = [vals[k] for k in order]
    sc = max(1.0, max(abs(x) for x in vals))
    if any(vals[k + 1] - vals[k] <= 1e-8 * sc for k in range(len(vals) - 1)):
        raise ValueError("A has a repeated eigenvalue, so its eigenvectors are not determined up to sign")
    vecs = []
    for k in order:
        v = [V[r][k] for r in range(len(A))]
        nv = _norm(v)
        vecs.append(_sign_fix([x / nv for x in v]))
    return {"eigenvalues": vals, "eigenvectors": vecs}


def _verify_eigvecs(i, o):
    A, lam, V = i["A"], o["eigenvalues"], o["eigenvectors"]
    n = len(A)
    if len(lam) != n or _shape(V) != (n, n) or lam != sorted(lam) or not all(_sign_ok(v) for v in V):
        return BAD
    sc = max(1.0, _fro(A))
    e1 = max(_norm([a - l * b for a, b in zip(_mv(A, v), v)]) for l, v in zip(lam, V)) / sc
    e2 = max(abs(a - b) for r, t in zip(_mm(V, _T(V)), _eye(n)) for a, b in zip(r, t))
    return max(e1, e2, _verify_spectrum(A, lam))


def _reference_eigvecs(A):
    import numpy as np
    w, V = np.linalg.eigh(np.asarray(A, float))
    return {"eigenvalues": w.tolist(), "eigenvectors": [_sign_fix(list(V[:, k])) for k in range(len(w))]}


def _random_sym(rng, n):
    M = _rmat(rng, n)
    return [[M[min(i, j)][max(i, j)] for j in range(n)] for i in range(n)]


_add("symmetric_eigenvectors",
     name="Eigenvalues and eigenvectors of a symmetric matrix", topic="eigenvalues / symmetric matrices",
     purpose="The orthonormal eigenvectors of a real symmetric matrix with distinct eigenvalues (spectral decomposition).",
     inputs=[_I("A", "matrix", "a symmetric matrix with distinct eigenvalues")],
     outputs=[_O("eigenvalues", "eigenvalues, ascending"), _O("eigenvectors", "unit eigenvectors in the same order, one per row")],
     primary="eigenvectors", conditions=["A is symmetric", "the eigenvalues are distinct", "at most 10 x 10",
                                         "sign convention: the first entry of largest magnitude of each eigenvector is positive"],
     method="cyclic Jacobi rotations; the accumulated rotations are the eigenvectors", latex=r"A\mathbf{q}_i = \lambda_i \mathbf{q}_i,\ Q^{T}Q = I",
     run=symmetric_eigenvectors, verify=_verify_eigvecs, reference=_reference_eigvecs,
     wrong=lambda A: (lambda r: dict(r, eigenvectors=[_sign_fix(v) for v in _T(r["eigenvectors"])]))(symmetric_eigenvectors(A)),
     random_inputs=lambda rng: {"A": _random_sym(rng, rng.randint(2, 5))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\beigenvectors?\b", r"\bspectral decomposition\b",
                                                                                                     r"\borthogonally diagonali[sz]e")
                       and not q.has(r"\bdominant\b", r"\bpower\b", r"\bsmallest\b", r"\blargest\b", r"\biteration\b", *_LIST_DESC) else None),
     templates=["Find the eigenvectors of [[2, 1], [1, 2]].", "Find the eigenvalues and eigenvectors of the symmetric matrix [2 0 0; 0 3 4; 0 4 9]."],
     examples=[{"inputs": {"A": "[[2, 1], [1, 2]]"}, "outputs": {"eigenvalues": [1.0, 3.0],
                                                                 "eigenvectors": [[1 / math.sqrt(2), -1 / math.sqrt(2)], [1 / math.sqrt(2), 1 / math.sqrt(2)]]}},
               {"inputs": {"A": [[2, 0, 0], [0, 3, 4], [0, 4, 9]]}, "outputs": {"eigenvalues": [1.0, 2.0, 11.0],
                "eigenvectors": [[0.0, 2 / math.sqrt(5), -1 / math.sqrt(5)], [1.0, 0.0, 0.0], [0.0, 1 / math.sqrt(5), 2 / math.sqrt(5)]]}}],
     questions=[{"q": "Find the eigenvectors of [[2, 1], [1, 2]].", "outputs": {"eigenvalues": [1.0, 3.0]}},
                {"q": "Find the eigenvectors of [[1, 0], [0, 1]].", "outcome": "INVALID_INPUT"},
                {"q": "Find the eigenvectors of [[1, 2], [3, 4]].", "outcome": "INVALID_INPUT"}])


def _hessenberg(A):
    n = len(A)
    H = [[float(x) for x in r] for r in A]
    for k in range(n - 2):
        x = [H[i][k] for i in range(k + 1, n)]
        nx = _norm(x)
        if nx == 0:
            continue
        alpha = -math.copysign(nx, x[0])
        v = x[:]
        v[0] -= alpha
        nv = _norm(v)
        if nv == 0:
            continue
        v = [t / nv for t in v]
        m = len(v)
        for j in range(n):
            d = 2 * math.fsum(v[i] * H[k + 1 + i][j] for i in range(m))
            for i in range(m):
                H[k + 1 + i][j] -= d * v[i]
        for i in range(n):
            d = 2 * math.fsum(H[i][k + 1 + j] * v[j] for j in range(m))
            for j in range(m):
                H[i][k + 1 + j] -= d * v[j]
        for i in range(k + 2, n):
            H[i][k] = 0.0
    return H


def _eig_qr(A):
    """Eigenvalues of a real square matrix: Hessenberg reduction, then the shifted QR algorithm in complex arithmetic
    (Givens rotations, Wilkinson shift, deflation); conjugate pairs are matched exactly at the end."""
    n = len(A)
    H = [[complex(x) for x in r] for r in _hessenberg(A)]
    eps = 2.2e-16
    out, hi, it, total = [], n - 1, 0, 0
    while hi >= 0:
        if hi == 0:
            out.append(H[0][0])
            break
        l = hi
        while l > 0 and abs(H[l][l - 1]) > eps * (abs(H[l][l]) + abs(H[l - 1][l - 1])) and abs(H[l][l - 1]) > 1e-300:
            l -= 1
        if l == hi:
            out.append(H[hi][hi])
            hi, it = hi - 1, 0
            continue
        a, b, c, d = H[hi - 1][hi - 1], H[hi - 1][hi], H[hi][hi - 1], H[hi][hi]
        tr, det = a + d, a * d - b * c
        disc = cmath.sqrt(tr * tr / 4 - det)
        mu = min((tr / 2 + disc, tr / 2 - disc), key=lambda z: abs(z - d))
        if it and it % 11 == 0:
            mu = d + abs(c)                                   # exceptional shift
        for k in range(l, hi + 1):
            H[k][k] -= mu
        rots = []
        for k in range(l, hi):
            x, y = H[k][k], H[k + 1][k]
            r = math.hypot(abs(x), abs(y))
            cc, ss = (x / r, y / r) if r else (1.0 + 0j, 0j)
            for j in range(k, hi + 1):
                t1, t2 = H[k][j], H[k + 1][j]
                H[k][j] = cc.conjugate() * t1 + ss.conjugate() * t2
                H[k + 1][j] = -ss * t1 + cc * t2
            rots.append((cc, ss))
        for k, (cc, ss) in enumerate(rots, start=l):
            for i in range(l, min(k + 2, hi) + 1):
                t1, t2 = H[i][k], H[i][k + 1]
                H[i][k] = t1 * cc + t2 * ss
                H[i][k + 1] = -t1 * ss.conjugate() + t2 * cc.conjugate()
        for k in range(l, hi + 1):
            H[k][k] += mu
        it, total = it + 1, total + 1
        if total > 200 * n:
            raise ValueError("the QR algorithm did not converge")
    return _real_spectrum(out, _fro(A))


def _real_spectrum(vals, fro):
    """For a real matrix: near-real eigenvalues made real, complex ones paired with their conjugates; canonical order."""
    tol = 1e-11 * max(1.0, fro)
    real = [complex(z).real for z in vals if abs(complex(z).imag) <= tol]
    pos = sorted((complex(z) for z in vals if complex(z).imag > tol), key=lambda z: (z.real, z.imag))
    neg = [complex(z) for z in vals if complex(z).imag < -tol]
    if len(pos) != len(neg):
        raise ValueError("the eigenvalues could not be separated into conjugate pairs")
    out = sorted(real)
    pairs = []
    for z in pos:
        w = min(neg, key=lambda u: abs(u.conjugate() - z))
        neg.remove(w)
        m = (z + w.conjugate()) / 2
        pairs += [m, m.conjugate()]
    pairs = sorted(pairs, key=lambda z: (z.real, -z.imag))
    return out + pairs


def general_eigenvalues(A):
    A = _need_mat(A, "A", square=True)
    ev = _eig_qr(A)
    return {"eigenvalues": [complex(z) if isinstance(z, complex) else float(z) for z in ev],
            "spectral_radius": max(abs(complex(z)) for z in ev)}


def _verify_general_eig(i, o):
    A, ev = i["A"], o["eigenvalues"]
    if len(ev) != len(A):
        return BAD
    try:
        canon = _real_spectrum(ev, _fro(A))
    except ValueError:
        return BAD
    if any(complex(a) != complex(b) for a, b in zip(canon, ev)):
        return BAD                                         # not in the canonical order / not exact conjugate pairs
    return max(_verify_spectrum(A, ev), _rel(o["spectral_radius"], max(abs(complex(z)) for z in ev)))


def _reference_general_eig(A):
    import numpy as np
    M = np.asarray(A, float)
    w = list(np.linalg.eigvals(M))
    v = _real_spectrum(w, float(np.linalg.norm(M)))
    return {"eigenvalues": v, "spectral_radius": float(max(abs(z) for z in w))}


def _random_general_eig(rng):
    while True:
        n = rng.randint(3, 5)
        A = _rmat(rng, n)
        try:
            ev = _eig_qr(A)
        except ValueError:
            continue
        sc = max(1.0, _fro(A))
        if all(abs(complex(a) - complex(b)) > 1e-3 * sc for k, a in enumerate(ev) for b in ev[k + 1:]):
            return {"A": A}                                  # well-separated eigenvalues (well conditioned)


def _parse_general_eig(q):
    if len(q.mats) != 1 or q.vecs or q.has(*_LIST_DESC):
        return None
    o = q.mats[0]
    if q.has(r"\bspectral radius\b") or (q.has(*_ASKS_MODULUS) and not q.has(r"\beigenvectors?\b", r"\bpower\b", r"\biteration\b",
                                                                             r"\bsingular\b")):
        return {"A": o["text"]}                              # the largest |eigenvalue| is the spectral radius
    if not q.has(r"\beigenvalues?\b") or q.has(r"\beigenvectors?\b", r"\bdominant\b", r"\blargest\b", r"\bsmallest\b", r"\bpower\b",
                                                r"\biteration\b", r"\bsingular\b", r"\bcharacteristic\b"):
        return None
    d = _dims(o)
    if d is None:
        return {"A": o["text"]}
    if d[0] != d[1]:
        return {"A": o["text"]}                              # not square: refused with the reason
    if d[0] <= 2 and not q.has(*_LIST_ORDER):               # 2 x 2: eigenvalues_2x2, unless the ordered list is asked
        return None
    sym = all(o["value"][r][c] == o["value"][c][r] for r in range(d[0]) for c in range(r))
    return None if sym else {"A": o["text"]}


_add("general_eigenvalues",
     name="Eigenvalues of a general square matrix (QR algorithm)", topic="eigenvalues / general matrices",
     purpose="All eigenvalues (real or complex) of a real square matrix, and its spectral radius.",
     inputs=[_I("A", "matrix", "a square matrix (up to 10 x 10)")],
     outputs=[_O("eigenvalues", "eigenvalues: real ones ascending, then complex pairs by real part, positive imaginary part first"),
              _O("spectral_radius", "largest |eigenvalue|")], primary="spectral_radius",
     conditions=["A is square", "at most 10 x 10"],
     method="Householder reduction to Hessenberg form, then the shifted QR algorithm (Wilkinson shifts, Givens rotations, deflation)",
     latex=r"\det(A - \lambda I) = 0,\quad \rho(A) = \max_i |\lambda_i|",
     run=general_eigenvalues, verify=_verify_general_eig, reference=_reference_general_eig,
     wrong=lambda A: (lambda d: {"eigenvalues": d, "spectral_radius": max(abs(x) for x in d)})(sorted(float(A[i][i]) for i in range(len(A)))),
     random_inputs=_random_general_eig, parse=_safe_parse(_parse_general_eig),
     templates=["Find the eigenvalues of [[1, 2, 0], [0, 3, 0], [2, -4, 2]].", "What is the spectral radius of [[0, -1, 0], [1, 0, 0], [0, 0, 0.5]]?"],
     examples=[{"inputs": {"A": "[[1, 2, 0], [0, 3, 0], [2, -4, 2]]"}, "outputs": {"eigenvalues": [1.0, 2.0, 3.0], "spectral_radius": 3.0}},
               {"inputs": {"A": [[0, -1, 0], [1, 0, 0], [0, 0, 0.5]]}, "outputs": {"eigenvalues": [0.5, complex(0, 1), complex(0, -1)], "spectral_radius": 1.0}}],
     questions=[{"q": "Find the eigenvalues of [[1, 2, 0], [0, 3, 0], [2, -4, 2]].", "outputs": {"eigenvalues": [1.0, 2.0, 3.0]}},
                {"q": "What is the spectral radius of [[0, -1, 0], [1, 0, 0], [0, 0, 0.5]]?", "outputs": {"spectral_radius": 1.0}},
                {"q": "Find the eigenvalues of [[1, 2, 3], [4, 5, 6]].", "outcome": "INVALID_INPUT"}])


def _lu_solver(M):
    """A solve function x = M^-1 y from one LU factorization with partial pivoting (floats)."""
    n = len(M)
    U = [[float(x) for x in r] for r in M]
    perm = list(range(n))
    L = [[0.0] * n for _ in range(n)]
    for k in range(n):
        p = max(range(k, n), key=lambda i: abs(U[i][k]))
        if U[p][k] == 0:
            U[p][k] = 1e-300 + 1e-15 * max(1.0, _fro(M))           # an exact eigenvalue as shift: perturb the pivot
        U[k], U[p], L[k], L[p], perm[k], perm[p] = U[p], U[k], L[p], L[k], perm[p], perm[k]
        for i in range(k + 1, n):
            f = U[i][k] / U[k][k]
            L[i][k] = f
            U[i] = [a - f * b for a, b in zip(U[i], U[k])]

    def solve(y):
        z = [y[p] for p in perm]
        for i in range(n):
            z[i] -= math.fsum(L[i][k] * z[k] for k in range(i))
        for i in reversed(range(n)):
            z[i] = (z[i] - math.fsum(U[i][k] * z[k] for k in range(i + 1, n))) / U[i][i]
        return z
    return solve


def _unit(v):
    nv = _norm(v)
    return _sign_fix([x / nv for x in v])


def _refine(A, lam, v, steps=4):
    """Shifted inverse iteration at the converged estimate: the eigenvector to machine precision."""
    n = len(A)
    solve = _lu_solver([[A[i][j] - (lam if i == j else 0.0) for j in range(n)] for i in range(n)])
    for _ in range(steps):
        v = _unit(solve(v))
    Av = _mv(A, v)
    return _dot(v, Av), v


def _iterate(A, step, extreme):
    """Power iteration with the map `step` (A x, or A^-1 x); refined, then checked against the QR spectrum."""
    n = len(A)
    ev = _eig_qr(A)
    mods = sorted(abs(complex(z)) for z in ev)
    target = (max if extreme == "max" else min)(ev, key=lambda z: abs(complex(z)))
    second = mods[-2] if extreme == "max" else mods[1]
    lead = mods[-1] if extreme == "max" else mods[0]
    sc = max(1.0, _fro(A))
    if isinstance(target, complex) or (abs(lead - second) <= 1e-6 * sc):
        raise ValueError("there is no single real eigenvalue of " + ("largest" if extreme == "max" else "smallest")
                         + " magnitude, so the iteration does not converge")
    for start in ([1.0 + k / (2.0 * n) for k in range(n)], [(-1.0) ** k * (k + 1) for k in range(n)], [1.0] + [0.0] * (n - 1)):
        x = _unit(start)
        for _ in range(20000):
            y = _unit(step(x))
            if _norm([a - b for a, b in zip(x, y)]) < 1e-10:
                x = y
                break
            x = y
        lam = _dot(x, _mv(A, x)) / _dot(x, x)
        lam, x = _refine(A, lam, x)
        if abs(lam - float(target)) <= 1e-8 * sc and _norm([a - lam * b for a, b in zip(_mv(A, x), x)]) <= 1e-12 * sc:
            return {"eigenvalue": lam, "eigenvector": x}
    raise ValueError("the iteration did not converge to the extreme eigenvalue")


def power_iteration(A):
    A = _need_mat(A, "A", square=True)
    return _iterate(A, lambda x: _mv(A, x), "max")


def inverse_iteration(A):
    A = _need_mat(A, "A", square=True)
    if _det_F(_FM(A)) == 0:
        raise ValueError("A is singular: its smallest eigenvalue is 0 and A cannot be inverted")
    solve = _lu_solver(A)
    return _iterate(A, solve, "min")


def _verify_iter(extreme):
    def v(i, o):
        A, lam, x = i["A"], o["eigenvalue"], o["eigenvector"]
        if len(x) != len(A) or not _sign_ok(x) or abs(_norm(x) - 1) > 1e-12:
            return BAD
        sc = max(1.0, _fro(A))
        res = _norm([a - lam * b for a, b in zip(_mv(A, x), x)]) / sc
        mods = [abs(complex(z)) for z in _eig_qr(A)]
        edge = max(mods) if extreme == "max" else min(mods)
        return max(res, abs(abs(lam) - edge) / sc)
    return v


def _reference_iter(A, extreme):
    import numpy as np
    w, V = np.linalg.eig(np.asarray(A, float))
    k = int(np.argmax(abs(w))) if extreme == "max" else int(np.argmin(abs(w)))
    v = np.real(V[:, k])
    return {"eigenvalue": float(np.real(w[k])), "eigenvector": _sign_fix(list(v / np.linalg.norm(v)))}


def _wrong_iter(A, step):
    x = _unit([1.0] * len(A))
    for _ in range(3):
        x = _unit(step(x))
    return {"eigenvalue": _dot(x, _mv(A, x)), "eigenvector": x}


def _random_positive(rng):
    n = rng.randint(2, 5)
    return {"A": [[rng.randint(1, 9) for _ in range(n)] for _ in range(n)]} if rng.random() < 0.5 else {"A": _random_sym(rng, n)}


_add("power_iteration",
     name="Dominant eigenvalue by power iteration", topic="eigenvalues / iterative methods",
     purpose="The eigenvalue of largest magnitude and its unit eigenvector, by the power method.",
     inputs=[_I("A", "matrix", "a square matrix with a single real dominant eigenvalue")],
     outputs=[_O("eigenvalue", "the dominant eigenvalue"), _O("eigenvector", "its unit eigenvector (first largest entry positive)")],
     primary="eigenvalue", conditions=["A is square, at most 10 x 10", "one real eigenvalue is strictly largest in magnitude"],
     method="power iteration x <- A x / |A x| to convergence, polished by two steps of shifted inverse iteration; the dominance is checked against the QR-algorithm spectrum",
     latex=r"\mathbf{x}_{k+1} = \frac{A\mathbf{x}_k}{\|A\mathbf{x}_k\|},\quad \lambda \approx \mathbf{x}_k^{T}A\mathbf{x}_k",
     run=power_iteration, verify=_verify_iter("max"), reference=lambda A: _reference_iter(A, "max"),
     wrong=lambda A: _wrong_iter(A, lambda x: _mv(A, x)), random_inputs=_random_positive,
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\bpower (?:method|iteration)\b", r"\bdominant eigen", r"\blargest eigenvalue\b")
                       and not q.has(r"\binverse\b", r"\bspectral radius\b", *_ASKS_MODULUS) else None),
     templates=["Use power iteration to find the dominant eigenvalue of [[2, 1], [1, 3]].", "Find the dominant eigenvalue of [4 1; 2 3] by the power method."],
     examples=[{"inputs": {"A": "[[4, 1], [2, 3]]"}, "outputs": {"eigenvalue": 5.0, "eigenvector": [1 / math.sqrt(2), 1 / math.sqrt(2)]}},
               {"inputs": {"A": [[2, 0, 0], [0, -5, 0], [0, 0, 1]]}, "outputs": {"eigenvalue": -5.0, "eigenvector": [0.0, 1.0, 0.0]}}],
     questions=[{"q": "Find the dominant eigenvalue of [4 1; 2 3] by the power method.", "outputs": {"eigenvalue": 5.0}},
                {"q": "Use power iteration to find the dominant eigenvalue of [[0, 1], [-1, 0]].", "outcome": "INVALID_INPUT"}])

_add("inverse_iteration",
     name="Smallest eigenvalue by inverse iteration", topic="eigenvalues / iterative methods",
     purpose="The eigenvalue of smallest magnitude and its unit eigenvector, by inverse power iteration.",
     inputs=[_I("A", "matrix", "a nonsingular square matrix with a single real eigenvalue of smallest magnitude")],
     outputs=[_O("eigenvalue", "the eigenvalue of smallest magnitude"), _O("eigenvector", "its unit eigenvector (first largest entry positive)")],
     primary="eigenvalue", conditions=["A is square and nonsingular, at most 10 x 10", "one real eigenvalue is strictly smallest in magnitude"],
     method="power iteration with A^-1 (one LU factorization, a solve per step), polished by shifted inverse iteration; checked against the QR-algorithm spectrum",
     latex=r"A\mathbf{y}_{k+1} = \mathbf{x}_k,\quad \mathbf{x}_{k+1} = \mathbf{y}_{k+1}/\|\mathbf{y}_{k+1}\|",
     run=inverse_iteration, verify=_verify_iter("min"), reference=lambda A: _reference_iter(A, "min"),
     wrong=lambda A: _wrong_iter(A, _lu_solver(A)), random_inputs=_random_positive,
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs
                       and q.has(r"\binverse (?:power )?(?:method|iteration)\b", r"\bsmallest eigenvalue\b") else None),
     templates=["Use inverse iteration to find the smallest eigenvalue of [[2, 1], [1, 3]].", "Find the smallest eigenvalue of [4 1; 2 3] by inverse power iteration."],
     examples=[{"inputs": {"A": "[[4, 1], [2, 3]]"}, "outputs": {"eigenvalue": 2.0, "eigenvector": [-1 / math.sqrt(5), 2 / math.sqrt(5)]}},
               {"inputs": {"A": [[2, 0, 0], [0, -5, 0], [0, 0, 1]]}, "outputs": {"eigenvalue": 1.0, "eigenvector": [0.0, 0.0, 1.0]}}],
     questions=[{"q": "Find the smallest eigenvalue of [4 1; 2 3] by inverse power iteration.", "outputs": {"eigenvalue": 2.0}},
                {"q": "Use inverse iteration to find the smallest eigenvalue of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


# --- matrix functions -----------------------------------------------------------------------------------------------
def _expm(A, extra=0):
    n = len(A)
    norm1 = max(math.fsum(abs(A[i][j]) for i in range(n)) for j in range(n))
    s = max(0, math.ceil(math.log2(norm1 / 0.5))) + extra if norm1 > 0.5 else extra
    B = [[float(x) / 2 ** s for x in r] for r in A]
    E, T = _eye(n), _eye(n)
    for k in range(1, 40):
        T = [[x / k for x in r] for r in _mm(T, B)]
        E = [[a + b for a, b in zip(r, t)] for r, t in zip(E, T)]
        if max(abs(x) for r in T for x in r) <= 1e-18 * max(abs(x) for r in E for x in r):
            break
    for _ in range(s):
        E = _mm(E, E)
    return E


def matrix_exponential(A):
    A = _need_mat(A, "A", square=True)
    norm1 = max(math.fsum(abs(A[i][j]) for i in range(len(A))) for j in range(len(A)))
    if norm1 > 50:
        raise ValueError("this version computes exp(A) for |A|_1 <= 50 (larger norms lose accuracy)")
    return {"exponential": _expm(A)}


def _verify_expm(i, o):
    """exp(A) commutes with A, and agrees with an independently scaled evaluation (two more halvings)."""
    A, E = i["A"], o["exponential"]
    if _shape(E) != _shape(A):
        return BAD
    E2 = _expm(A, extra=2)
    sc = max(1.0, max(abs(x) for r in E2 for x in r))
    e1 = max(abs(a - b) for r, t in zip(E, E2) for a, b in zip(r, t)) / sc
    AE, EA = _mm(A, E), _mm(E, A)
    e2 = max(abs(a - b) for r, t in zip(AE, EA) for a, b in zip(r, t)) / (sc * max(1.0, _fro(A)))
    return max(e1, e2)


_add("matrix_exponential",
     name="Matrix exponential", topic="matrix functions / exponential",
     purpose="exp(A) = I + A + A^2/2! + ..., the solution operator of x' = A x.",
     inputs=[_I("A", "matrix", "a square matrix with |A|_1 <= 50")], outputs=[_O("exponential", "the matrix exp(A)")], primary="exponential",
     conditions=["A is square, at most 10 x 10", "|A|_1 <= 50"],
     method="scaling and squaring: a Taylor series for exp(A / 2^s) with |A / 2^s|_1 <= 1/2, then s squarings",
     latex=r"e^{A} = \sum_{k=0}^{\infty} \frac{A^k}{k!} = \big(e^{A/2^s}\big)^{2^s}",
     run=matrix_exponential, verify=_verify_expm,
     reference=lambda A: {"exponential": __import__("scipy.linalg", fromlist=["expm"]).expm(__import__("numpy").asarray(A, float)).tolist()},
     wrong=lambda A: {"exponential": [[math.exp(x) for x in r] for r in A]},
     random_inputs=lambda rng: {"A": [[round(rng.uniform(-2, 2), 1) for _ in range(n)] for n in [rng.randint(2, 4)] for _ in range(n)]},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bmatrix exponential\b", r"\bexponential of\b",
                                                                                                     r"\bexp\s*\(", r"\be\^\s*a\b") else None),
     templates=["Find the matrix exponential of [[0, 1], [0, 0]].", "Compute the exponential of the matrix [0 -1; 1 0]."],
     examples=[{"inputs": {"A": "[[0, 1], [0, 0]]"}, "outputs": {"exponential": [[1.0, 1.0], [0.0, 1.0]]}},
               {"inputs": {"A": [[0, -1], [1, 0]]}, "outputs": {"exponential": [[math.cos(1), -math.sin(1)], [math.sin(1), math.cos(1)]]}},
               {"inputs": {"A": [[1, 0], [0, 2]]}, "outputs": {"exponential": [[math.e, 0.0], [0.0, math.e ** 2]]}}],
     questions=[{"q": "Find the matrix exponential of [[0, 1], [0, 0]].", "outputs": {"exponential": [[1.0, 1.0], [0.0, 1.0]]}},
                {"q": "Compute the exponential of the matrix [0 -1; 1 0].", "outputs": {"exponential": [[math.cos(1), -math.sin(1)], [math.sin(1), math.cos(1)]]}}])


def matrix_square_root(A):
    A = _need_symmetric(A)
    F = _FM(A)
    for k in range(1, len(A) + 1):
        if _det_F([r[:k] for r in F[:k]]) <= 0:
            raise ValueError("A must be positive definite (a leading principal minor is not positive)")
    vals, V = _jacobi_eig(A)
    n = len(A)
    X = [[math.fsum(V[i][k] * math.sqrt(vals[k]) * V[j][k] for k in range(n)) for j in range(n)] for i in range(n)]
    X = [[(X[i][j] + X[j][i]) / 2 for j in range(n)] for i in range(n)]
    return {"sqrt": X}


def _verify_sqrtm(i, o):
    """X symmetric, X^2 = A, X positive definite (Cholesky succeeds): the unique positive definite square root."""
    A, X = i["A"], o["sqrt"]
    n = len(A)
    if _shape(X) != (n, n) or any(X[r][c] != X[c][r] for r in range(n) for c in range(r)):
        return BAD
    L = [[0.0] * n for _ in range(n)]                         # Cholesky of X: fails unless X is positive definite
    for j in range(n):
        s = X[j][j] - math.fsum(L[j][k] ** 2 for k in range(j))
        if s <= 0:
            return BAD
        L[j][j] = math.sqrt(s)
        for r in range(j + 1, n):
            L[r][j] = (X[r][j] - math.fsum(L[r][k] * L[j][k] for k in range(j))) / L[j][j]
    XX = _mm(X, X)
    return max(abs(a - b) for r, t in zip(XX, A) for a, b in zip(r, t)) / max(1.0, _fro(A))


_add("matrix_square_root",
     name="Square root of a positive definite matrix", topic="matrix functions / square root",
     loose_ok=(r"\bx\s*(?:\^|\*\*)\s*2\b", r"(?:\^|\*\*)\s*\(\s*1\s*/\s*2\s*\)", r"(?:\^|\*\*)\s*0?\.5\b"),
     purpose="The unique symmetric positive definite X with X^2 = A, for a symmetric positive definite A.",
     inputs=[_I("A", "matrix", "a symmetric positive definite matrix")], outputs=[_O("sqrt", "the principal square root A^(1/2)")], primary="sqrt",
     conditions=["A is symmetric positive definite", "at most 10 x 10"],
     method="spectral decomposition by Jacobi rotations: A^(1/2) = Q diag(sqrt(lambda)) Q^T", latex=r"A^{1/2} = Q\,\Lambda^{1/2} Q^{T}",
     run=matrix_square_root, verify=_verify_sqrtm,
     reference=lambda A: {"sqrt": __import__("numpy").real(__import__("scipy.linalg", fromlist=["sqrtm"]).sqrtm(__import__("numpy").asarray(A, float))).tolist()},
     wrong=lambda A: {"sqrt": cholesky(A)["L"]},
     random_inputs=lambda rng: {"A": _random_spd(rng, rng.randint(2, 4))},
     # 'square root', or X described by X^2 = A: then X must be stated to be positive definite (or the principal root),
     # since a matrix has several square roots
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and (
         q.has(r"\bsquare root\b", r"\bsqrtm?\b")
         or (re.search(r"\bx\s*(?:squared|\^\s*2|\*\*\s*2)|\bwhose square\b", q.rest)
             and re.search(r"\b(?:positive definite|principal)\b[^@]*\bx\b|\bpositive definite matrix (?:x\b|whose square)|\bprincipal\b", q.rest)))
         and not q.has(r"\bcholesky\b", r"\btriangular\b", r"\beigen", r"\bdeterminant\b", r"\binverse\b") else None),
     templates=["Find the square root of the matrix [[5, 4], [4, 5]].", "Compute the principal square root of [2 0; 0 9]."],
     examples=[{"inputs": {"A": "[[5, 4], [4, 5]]"}, "outputs": {"sqrt": [[2.0, 1.0], [1.0, 2.0]]}},
               {"inputs": {"A": [[2, 0], [0, 9]]}, "outputs": {"sqrt": [[math.sqrt(2), 0.0], [0.0, 3.0]]}}],
     questions=[{"q": "Find the square root of the matrix [[5, 4], [4, 5]].", "outputs": {"sqrt": [[2.0, 1.0], [1.0, 2.0]]}},
                {"q": "Find the square root of the matrix [[1, 2], [2, 1]].", "outcome": "INVALID_INPUT"}])


def polar_decomposition(A):
    A = _need_mat(A, "A", square=True)
    if _det_F(_FM(A)) == 0:
        raise ValueError("A is singular: its polar factor is not unique")
    U, s, V = _svd_jacobi(A)                 # U, V by columns (lists of column vectors), A = sum s_k u_k v_k^T
    n = len(A)
    W = [[math.fsum(U[k][i] * V[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
    P = [[math.fsum(V[k][i] * s[k] * V[k][j] for k in range(n)) for j in range(n)] for i in range(n)]
    P = [[(P[i][j] + P[j][i]) / 2 for j in range(n)] for i in range(n)]
    return {"U": W, "P": P}


def _verify_polar(i, o):
    """U orthogonal, P symmetric positive definite, U P = A: unique for a nonsingular A."""
    A, U, P = i["A"], o["U"], o["P"]
    n = len(A)
    if _shape(U) != (n, n) or _shape(P) != (n, n) or any(P[r][c] != P[c][r] for r in range(n) for c in range(r)):
        return BAD
    L = [[0.0] * n for _ in range(n)]
    for j in range(n):
        s = P[j][j] - math.fsum(L[j][k] ** 2 for k in range(j))
        if s <= 0:
            return BAD
        L[j][j] = math.sqrt(s)
        for r in range(j + 1, n):
            L[r][j] = (P[r][j] - math.fsum(L[r][k] * L[j][k] for k in range(j))) / L[j][j]
    e1 = max(abs(a - b) for r, t in zip(_mm(_T(U), U), _eye(n)) for a, b in zip(r, t))
    e2 = max(abs(a - b) for r, t in zip(_mm(U, P), A) for a, b in zip(r, t)) / max(1.0, _fro(A))
    return max(e1, e2)


def _wrong_polar(A):
    r = polar_decomposition(A)
    n = len(A)
    left = [[math.fsum(r["U"][i][k] * r["P"][k][l] * r["U"][j][l] for k in range(n) for l in range(n)) for j in range(n)] for i in range(n)]
    return {"U": r["U"], "P": [[(left[i][j] + left[j][i]) / 2 for j in range(n)] for i in range(n)]}


_add("polar_decomposition",
     name="Polar decomposition", topic="decompositions / polar decomposition",
     purpose="Write a nonsingular square matrix as A = U P: U orthogonal (a rotation or reflection), P symmetric positive definite (a stretch).",
     inputs=[_I("A", "matrix", "a nonsingular square matrix")], outputs=[_O("U", "orthogonal factor"), _O("P", "symmetric positive definite factor (A^T A)^(1/2)")],
     primary="P", conditions=["A is square and nonsingular", "at most 10 x 10", "right polar form A = U P (scipy.linalg.polar's default)"],
     method="from the one-sided Jacobi SVD A = W S V^T: U = W V^T, P = V S V^T", latex=r"A = UP,\quad P = (A^{T}A)^{1/2},\ U = AP^{-1}",
     run=polar_decomposition, verify=_verify_polar,
     reference=lambda A: (lambda r: {"U": r[0].tolist(), "P": r[1].tolist()})(__import__("scipy.linalg", fromlist=["polar"]).polar(__import__("numpy").asarray(A, float))),
     wrong=_wrong_polar,
     random_inputs=lambda rng: {"A": _rnd_float_mat(rng, rng.randint(2, 4))},
     parse=_safe_parse(lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bpolar decomposition\b", r"\bpolar factori[sz]ation\b") else None),
     templates=["Find the polar decomposition of [[2, 0], [0, 3]].", "Compute the polar decomposition of the matrix [0 -2; 1 0]."],
     examples=[{"inputs": {"A": "[[2, 0], [0, 3]]"}, "outputs": {"U": [[1.0, 0.0], [0.0, 1.0]], "P": [[2.0, 0.0], [0.0, 3.0]]}},
               {"inputs": {"A": [[0, -2], [1, 0]]}, "outputs": {"U": [[0.0, -1.0], [1.0, 0.0]], "P": [[1.0, 0.0], [0.0, 2.0]]}}],
     questions=[{"q": "Compute the polar decomposition of the matrix [0 -2; 1 0].", "outputs": {"U": [[0.0, -1.0], [1.0, 0.0]], "P": [[1.0, 0.0], [0.0, 2.0]]}},
                {"q": "Find the polar decomposition of [[1, 2], [2, 4]].", "outcome": "INVALID_INPUT"}])


# ============================================================================================================
# BATCH 4 — iterative solvers, tridiagonal and sparse matrices, discrete Poisson problems
# ============================================================================================================
def _sdd(A):
    F = _FM(A)
    return all(abs(F[i][i]) > sum((abs(x) for j, x in enumerate(F[i]) if j != i), Fraction(0)) for i in range(len(F)))


def _spd(A):
    F = _FM(A)
    n = len(F)
    if any(F[i][j] != F[j][i] for i in range(n) for j in range(i)):
        return False
    return all(_det_F([r[:k] for r in F[:k]]) > 0 for k in range(1, n + 1))


def _fixed_point(step, x, A, b):
    """Iterate x <- step(x) until it stops changing (or the residual reaches rounding level); ValueError otherwise."""
    best = None
    for _ in range(50000):
        y = step(x)
        d = max(abs(p - q) for p, q in zip(x, y))
        x = y
        if d <= 2.3e-16 * max(1.0, max(abs(t) for t in x)):
            return x
        if best is not None and d >= best and d <= 1e-13 * max(1.0, max(abs(t) for t in x)):
            return x                                     # rounding-level oscillation: converged
        best = d if best is None else min(best, d)
    raise ValueError("the iteration did not converge")


def jacobi_iteration(A, b):
    A = _need_mat(A, "A", square=True)
    b = _need_vec(b, "b", len(A))
    if not _sdd(A):
        raise ValueError("A must be strictly diagonally dominant by rows (then the Jacobi iteration converges)")
    n = len(A)
    A = [[float(x) for x in r] for r in A]
    step = lambda x: [(b[i] - math.fsum(A[i][j] * x[j] for j in range(n) if j != i)) / A[i][i] for i in range(n)]
    return {"solution": _fixed_point(step, [0.0] * n, A, b)}


def gauss_seidel(A, b):
    A = _need_mat(A, "A", square=True)
    b = _need_vec(b, "b", len(A))
    if not (_sdd(A) or _spd(A)):
        raise ValueError("A must be strictly diagonally dominant or symmetric positive definite (then Gauss-Seidel converges)")
    n = len(A)
    A = [[float(x) for x in r] for r in A]

    def step(x):
        x = list(x)
        for i in range(n):
            x[i] = (b[i] - math.fsum(A[i][j] * x[j] for j in range(n) if j != i)) / A[i][i]
        return x
    return {"solution": _fixed_point(step, [0.0] * n, A, b)}


def conjugate_gradient(A, b):
    A = _need_mat(A, "A", square=True)
    b = _need_vec(b, "b", len(A))
    if not _spd(A):
        raise ValueError("A must be symmetric positive definite")
    n = len(A)
    A = [[float(x) for x in r] for r in A]
    x = [0.0] * n
    nb = _norm(b)
    if nb == 0:
        return {"solution": x}
    for _ in range(3):                                  # CG, restarted from the current x (refines rounding errors)
        r = [bi - v for bi, v in zip(b, _mv(A, x))]
        p = list(r)
        rr = _dot(r, r)
        for _ in range(4 * n):
            if math.sqrt(rr) <= 1e-17 * nb:
                break
            Ap = _mv(A, p)
            alpha = rr / _dot(p, Ap)
            x = [a + alpha * c for a, c in zip(x, p)]
            r = [a - alpha * c for a, c in zip(r, Ap)]
            rr2 = _dot(r, r)
            p = [a + (rr2 / rr) * c for a, c in zip(r, p)]
            rr = rr2
    res = _norm([bi - v for bi, v in zip(b, _mv(A, x))])
    if res > 1e-12 * (_fro(A) * _norm(x) + nb):
        raise ValueError("conjugate gradients did not converge")
    return {"solution": x}


def _random_sdd_system(rng):
    n = rng.randint(2, 6)
    A = _rmat(rng, n)
    for i in range(n):
        s = sum(abs(_F(x)) for j, x in enumerate(A[i]) if j != i)
        A[i][i] = float(s) + rng.randint(1, 9) * (1 if rng.random() < 0.7 else -1)
    return {"A": A, "b": _rvec(rng, n)}


def _iter_wrong(A, b, kind):
    n = len(A)
    x = [0.0] * n
    for _ in range(3):
        x = [(b[i] - math.fsum(A[i][j] * x[j] for j in range(n) if j != i)) / A[i][i] for i in range(n)]
    return {"solution": x}


def _parse_iter(words):
    def p(q):
        # the method converges fully: a stated tolerance or number of iterations would ask for something else, and a
        # gradient or step (an optimization reading, x = -A^-1 g) is not the right-hand side b
        if not q.has(*words) or len(q.mats) != 1 or len(q.vecs) > 1 or q.has(r"\bstep\b", r"\bdirection\b") \
                or re.search(r"\bgradients?\b", re.sub(r"\bconjugate gradients?\b", " ", q.text)):
            return None
        A = (_pick(q, "matrix", "A") or q.mats[0])["text"]
        if len(q.vecs) == 1:
            return None if _loose(q) else {"A": A, "b": q.vecs[0]["text"]}
        b = _bare_vector(q)
        return None if b is None else {"A": A, "b": b}
    return p


_ref_solve = lambda A, b: {"solution": __import__("scipy.linalg", fromlist=["solve"]).solve(__import__("numpy").asarray(A, float), __import__("numpy").asarray(b, float)).tolist()}

_add("jacobi_iteration",
     name="Jacobi iteration for Ax = b", topic="linear systems / iterative methods",
     purpose="Solve Ax = b for a strictly diagonally dominant A by the Jacobi method, iterated to convergence.",
     inputs=_SYS_IN, outputs=[_O("solution", "the converged solution x")], primary="solution",
     conditions=["A is square and strictly diagonally dominant by rows (guarantees convergence)", "at most 10 x 10"],
     method="x_i <- (b_i - sum_{j != i} a_ij x_j) / a_ii for all i at once, from x = 0, until the iterates stop changing",
     latex=r"x_i^{(k+1)} = \frac{1}{a_{ii}}\Big(b_i - \sum_{j\ne i} a_{ij} x_j^{(k)}\Big)",
     run=jacobi_iteration, verify=_verify_solution, reference=_ref_solve, wrong=lambda A, b: _iter_wrong(A, b, "j"),
     random_inputs=_random_sdd_system, parse=_safe_parse(_parse_iter((r"\bjacobi (?:iteration|method)\b", r"\bjacobi's method\b"))),
     templates=["Use Jacobi iteration to solve Ax = b with A = [[4, 1], [2, 5]] and b = (6, 12).",
                "Solve Ax = b by the Jacobi method for A = [10 -1 2; -1 11 -1; 2 -1 10] and b = (6, 25, -11)."],
     examples=[{"inputs": {"A": "[[4, 1], [2, 5]]", "b": "6, 12"}, "outputs": {"solution": [1.0, 2.0]}},
               {"inputs": {"A": [[10, -1, 2], [-1, 11, -1], [2, -1, 10]], "b": [6, 25, -11]}, "outputs": {"solution": [1.0432692307692308, 2.269230769230769, -1.0817307692307692]}}],
     questions=[{"q": "Use Jacobi iteration to solve Ax = b with A = [[4, 1], [2, 5]] and b = (6, 12).", "outputs": {"solution": [1.0, 2.0]}},
                {"q": "Use Jacobi iteration to solve Ax = b with A = [[1, 2], [3, 4]] and b = (1, 1).", "outcome": "INVALID_INPUT"}])

_add("gauss_seidel",
     name="Gauss-Seidel iteration for Ax = b", topic="linear systems / iterative methods",
     purpose="Solve Ax = b by the Gauss-Seidel method (each new value used at once), iterated to convergence.",
     inputs=_SYS_IN, outputs=[_O("solution", "the converged solution x")], primary="solution",
     conditions=["A is square and strictly diagonally dominant, or symmetric positive definite (guarantees convergence)", "at most 10 x 10"],
     method="x_i <- (b_i - sum_{j<i} a_ij x_j(new) - sum_{j>i} a_ij x_j(old)) / a_ii, from x = 0, until the iterates stop changing",
     latex=r"x_i^{(k+1)} = \frac{1}{a_{ii}}\Big(b_i - \sum_{j<i} a_{ij} x_j^{(k+1)} - \sum_{j>i} a_{ij} x_j^{(k)}\Big)",
     run=gauss_seidel, verify=_verify_solution, reference=_ref_solve, wrong=lambda A, b: _iter_wrong(A, b, "gs"),
     random_inputs=lambda rng: _random_sdd_system(rng) if rng.random() < 0.6 else (lambda A: {"A": A, "b": _rvec(rng, len(A))})(_random_spd(rng)),
     parse=_safe_parse(_parse_iter((r"\bgauss-?seidel\b",))),
     templates=["Use Gauss-Seidel to solve Ax = b with A = [[4, 1], [2, 5]] and b = (6, 12).",
                "Solve Ax = b by the Gauss-Seidel method for A = [4 -1 0; -1 4 -1; 0 -1 4] and b = (2, 4, 10)."],
     examples=[{"inputs": {"A": "[[4, 1], [2, 5]]", "b": "6, 12"}, "outputs": {"solution": [1.0, 2.0]}},
               {"inputs": {"A": [[4, -1, 0], [-1, 4, -1], [0, -1, 4]], "b": [2, 4, 10]}, "outputs": {"solution": [1.0, 2.0, 3.0]}}],
     questions=[{"q": "Use Gauss-Seidel to solve Ax = b with A = [[4, 1], [2, 5]] and b = (6, 12).", "outputs": {"solution": [1.0, 2.0]}},
                {"q": "Use Gauss-Seidel to solve Ax = b with A = [[1, 3], [3, 1]] and b = (1, 1).", "outcome": "INVALID_INPUT"}])

_add("conjugate_gradient",
     name="Conjugate gradient method for Ax = b", topic="linear systems / iterative methods",
     purpose="Solve Ax = b for a symmetric positive definite A by the method of conjugate gradients.",
     inputs=_SYS_IN, outputs=[_O("solution", "the solution x")], primary="solution",
     conditions=["A is symmetric positive definite (checked exactly)", "at most 10 x 10"],
     method="conjugate gradients from x = 0 (at most n steps in exact arithmetic), restarted twice to remove rounding errors",
     latex=r"\alpha_k = \frac{\mathbf{r}_k^{T}\mathbf{r}_k}{\mathbf{p}_k^{T}A\mathbf{p}_k},\ \mathbf{x}_{k+1} = \mathbf{x}_k + \alpha_k\mathbf{p}_k,\ \mathbf{p}_{k+1} = \mathbf{r}_{k+1} + \frac{\mathbf{r}_{k+1}^{T}\mathbf{r}_{k+1}}{\mathbf{r}_k^{T}\mathbf{r}_k}\mathbf{p}_k",
     run=conjugate_gradient, verify=_verify_solution, reference=_ref_solve,
     wrong=lambda A, b: (lambda x: {"solution": x})((lambda r: [(_dot(r, r) / _dot(r, _mv(A, r))) * t for t in r])(list(map(float, b)))),
     random_inputs=lambda rng: (lambda A: {"A": A, "b": _rvec(rng, len(A))})(_random_spd(rng)),
     parse=_safe_parse(_parse_iter((r"\bconjugate gradients?\b",))),
     templates=["Use conjugate gradients to solve Ax = b with A = [[4, 1], [1, 3]] and b = (1, 2).",
                "Solve Ax = b by the conjugate gradient method for A = [2 -1 0; -1 2 -1; 0 -1 2] and b = (1, 0, 1)."],
     examples=[{"inputs": {"A": "[[4, 1], [1, 3]]", "b": "1, 2"}, "outputs": {"solution": [1 / 11, 7 / 11]}},
               {"inputs": {"A": [[2, -1, 0], [-1, 2, -1], [0, -1, 2]], "b": [1, 0, 1]}, "outputs": {"solution": [1.0, 1.0, 1.0]}}],
     questions=[{"q": "Use conjugate gradients to solve Ax = b with A = [[4, 1], [1, 3]] and b = (1, 2).", "outputs": {"solution": [1 / 11, 7 / 11]}},
                {"q": "Use conjugate gradients to solve Ax = b with A = [[1, 2], [2, 1]] and b = (1, 2).", "outcome": "INVALID_INPUT"}])


def tridiagonal_solve(sub, diag, sup, d):
    diag = _need_vec(diag, "diag", min_n=2, max_n=200)
    n = len(diag)
    sub, sup, d = _need_vec(sub, "sub", n - 1), _need_vec(sup, "sup", n - 1), _need_vec(d, "d", n)
    a, b, c, r = [Fraction(0)] + _FV(sub), _FV(diag), _FV(sup) + [Fraction(0)], _FV(d)
    cp, dp = [Fraction(0)] * n, [Fraction(0)] * n
    for i in range(n):                                  # the Thomas algorithm, exact
        den = b[i] - (a[i] * cp[i - 1] if i else 0)
        if den == 0:
            raise ValueError("a zero pivot arises: the Thomas algorithm needs nonzero pivots (e.g. a diagonally dominant matrix)")
        cp[i] = c[i] / den
        dp[i] = (r[i] - (a[i] * dp[i - 1] if i else 0)) / den
    x = [Fraction(0)] * n
    for i in reversed(range(n)):
        x[i] = dp[i] - (cp[i] * x[i + 1] if i < n - 1 else 0)
    return {"solution": _fl(x)}


def _verify_tridiag(i, o):
    n, x = len(i["diag"]), o["solution"]
    if len(x) != n:
        return BAD
    A = [[(i["diag"][r] if c == r else i["sub"][c] if c == r - 1 else i["sup"][r] if c == r + 1 else 0.0) for c in range(n)] for r in range(n)]
    return _verify_solution({"A": A, "b": i["d"]}, o)


def _reference_tridiag(sub, diag, sup, d):
    import numpy as np
    from scipy.linalg import solve_banded
    n = len(diag)
    ab = np.zeros((3, n))
    ab[0, 1:], ab[1], ab[2, :-1] = sup, diag, sub
    return {"solution": solve_banded((1, 1), ab, np.asarray(d, float)).tolist()}


def _parse_tridiag(q):
    if not q.has(r"\btridiagonal\b", r"\bthomas\b") or q.mats or len(q.vecs) != 4:
        return None
    got = {}
    for o in q.vecs:
        before = q.raw[:o["start"]].lower()
        tail = before[-40:]
        if o["name"] in ("sub", "diag", "sup", "d"):
            key = o["name"]
        elif re.search(r"(?:sub-?diagonal|lower diagonal)\s*(?:=\s*)?$", tail):
            key = "sub"
        elif re.search(r"(?:super-?diagonal|upper diagonal)\s*(?:=\s*)?$", tail):
            key = "sup"
        elif re.search(r"(?:main diagonal|diagonal)\s*(?:=\s*)?$", tail):
            key = "diag"
        elif re.search(r"(?:right-hand side|right hand side|rhs)\s*(?:=\s*)?$", tail):
            key = "d"
        else:
            return None
        if key in got:
            return None
        got[key] = o["text"]
    return got if len(got) == 4 else None


_add("tridiagonal_solve",
     name="Tridiagonal system (Thomas algorithm)", topic="linear systems / banded and sparse systems",
     purpose="Solve a tridiagonal system given its three diagonals and right-hand side, in O(n) operations.",
     inputs=[_I("sub", "vector", "sub-diagonal a_2..a_n (n-1 entries)"), _I("diag", "vector", "main diagonal b_1..b_n"),
             _I("sup", "vector", "super-diagonal c_1..c_(n-1) (n-1 entries)"), _I("d", "vector", "right-hand side (n entries)")],
     outputs=[_O("solution", "the solution x")], primary="solution",
     conditions=["n >= 2, at most 200", "no zero pivot arises in the elimination (true for diagonally dominant matrices)"],
     method="the Thomas algorithm (forward sweep, back substitution) in exact rational arithmetic",
     latex=r"a_i x_{i-1} + b_i x_i + c_i x_{i+1} = d_i",
     run=tridiagonal_solve, verify=_verify_tridiag, reference=_reference_tridiag,
     wrong=lambda sub, diag, sup, d: tridiagonal_solve([0.0] * len(sub), diag, sup, d),
     random_inputs=lambda rng: (lambda n: (lambda sub, sup: {"sub": sub, "sup": sup, "d": _rvec(rng, n),
                                                             "diag": [abs(float(_F((sub[i - 1] if i else 0)))) + abs(float(_F((sup[i] if i < n - 1 else 0)))) + rng.randint(1, 5) for i in range(n)]})(
         _rvec(rng, n - 1), _rvec(rng, n - 1)))(rng.randint(2, 12)),
     parse=_safe_parse(_parse_tridiag),
     templates=["Solve the tridiagonal system with sub-diagonal (1, 1), diagonal (4, 4, 4), super-diagonal (1, 1) and right-hand side (5, 6, 5).",
                "Use the Thomas algorithm with lower diagonal [-1, -1, -1], main diagonal [2, 2, 2, 2], upper diagonal [-1, -1, -1] and right-hand side [1, 0, 0, 1]."],
     examples=[{"inputs": {"sub": "1, 1", "diag": "4, 4, 4", "sup": "1, 1", "d": "5, 6, 5"}, "outputs": {"solution": [1.0, 1.0, 1.0]}},
               {"inputs": {"sub": [-1, -1, -1], "diag": [2, 2, 2, 2], "sup": [-1, -1, -1], "d": [1, 0, 0, 1]}, "outputs": {"solution": [1.0, 1.0, 1.0, 1.0]}}],
     questions=[{"q": "Solve the tridiagonal system with sub-diagonal (1, 1), diagonal (4, 4, 4), super-diagonal (1, 1) and right-hand side (5, 6, 5).",
                 "outputs": {"solution": [1.0, 1.0, 1.0]}},
                {"q": "Solve the tridiagonal system with sub-diagonal (1, 1), diagonal (4, 4, 4), super-diagonal (1, 1, 1) and right-hand side (5, 6, 5).",
                 "outcome": "INVALID_INPUT"}])


def _compress(A, by_rows):
    A = _need_mat(A, "A", max_dim=50)
    m, n = len(A), len(A[0])
    outer, inner = (m, n) if by_rows else (n, m)
    data, idx, ptr = [], [], [0]
    for o in range(outer):
        for k in range(inner):
            v = A[o][k] if by_rows else A[k][o]
            if v != 0:
                data.append(float(v))
                idx.append(k)
        ptr.append(len(data))
    return {"data": data, "indices": idx, "indptr": ptr, "nnz": len(data), "density": len(data) / (m * n)}


def csr_format(A):
    return _compress(A, True)


def csc_format(A):
    return _compress(A, False)


def _expand(data, indices, indptr, outer, inner, by_rows):
    M = [[0.0] * inner for _ in range(outer)]
    for o in range(outer):
        for k in range(int(indptr[o]), int(indptr[o + 1])):
            M[o][int(indices[k])] += data[k]
    return M if by_rows else _T(M)


def _verify_compressed(by_rows):
    def v(i, o):
        A = i["A"]
        m, n = len(A), len(A[0])
        outer, inner = (m, n) if by_rows else (n, m)
        data, idx, ptr = o["data"], o["indices"], o["indptr"]
        if len(ptr) != outer + 1 or ptr[0] != 0 or ptr[-1] != len(data) or len(idx) != len(data) or any(x == 0 for x in data):
            return BAD
        if any(ptr[k] > ptr[k + 1] for k in range(outer)) or any(not 0 <= j < inner for j in idx):
            return BAD
        if any(idx[k] >= idx[k + 1] for o2 in range(outer) for k in range(int(ptr[o2]), int(ptr[o2 + 1]) - 1)):
            return BAD                                       # canonical: sorted, no duplicates
        M = _expand(data, idx, ptr, outer, inner, by_rows)
        return max(max(abs(a - b) for r, t in zip(M, A) for a, b in zip(r, t)) / max(1.0, max(abs(x) for r in A for x in r)),
                   abs(o["nnz"] - len(data)), abs(o["density"] - len(data) / (m * n)))
    return v


def _reference_compressed(A, by_rows):
    import numpy as np
    from scipy import sparse
    M = (sparse.csr_matrix if by_rows else sparse.csc_matrix)(np.asarray(A, float))
    M.sort_indices()
    return {"data": M.data.tolist(), "indices": M.indices.tolist(), "indptr": M.indptr.tolist(), "nnz": int(M.nnz),
            "density": M.nnz / (M.shape[0] * M.shape[1])}


def _random_sparse(rng):
    m, n = rng.randint(2, 6), rng.randint(2, 6)
    return {"A": [[(_rnum(rng) or 1) if rng.random() < 0.35 else 0 for _ in range(n)] for _ in range(m)]}


for _key, _rows in (("csr_format", True), ("csc_format", False)):
    _nm = "row" if _rows else "column"
    _add(_key,
         name=f"Compressed sparse {_nm} (CS{'R' if _rows else 'C'}) form of a matrix", topic="sparse matrices / storage formats",
         purpose=f"Store a matrix in compressed sparse {_nm} form: the nonzero values, their {'column' if _rows else 'row'} indices and the {_nm} pointers.",
         inputs=[_I("A", "matrix", "the matrix (up to 50 x 50)")],
         outputs=[_O("data", f"nonzero values, {_nm} by {_nm}"), _O("indices", f"0-based {'column' if _rows else 'row'} index of each value"),
                  _O("indptr", f"{_nm} pointers: the values of {_nm} k are data[indptr[k]:indptr[k+1]]"),
                  _O("nnz", "number of stored nonzeros"), _O("density", "nnz / (rows x columns)")],
         primary="nnz", conditions=["at most 50 x 50", "zeros are not stored; indices sorted within each " + _nm + " (canonical form)"],
         method=f"scan {_nm} by {_nm}, keep the nonzeros", latex=r"\texttt{data}[\texttt{indptr}[k]:\texttt{indptr}[k+1]]",
         run=csr_format if _rows else csc_format, verify=_verify_compressed(_rows),
         reference=(lambda A: _reference_compressed(A, True)) if _rows else (lambda A: _reference_compressed(A, False)),
         wrong=csc_format if _rows else csr_format,
         random_inputs=_random_sparse,
         parse=_safe_parse((lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bcsr\b", r"\bcompressed sparse row\b")
                            else None) if _rows else
                           (lambda q: {"A": q.mats[0]["text"]} if len(q.mats) == 1 and not q.vecs and q.has(r"\bcsc\b", r"\bcompressed sparse column\b")
                            else None)),
         templates=[f"Convert [[1, 0, 2], [0, 0, 3], [4, 5, 0]] to {'CSR' if _rows else 'CSC'} format.",
                    f"Write the matrix [0 7; 0 0; 8 0] in compressed sparse {_nm} form."],
         examples=[{"inputs": {"A": "[[1, 0, 2], [0, 0, 3], [4, 5, 0]]"},
                    "outputs": ({"data": [1.0, 2.0, 3.0, 4.0, 5.0], "indices": [0, 2, 2, 0, 1], "indptr": [0, 2, 3, 5], "nnz": 5} if _rows else
                                {"data": [1.0, 4.0, 5.0, 2.0, 3.0], "indices": [0, 2, 2, 0, 1], "indptr": [0, 2, 3, 5], "nnz": 5})},
                   {"inputs": {"A": [[0, 7], [0, 0], [8, 0]]},
                    "outputs": ({"data": [7.0, 8.0], "indices": [1, 0], "indptr": [0, 1, 1, 2]} if _rows else
                                {"data": [8.0, 7.0], "indices": [2, 0], "indptr": [0, 1, 2]})}],
         questions=[{"q": f"Convert [[1, 0, 2], [0, 0, 3], [4, 5, 0]] to {'CSR' if _rows else 'CSC'} format.", "outputs": {"nnz": 5}}])


def csr_to_dense(data, indices, indptr, ncols):
    data, indices, indptr = _need_vec(data, "data", min_n=0), _need_vec(indices, "indices", min_n=0), _need_vec(indptr, "indptr", min_n=2)
    if isinstance(ncols, bool) or ncols != int(ncols) or not 1 <= ncols <= 50:
        raise ValueError("ncols must be a whole number from 1 to 50")
    if len(indices) != len(data) or indptr[0] != 0 or indptr[-1] != len(data) or any(indptr[k] > indptr[k + 1] for k in range(len(indptr) - 1)):
        raise ValueError("inconsistent CSR arrays: indptr must start at 0, never decrease and end at len(data) = len(indices)")
    if any(x != int(x) for x in list(indices) + list(indptr)) or any(not 0 <= j < ncols for j in indices) or len(indptr) - 1 > 50:
        raise ValueError("every index must be a whole number from 0 to ncols - 1 (at most 50 rows)")
    D = [_F(x) for x in data]
    M = [[Fraction(0)] * int(ncols) for _ in range(len(indptr) - 1)]
    for r in range(len(indptr) - 1):
        for k in range(int(indptr[r]), int(indptr[r + 1])):
            M[r][int(indices[k])] += D[k]                     # duplicates are summed
    return {"dense": _fl(M)}


def _random_csr(rng):
    A = _random_sparse(rng)["A"]
    r = _compress(A, True)
    return {"data": r["data"], "indices": [float(x) for x in r["indices"]], "indptr": [float(x) for x in r["indptr"]], "ncols": len(A[0])}


def _parse_csr_arrays(q, need_x):
    names = {o["name"]: o["text"] for o in q.vecs if o["name"]}
    if not all(k in names for k in ("data", "indices", "indptr")) or q.mats:
        return None
    out = {k: names[k] for k in ("data", "indices", "indptr")}
    if need_x:
        if "x" not in names:
            return None
        out["x"] = names["x"]
    else:
        m = re.search(r"\b(\d+)\s+columns\b", q.text)
        nc = q.named("ncols") if m is None else float(m.group(1))
        if nc is None:
            return None
        out["ncols"] = nc
    return out


_add("csr_to_dense",
     name="Dense matrix from CSR arrays", topic="sparse matrices / storage formats",
     purpose="Rebuild the full matrix from compressed sparse row arrays (data, indices, indptr) and the number of columns.",
     inputs=[_I("data", "vector", "stored values", min_length=0), _I("indices", "vector", "0-based column index of each value", min_length=0),
             _I("indptr", "vector", "row pointers (rows + 1 entries)"), _I("ncols", "integer", "number of columns", min=1, max=50)],
     outputs=[_O("dense", "the full matrix")], primary="dense",
     conditions=["indptr starts at 0, never decreases, ends at len(data)", "indices within 0..ncols-1", "duplicate entries are summed", "at most 50 x 50"],
     method="row r holds data[indptr[r]:indptr[r+1]] at columns indices[indptr[r]:indptr[r+1]]",
     latex=r"A_{r,\,\texttt{indices}[k]} \mathrel{+}= \texttt{data}[k],\ \texttt{indptr}[r] \le k < \texttt{indptr}[r+1]",
     run=csr_to_dense, verify=_recompute(csr_to_dense, ["dense"]),
     reference=lambda data, indices, indptr, ncols: {"dense": __import__("scipy.sparse", fromlist=["csr_matrix"]).csr_matrix(
         (__import__("numpy").asarray(data, float), __import__("numpy").asarray(indices, int), __import__("numpy").asarray(indptr, int)),
         shape=(len(indptr) - 1, int(ncols))).toarray().tolist()},
     wrong=lambda data, indices, indptr, ncols: csr_to_dense(
         [x for r in range(len(indptr) - 1) for x in data[int(indptr[r]):max(int(indptr[r]), int(indptr[r + 1]) - 1)]],
         [x for r in range(len(indptr) - 1) for x in indices[int(indptr[r]):max(int(indptr[r]), int(indptr[r + 1]) - 1)]],
         [0] + [sum(max(0, int(indptr[k + 1]) - int(indptr[k]) - 1) for k in range(r + 1)) for r in range(len(indptr) - 1)], ncols),
     random_inputs=_random_csr, parse=_safe_parse(lambda q: _parse_csr_arrays(q, False) if q.has(r"\bcsr\b", r"\bdense\b", r"\bfull matrix\b") else None),
     templates=["Build the dense matrix with 3 columns from the CSR arrays data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3]."],
     examples=[{"inputs": {"data": "1, 2, 3", "indices": "0, 2, 1", "indptr": "0, 2, 3", "ncols": 3}, "outputs": {"dense": [[1.0, 0.0, 2.0], [0.0, 3.0, 0.0]]}},
               {"inputs": {"data": [5], "indices": [1], "indptr": [0, 0, 1], "ncols": 2}, "outputs": {"dense": [[0.0, 0.0], [0.0, 5.0]]}}],
     questions=[{"q": "Build the dense matrix with 3 columns from the CSR arrays data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3].",
                 "outputs": {"dense": [[1.0, 0.0, 2.0], [0.0, 3.0, 0.0]]}},
                {"q": "Build the dense matrix with 2 columns from the CSR arrays data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3].",
                 "outcome": "INVALID_INPUT"}])


def sparse_matvec(data, indices, indptr, x):
    x = _need_vec(x, "x", max_n=50)
    M = csr_to_dense(data, indices, indptr, len(x))["dense"]
    X = _FV(x)
    return {"product": [float(sum((_F(a) * b for a, b in zip(r, X)), Fraction(0))) for r in M]}


_add("sparse_matvec",
     name="Sparse matrix times vector (CSR)", topic="sparse matrices / sparse operations",
     purpose="The product y = A x for a matrix A given in compressed sparse row form.",
     inputs=[_I("data", "vector", "stored values", min_length=0), _I("indices", "vector", "0-based column index of each value", min_length=0),
             _I("indptr", "vector", "row pointers (rows + 1 entries)"), _I("x", "vector", "the vector (one entry per column)")],
     outputs=[_O("product", "the vector A x")], primary="product",
     conditions=["consistent CSR arrays", "column indices below len(x)", "at most 50 x 50"],
     method="y_r = sum over the stored entries of row r of data[k] x[indices[k]] (exact)",
     latex=r"y_r = \sum_{k=\texttt{indptr}[r]}^{\texttt{indptr}[r+1]-1} \texttt{data}[k]\,x_{\texttt{indices}[k]}",
     run=sparse_matvec, verify=_recompute(sparse_matvec, ["product"]),
     reference=lambda data, indices, indptr, x: {"product": (__import__("scipy.sparse", fromlist=["csr_matrix"]).csr_matrix(
         (__import__("numpy").asarray(data, float), __import__("numpy").asarray(indices, int), __import__("numpy").asarray(indptr, int)),
         shape=(len(indptr) - 1, len(x))) @ __import__("numpy").asarray(x, float)).tolist()},
     wrong=lambda data, indices, indptr, x: {"product": [math.fsum(data[k] * x[int(indices[k])] for k in range(int(indptr[r]), max(int(indptr[r]), int(indptr[r + 1]) - 1)))
                                                         for r in range(len(indptr) - 1)]},
     random_inputs=lambda rng: (lambda c: {**{k: c[k] for k in ("data", "indices", "indptr")}, "x": _rvec(rng, int(c["ncols"]))})(_random_csr(rng)),
     parse=_safe_parse(lambda q: _parse_csr_arrays(q, True) if q.has(r"\bcsr\b", r"\bsparse\b") else None),
     templates=["Multiply the CSR matrix data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3] by x = [1, 1, 1]."],
     examples=[{"inputs": {"data": "1, 2, 3", "indices": "0, 2, 1", "indptr": "0, 2, 3", "x": "1, 1, 1"}, "outputs": {"product": [3.0, 3.0]}},
               {"inputs": {"data": [2, -1], "indices": [0, 1], "indptr": [0, 1, 2], "x": [3, 4]}, "outputs": {"product": [6.0, -4.0]}}],
     questions=[{"q": "Multiply the CSR matrix data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3] by x = [1, 1, 1].", "outputs": {"product": [3.0, 3.0]}},
                {"q": "Multiply the CSR matrix data = [1, 2, 3], indices = [0, 2, 1], indptr = [0, 2, 3] by x = [1, 1].", "outcome": "INVALID_INPUT"}])


def poisson_1d(f, L):
    f = _need_vec(f, "f", min_n=1, max_n=200)
    _finite(L, "L")
    if L <= 0:
        raise ValueError("the interval length L must be positive")
    n = len(f)
    h = _F(L) / (n + 1)
    rhs = [h * h * x for x in _FV(f)]
    cp, dp = [Fraction(0)] * n, [Fraction(0)] * n          # Thomas algorithm on tridiag(-1, 2, -1), exact
    for i in range(n):
        den = 2 + (cp[i - 1] if i else 0)
        cp[i] = Fraction(-1) / den
        dp[i] = (rhs[i] + (dp[i - 1] if i else 0)) / den
    u = [Fraction(0)] * n
    for i in reversed(range(n)):
        u[i] = dp[i] - (cp[i] * u[i + 1] if i < n - 1 else 0)
    return {"u": _fl(u), "h": float(h)}


def _verify_poisson1d(i, o):
    f, u = i["f"], o["u"]
    n = len(f)
    if len(u) != n:
        return BAD
    h = i["L"] / (n + 1)
    ext = [0.0] + list(u) + [0.0]
    res = [(2 * ext[k] - ext[k - 1] - ext[k + 1]) - h * h * f[k - 1] for k in range(1, n + 1)]
    sc = max(1e-300, 4 * max(abs(x) for x in ext) + h * h * max(abs(x) for x in f))
    return max(max(abs(x) for x in res) / sc, abs(o["h"] - h) / h)


def _reference_poisson1d(f, L):
    import numpy as np
    import pyamg
    from scipy.sparse.linalg import spsolve
    n = len(f)
    A = pyamg.gallery.poisson((n,), format="csr").astype(float)
    h = L / (n + 1)
    return {"u": np.atleast_1d(spsolve(A.tocsc(), h * h * np.asarray(f, float))).tolist(), "h": h}


_add("poisson_1d",
     name="1-D Poisson equation by finite differences", topic="numerical PDE / discrete Poisson problems",
     purpose="Solve -u'' = f on (0, L) with u(0) = u(L) = 0 on n equally spaced interior points (second-order finite differences).",
     inputs=[_I("f", "vector", "values of f at the n interior grid points x_k = k L/(n+1)"), _I("L", "number", "length of the interval")],
     outputs=[_O("u", "approximate u at the interior points"), _O("h", "grid spacing L/(n+1)")], primary="u",
     conditions=["L > 0", "u = 0 at both ends (homogeneous Dirichlet conditions)", "at most 200 interior points"],
     method="the tridiagonal system (-u_(k-1) + 2u_k - u_(k+1))/h^2 = f_k solved exactly by the Thomas algorithm",
     latex=r"\frac{-u_{k-1} + 2u_k - u_{k+1}}{h^2} = f_k,\quad u_0 = u_{n+1} = 0",
     run=poisson_1d, verify=_verify_poisson1d, reference=_reference_poisson1d,
     wrong=lambda f, L: (lambda r: {"u": [x / (L / (len(f) + 1)) ** 2 for x in r["u"]], "h": r["h"]})(poisson_1d(f, L)),
     random_inputs=lambda rng: {"f": _rvec(rng, rng.randint(1, 15)), "L": rng.choice([1, 2, 0.5, round(rng.uniform(0.5, 5), 2)])},
     parse=_safe_parse(lambda q: (lambda L: {"f": (_pick(q, "vector", "f") or q.vecs[0])["text"], "L": L} if L is not None else None)(
         q.named("L") if q.named("L") is not None else (lambda m: float(m.group(2)) - float(m.group(1)) if m and float(m.group(1)) == 0 else None)(
             re.search(r"\bon\s*\[\s*(" + _N + r")\s*,\s*(" + _N + r")\s*\]", q.raw)))
         if q.has(r"\bpoisson\b") and not q.has(r"\b2-?d\b", r"\btwo-dimensional\b", r"\bgrid of\b") and len(q.vecs) >= 1 and not q.mats else None),
     templates=["Solve the 1-D Poisson equation -u'' = f with zero boundary values, L = 1 and f = (1, 1, 1) at the interior points.",
                "Solve the one-dimensional Poisson problem with u(0) = u(L) = 0, L = 4 and f = [2, 2, 2]."],
     examples=[{"inputs": {"f": "1, 1, 1", "L": 1}, "outputs": {"u": [3 / 32, 1 / 8, 3 / 32], "h": 0.25}},
               {"inputs": {"f": [2, 2, 2], "L": 4}, "outputs": {"u": [3.0, 4.0, 3.0], "h": 1.0}}],
     questions=[{"q": "Solve the 1-D Poisson equation -u'' = f with zero boundary values, L = 1 and f = (1, 1, 1) at the interior points.",
                 "outputs": {"u": [3 / 32, 1 / 8, 3 / 32]}},
                {"q": "Solve the one-dimensional Poisson problem with u(0) = u(L) = 0, L = -4 and f = [2, 2, 2].", "outcome": "INVALID_INPUT"}])


def poisson_2d(f, h):
    F = _need_mat(f, "f", max_dim=12)
    _finite(h, "h")
    if h <= 0:
        raise ValueError("the grid spacing h must be positive")
    n, m = len(F), len(F[0])
    N = n * m
    w = m                                                  # half bandwidth of the 5-point matrix in row-major order
    hh = float(h) * float(h)
    A = [[0.0] * N for _ in range(N)]
    rhs = [hh * float(F[i][j]) for i in range(n) for j in range(m)]
    for i in range(n):
        for j in range(m):
            k = i * m + j
            A[k][k] = 4.0
            if j > 0:
                A[k][k - 1] = -1.0
            if j < m - 1:
                A[k][k + 1] = -1.0
            if i > 0:
                A[k][k - m] = -1.0
            if i < n - 1:
                A[k][k + m] = -1.0
    for c in range(N):                                     # banded Gaussian elimination (SPD: no pivoting needed)
        for r in range(c + 1, min(N, c + w + 1)):
            if A[r][c] != 0:
                fct = A[r][c] / A[c][c]
                for k in range(c, min(N, c + w + 1)):
                    A[r][k] -= fct * A[c][k]
                rhs[r] -= fct * rhs[c]
    u = [0.0] * N
    for r in reversed(range(N)):
        u[r] = (rhs[r] - math.fsum(A[r][k] * u[k] for k in range(r + 1, min(N, r + w + 1)))) / A[r][r]
    return {"u": [u[i * m:(i + 1) * m] for i in range(n)]}


def _verify_poisson2d(i, o):
    F, U, h = i["f"], o["u"], i["h"]
    n, m = len(F), len(F[0])
    if _shape(U) != (n, m):
        return BAD
    g = lambda a, b: U[a][b] if 0 <= a < n and 0 <= b < m else 0.0
    res = max(abs(4 * U[a][b] - g(a - 1, b) - g(a + 1, b) - g(a, b - 1) - g(a, b + 1) - h * h * F[a][b]) for a in range(n) for b in range(m))
    sc = max(1e-300, 8 * max(abs(x) for r in U for x in r) + h * h * max(abs(x) for r in F for x in r))
    return res / sc


def _reference_poisson2d(f, h):
    import numpy as np
    import pyamg
    from scipy.sparse.linalg import spsolve
    F = np.asarray(f, float)
    A = pyamg.gallery.poisson(F.shape, format="csr").astype(float)
    return {"u": spsolve(A.tocsc(), h * h * F.ravel()).reshape(F.shape).tolist()}


_add("poisson_2d",
     name="2-D Poisson equation on a grid (5-point stencil)", topic="numerical PDE / discrete Poisson problems",
     purpose="Solve -(u_xx + u_yy) = f on a rectangle with u = 0 on the boundary, on an n x m grid of interior points with spacing h.",
     inputs=[_I("f", "matrix", "values of f at the interior grid points (row i, column j)"), _I("h", "number", "grid spacing (equal in x and y)")],
     outputs=[_O("u", "approximate u at the interior grid points")], primary="u",
     conditions=["h > 0", "u = 0 on the boundary", "at most 12 x 12 interior points"],
     method="the 5-point finite-difference system (4u_ij - neighbours)/h^2 = f_ij, solved by banded Gaussian elimination",
     latex=r"\frac{4u_{i,j} - u_{i-1,j} - u_{i+1,j} - u_{i,j-1} - u_{i,j+1}}{h^2} = f_{i,j}",
     run=poisson_2d, verify=_verify_poisson2d, reference=_reference_poisson2d,
     wrong=lambda f, h: (lambda r: {"u": [[x / (h * h) for x in row] for row in r["u"]]})(poisson_2d(f, h)),
     random_inputs=lambda rng: {"f": _rmat(rng, rng.randint(1, 6), rng.randint(1, 6)), "h": rng.choice([1, 0.5, 0.25, 0.1, round(rng.uniform(0.05, 2), 3)])},
     parse=_safe_parse(lambda q: {"f": q.mats[0]["text"], "h": q.named("h")} if len(q.mats) == 1 and not q.vecs and q.has(r"\bpoisson\b")
                       and q.named("h") is not None else None),
     templates=["Solve the 2-D Poisson equation with zero boundary values on the grid with h = 0.5 and f = [[1, 1], [1, 1]].",
                "Solve the Poisson problem -laplace(u) = f with u = 0 on the boundary, h = 1, f = [4 0 4; 0 4 0; 4 0 4]."],
     examples=[{"inputs": {"f": "[[1, 1], [1, 1]]", "h": 0.5}, "outputs": {"u": [[0.125, 0.125], [0.125, 0.125]]}},
               {"inputs": {"f": [[4]], "h": 1}, "outputs": {"u": [[1.0]]}}],
     questions=[{"q": "Solve the 2-D Poisson equation with zero boundary values on the grid with h = 0.5 and f = [[1, 1], [1, 1]].",
                 "outputs": {"u": [[0.125, 0.125], [0.125, 0.125]]}}])


# ============================================================================================================
# BATCH 4 — numerical methods: polynomials, root finding, interpolation, quadrature, differentiation
# ============================================================================================================
_PT = r"(?:\d+(?:\.\d+)?\s*\*?\s*)?x(?![a-z0-9_(])(?:\s*(?:\^|\*\*)\s*\d+)?|\d+(?:\.\d+)?(?![\d.]*\s*\*?\s*x)"
_PEXPR = re.compile(r"(?<![\w.^*/=])\s*([-+]?\s*(?:" + _PT + r")(?:\s*[-+]\s*(?:" + _PT + r"))*)")


def _poly_coeffs(expr):
    """Coefficients (highest power first) of a polynomial in x written term by term; None if it is not one."""
    s = expr.replace(" ", "").replace("**", "^")
    if not s.startswith(("+", "-")):
        s = "+" + s
    terms = re.findall(r"[+-][^+-]+", s)
    if "".join(terms) != s:
        return None
    co = {}
    for t in terms:
        m = re.fullmatch(r"([+-])(\d+(?:\.\d+)?)?\*?x(?:\^(\d+))?", t)
        if m:
            c = float(m.group(2)) if m.group(2) else 1.0
            d = int(m.group(3)) if m.group(3) else 1
        else:
            m = re.fullmatch(r"([+-])(\d+(?:\.\d+)?)", t)
            if not m:
                return None
            c, d = float(m.group(2)), 0
        co[d] = co.get(d, 0.0) + (c if m.group(1) == "+" else -c)
    deg = max(co)
    if deg > 12:
        return None
    return [co.get(k, 0.0) for k in range(deg, -1, -1)]


def _poly_from_question(q):
    """The polynomial of a question: written out in x, or 'coefficients = (...)' (highest power first)."""
    named = _pick(q, "vector", "coefficients")
    if named is not None and named["name"] == "coefficients":
        return named["text"]
    for m in _PEXPR.finditer(q.rest):
        e = m.group(1).strip()
        if "x" not in e or not re.search(r"[-+^]", e.lstrip("+-")):
            continue
        after = q.rest[m.end():].lstrip()
        if after.startswith("=") and not re.match(r"=\s*0(?!\d|\.\d)", after):
            return None                                  # an equation with something else on the right: not read
        c = _poly_coeffs(e)
        return _vec_text(c) if c else None
    return None


def _need_poly(c, name="coefficients"):
    c = _need_vec(c, name, min_n=1, max_n=13)
    while len(c) > 1 and c[0] == 0:
        c = c[1:]
    return c


def _peval(c, x):
    v = 0.0
    for a in c:
        v = v * x + a
    return v


def _pscale(c, x):
    return math.fsum(abs(a) * abs(x) ** (len(c) - 1 - k) for k, a in enumerate(c))


def _pder(c):
    n = len(c) - 1
    return [a * (n - k) for k, a in enumerate(c[:-1])] or [0.0]


def _well_conditioned_root(c, x):
    d = abs(_peval(_pder(c), x))
    delta = 1e-12 * max(1.0, abs(x))
    return d * delta > 1e3 * 2.2e-16 * max(_pscale(c, x), 1e-300)


def _root_residual(i, o, interval=None):
    """A sign change of p across [x - delta, x + delta] (delta = 1e-12 max(1, |x|)): a root lies within delta of x."""
    c, x = [float(t) for t in i["coefficients"]], o["root"]
    while len(c) > 1 and c[0] == 0:
        c = c[1:]
    if interval and not (interval[0] <= x <= interval[1]):
        return BAD
    if _peval(c, x) == 0:
        return 0.0
    delta = 1e-12 * max(1.0, abs(x))
    lo, hi = _peval(c, x - delta), _peval(c, x + delta)
    return 0.0 if lo * hi <= 0 else BAD


def _sturm_count(c, a, b):
    """Number of distinct real roots of the polynomial in (a, b] (exact Sturm sequence)."""
    P = [_FV(c)]
    d = len(c) - 1
    P.append([t * (d - k) for k, t in enumerate(P[0][:-1])])
    while len(P[-1]) > 1:                                 # remainder sequence p_(k+1) = -rem(p_(k-1), p_k)
        num, den = P[-2][:], P[-1]
        while len(num) >= len(den):
            f = num[0] / den[0]
            for k in range(len(den)):
                num[k] -= f * den[k]
            num = num[1:]
        while num and num[0] == 0:
            num = num[1:]
        if not num:
            break
        P.append([-t for t in num])

    def changes(x):
        vals = [sum((t * x ** (len(p) - 1 - k) for k, t in enumerate(p)), Fraction(0)) for p in P]
        vals = [v for v in vals if v != 0]
        return sum(1 for u, v in zip(vals, vals[1:]) if (u > 0) != (v > 0))
    return changes(_F(a)) - changes(_F(b))


def bisection_root(coefficients, a, b):
    c = _need_poly(coefficients)
    _finite(a, "a"), _finite(b, "b")
    if not a < b:
        raise ValueError("the interval must have a < b")
    if len(c) < 2:
        raise ValueError("a constant polynomial has no isolated root")
    fa, fb = _peval(c, a), _peval(c, b)
    if fa == 0 or fb == 0:
        raise ValueError("an end of the interval is itself a root: give an interval with the root strictly inside")
    if (fa > 0) == (fb > 0):
        raise ValueError("p(a) and p(b) must have opposite signs")
    if _sturm_count(c, a, b) != 1:
        raise ValueError("the interval must contain exactly one root")
    lo, hi = float(a), float(b)
    for _ in range(3000):
        mid = (lo + hi) / 2
        if mid <= lo or mid >= hi:
            break
        fm = _peval(c, mid)
        if fm == 0:
            lo = hi = mid
            break
        if (fm > 0) == (fa > 0):
            lo = mid
        else:
            hi = mid
    x = (lo + hi) / 2
    if not _well_conditioned_root(c, x):
        raise ValueError("the root is too ill-conditioned (a multiple or nearly multiple root)")
    return {"root": x}


def _wrong_bisection(coefficients, a, b):
    """Stops after 10 halvings (a plausible mistake: too few iterations)."""
    c = _need_poly(coefficients)
    lo, hi = float(a), float(b)
    for _ in range(10):
        mid = (lo + hi) / 2
        if (_peval(c, mid) > 0) == (_peval(c, lo) > 0):
            lo = mid
        else:
            hi = mid
    return {"root": (lo + hi) / 2}


def _wrong_newton(coefficients, x0):
    """Stops after 2 Newton steps."""
    c = _need_poly(coefficients)
    x = float(x0)
    for _ in range(2):
        d = _peval(_pder(c), x)
        x = x - _peval(c, x) / d if d else x
    return {"root": x}


def _random_poly_case(rng, with_interval=True):
    k = rng.randint(1, 4)
    roots = sorted(rng.sample(range(-8, 9), k))
    roots = [r + rng.choice([0, 0.5, 0.25, 1 / 3]) for r in roots]
    lead = rng.choice([1, 2, -1, 3, 0.5])
    c = [lead]
    for r in roots:
        c = [x - r * y for x, y in zip(c + [0.0], [0.0] + c)]
    c = [round(x, 6) for x in c]
    t = rng.choice(roots)
    return c, t, roots


def _random_bisection(rng):
    c, t, roots = _random_poly_case(rng)
    gaps = [abs(t - r) for r in roots if r != t] or [2.0]
    w = min(gaps) * rng.uniform(0.2, 0.45)
    return {"coefficients": c, "a": round(t - w * rng.uniform(0.5, 1), 3), "b": round(t + w * rng.uniform(0.5, 1), 3)}


def _parse_interval(q):
    for o in q.vecs:
        if o["value"] and len(o["value"]) == 2 and o["name"] is None and re.search(r"(?:on|in|over|interval)\s*(?:the\s+interval\s*)?$", q.raw[:o["start"]].lower().rstrip()):
            return o["value"]
    m = re.search(r"\bbetween\s+(" + _N + r")\s+and\s+(" + _N + r")", q.rest)
    if m:
        return [float(m.group(1)), float(m.group(2))]
    a, b = q.named("a"), q.named("b")
    return [a, b] if a is not None and b is not None else None


def _parse_root(method):
    def p(q):
        words = {"bisection": (r"\bbisection\b",), "newton": (r"\bnewton'?s? method\b", r"\bnewton-raphson\b", r"\bnewton iteration\b"),
                 "secant": (r"\bsecant\b",)}[method]
        if not q.has(*words) or q.mats:
            return None
        poly = _poly_from_question(q)
        if poly is None:
            return None
        if method == "bisection":
            iv = _parse_interval(q)
            return {"coefficients": poly, "a": iv[0], "b": iv[1]} if iv else None
        if method == "newton":
            x0 = q.named("x0")
            if x0 is None:
                m = re.search(r"\b(?:starting|start|initial guess|beginning)\s+(?:at|from|with|of)?\s*(?:x\s*=\s*)?(" + _N + r")", q.rest)
                x0 = float(m.group(1)) if m else None
            return {"coefficients": poly, "x0": x0} if x0 is not None else None
        x0, x1 = q.named("x0"), q.named("x1")
        return {"coefficients": poly, "x0": x0, "x1": x1} if x0 is not None and x1 is not None else None
    return p


_POLY_IN = _I("coefficients", "vector", "polynomial coefficients, highest power first (written out in x in a question)")

_add("bisection_root",
     name="Root of a polynomial by bisection", topic="numerical methods / root finding",
     purpose="The root of a polynomial inside an interval [a, b] where it changes sign, by repeated halving.",
     inputs=[_POLY_IN, _I("a", "number", "left end of the interval"), _I("b", "number", "right end of the interval")],
     outputs=[_O("root", "the root in [a, b]")], primary="root",
     conditions=["a < b", "p(a) and p(b) have opposite signs", "exactly one root in (a, b) (checked exactly by a Sturm sequence)",
                 "the root is simple and well conditioned"],
     method="bisection until the interval cannot be halved in floating point (about 52 halvings per unit)",
     latex=r"m = \tfrac{a+b}{2};\ [a,b] \leftarrow [a,m]\ \text{if}\ p(a)p(m) < 0\ \text{else}\ [m,b]",
     run=bisection_root, verify=lambda i, o: _root_residual(i, o, (i["a"], i["b"])),
     reference=lambda coefficients, a, b: {"root": float(__import__("scipy.optimize", fromlist=["brentq"]).brentq(
         lambda t: float(__import__("numpy").polyval(coefficients, t)), a, b, xtol=1e-300, rtol=8.9e-16, maxiter=1000))},
     wrong=_wrong_bisection,
     random_inputs=_random_bisection, parse=_safe_parse(_parse_root("bisection")),
     templates=["Use bisection to find the root of x^3 - x - 2 on [1, 2].", "Find the root of f(x) = x^2 - 2 between 1 and 2 by the bisection method."],
     examples=[{"inputs": {"coefficients": "1, 0, -2", "a": 1, "b": 2}, "outputs": {"root": math.sqrt(2)}},
               {"inputs": {"coefficients": [1, -1, -2], "a": 0, "b": 5}, "outputs": {"root": 2.0}}],
     questions=[{"q": "Find the root of f(x) = x^2 - 2 between 1 and 2 by the bisection method.", "outputs": {"root": math.sqrt(2)}},
                {"q": "Use bisection to find the root of x^3 - x - 2 on [1, 2].", "outputs": {"root": 1.5213797068045676}},
                {"q": "Use bisection to find the root of x^2 - 2 on [2, 3].", "outcome": "INVALID_INPUT"},
                {"q": "Use bisection to find the root of x^3 - x on [-2, 2].", "outcome": "INVALID_INPUT"}])


def _newton_like(c, step, x, x_prev=None):
    for _ in range(200):
        nx = step(x, x_prev)
        if nx is None:
            break
        if not math.isfinite(nx):
            raise ValueError("the iteration diverged")
        done = abs(nx - x) <= 4.4e-16 * max(abs(nx), 1e-300) or _peval(c, nx) == 0
        x_prev, x = x, nx
        if done:
            break
    else:
        raise ValueError("the iteration did not converge in 200 steps")
    for _ in range(3):                                   # settle the last bit (Newton steps on the converged root)
        d = _peval(_pder(c), x)
        if d == 0:
            break
        nx = x - _peval(c, x) / d
        if not math.isfinite(nx) or abs(nx - x) > 1e-12 * max(1.0, abs(x)):
            break
        x = nx
    if _peval(c, x) != 0 and not _well_conditioned_root(c, x):
        raise ValueError("the iteration did not reach a simple, well-conditioned root")
    if _root_residual({"coefficients": c}, {"root": x}) != 0.0:
        raise ValueError("the iteration did not converge to a root")
    return x


def newton_root(coefficients, x0):
    c = _need_poly(coefficients)
    _finite(x0, "x0")
    if len(c) < 2:
        raise ValueError("a constant polynomial has no isolated root")
    dc = _pder(c)

    def step(x, _):
        d = _peval(dc, x)
        if d == 0:
            raise ValueError("p'(x) = 0 at an iterate: Newton's method breaks down")
        return x - _peval(c, x) / d
    return {"root": _newton_like(c, step, float(x0))}


def secant_root(coefficients, x0, x1):
    c = _need_poly(coefficients)
    _finite(x0, "x0"), _finite(x1, "x1")
    if len(c) < 2:
        raise ValueError("a constant polynomial has no isolated root")
    if x0 == x1:
        raise ValueError("the two starting points must differ")

    def step(x, xp):
        fx, fp = _peval(c, x), _peval(c, xp)
        if fx == fp:
            return None if fx == 0 else (_raise("p(x0) = p(x1): the secant line is horizontal"))
        return x - fx * (x - xp) / (fx - fp)
    return {"root": _newton_like(c, step, float(x1), float(x0))}


def _raise(msg):
    raise ValueError(msg)


def _random_newton(rng):
    c, t, roots = _random_poly_case(rng)
    gaps = [abs(t - r) for r in roots if r != t] or [2.0]
    return {"coefficients": c, "x0": round(t + min(gaps) * rng.uniform(-0.15, 0.15), 4)}


def _random_secant(rng):
    c, t, roots = _random_poly_case(rng)
    gaps = [abs(t - r) for r in roots if r != t] or [2.0]
    g = min(gaps)
    return {"coefficients": c, "x0": round(t + g * rng.uniform(-0.15, -0.02), 4), "x1": round(t + g * rng.uniform(0.02, 0.15), 4)}


_add("newton_root",
     name="Root of a polynomial by Newton's method", topic="numerical methods / root finding",
     purpose="A root of a polynomial by Newton's method x <- x - p(x)/p'(x) from a starting value x0.",
     inputs=[_POLY_IN, _I("x0", "number", "starting value")], outputs=[_O("root", "the root the iteration converges to")], primary="root",
     conditions=["the iteration converges (within 200 steps) to a simple, well-conditioned root", "p'(x) never vanishes at an iterate"],
     method="Newton-Raphson iteration until the step is below the last bit of x",
     latex=r"x_{k+1} = x_k - \frac{p(x_k)}{p'(x_k)}",
     run=newton_root, verify=lambda i, o: _root_residual(i, o),
     reference=lambda coefficients, x0: {"root": float(__import__("scipy.optimize", fromlist=["newton"]).newton(
         lambda t: float(__import__("numpy").polyval(coefficients, t)), float(x0),
         fprime=lambda t: float(__import__("numpy").polyval(__import__("numpy").polyder(coefficients), t)), tol=1e-15, rtol=4.5e-16, maxiter=500))},
     wrong=_wrong_newton,
     random_inputs=_random_newton, parse=_safe_parse(_parse_root("newton")),
     templates=["Use Newton's method to find a root of x^3 - 2x - 5 starting from x0 = 2.", "Find the root of f(x) = x^2 - 2 by Newton's method with x0 = 1."],
     examples=[{"inputs": {"coefficients": "1, 0, -2, -5", "x0": 2}, "outputs": {"root": 2.0945514815423265}},
               {"inputs": {"coefficients": [1, 0, -2], "x0": 1}, "outputs": {"root": math.sqrt(2)}}],
     questions=[{"q": "Use Newton's method to find a root of x^3 - 2x - 5 starting from x0 = 2.", "outputs": {"root": 2.0945514815423265}},
                {"q": "Find the root of f(x) = x^2 - 2 by Newton's method with x0 = 1.", "outputs": {"root": math.sqrt(2)}},
                {"q": "Find the root of f(x) = x^2 + 1 by Newton's method with x0 = 1.", "outcome": "INVALID_INPUT"}])

_add("secant_root",
     name="Root of a polynomial by the secant method", topic="numerical methods / root finding",
     purpose="A root of a polynomial by the secant method from two starting values x0 and x1.",
     inputs=[_POLY_IN, _I("x0", "number", "first starting value"), _I("x1", "number", "second starting value")],
     outputs=[_O("root", "the root the iteration converges to")], primary="root",
     conditions=["x0 != x1", "the iteration converges (within 200 steps) to a simple, well-conditioned root"],
     method="secant iteration x_(k+1) = x_k - p(x_k)(x_k - x_(k-1))/(p(x_k) - p(x_(k-1)))",
     latex=r"x_{k+1} = x_k - p(x_k)\,\frac{x_k - x_{k-1}}{p(x_k) - p(x_{k-1})}",
     run=secant_root, verify=lambda i, o: _root_residual(i, o),
     reference=lambda coefficients, x0, x1: {"root": float(__import__("scipy.optimize", fromlist=["newton"]).newton(
         lambda t: float(__import__("numpy").polyval(coefficients, t)), float(x0), x1=float(x1), tol=1e-15, rtol=4.5e-16, maxiter=500))},
     wrong=lambda coefficients, x0, x1: {"root": (lambda c: (lambda a, b: a - _peval(c, a) * (a - b) / (_peval(c, a) - _peval(c, b))
                                                             if _peval(c, a) != _peval(c, b) else a)(float(x1), float(x0)))(_need_poly(coefficients))},
     random_inputs=_random_secant, parse=_safe_parse(_parse_root("secant")),
     templates=["Use the secant method to find a root of x^3 - 2x - 5 with x0 = 2 and x1 = 3.",
                "Find the root of f(x) = x^2 - 2 by the secant method with x0 = 1, x1 = 2."],
     examples=[{"inputs": {"coefficients": "1, 0, -2, -5", "x0": 2, "x1": 3}, "outputs": {"root": 2.0945514815423265}},
               {"inputs": {"coefficients": [1, 0, -2], "x0": 1, "x1": 2}, "outputs": {"root": math.sqrt(2)}}],
     questions=[{"q": "Use the secant method to find a root of x^3 - 2x - 5 with x0 = 2 and x1 = 3.", "outputs": {"root": 2.0945514815423265}},
                {"q": "Find the root of f(x) = x^2 - 2 by the secant method with x0 = 1, x1 = 1.", "outcome": "INVALID_INPUT"}])


def polynomial_evaluation(coefficients, x):
    c = _need_poly(coefficients)
    X = _F(x, "x")
    v, dv = Fraction(0), Fraction(0)
    for a in _FV(c):                                     # Horner, with the derivative alongside
        dv = dv * X + v
        v = v * X + a
    return {"value": float(v), "derivative": float(dv)}


def _parse_evaluate(q):
    if not q.has(r"\bevaluate\b", r"\bvalue of\b", r"\bhorner\b") or q.mats or q.has(r"\binterpolat", r"\bspline\b", r"\bintegra", r"\broot\b"):
        return None
    poly = _poly_from_question(q)
    m = re.search(r"\bat\s+x\s*=\s*(" + _N + r")", q.raw.lower()) or re.search(r"\bp\((" + _N + r")\)", q.raw.lower())
    return {"coefficients": poly, "x": float(m.group(1))} if poly and m else None


_add("polynomial_evaluation",
     name="Evaluate a polynomial and its derivative (Horner's method)", topic="numerical methods / polynomials",
     purpose="The value p(x) and the derivative p'(x) of a polynomial at a point.",
     inputs=[_POLY_IN, _I("x", "number", "the point")], outputs=[_O("value", "p(x)"), _O("derivative", "p'(x)")], primary="value",
     conditions=["at most degree 12"], method="Horner's scheme with the derivative carried along (exact decimal arithmetic)",
     latex=r"p(x) = (\cdots((a_n x + a_{n-1})x + a_{n-2})x + \cdots)x + a_0",
     run=polynomial_evaluation, verify=_recompute(polynomial_evaluation, ["value", "derivative"]),
     reference=lambda coefficients, x: (lambda np: {"value": float(np.polyval(coefficients, x)), "derivative": float(np.polyval(np.polyder(coefficients), x))})(__import__("numpy")),
     wrong=lambda coefficients, x: polynomial_evaluation(list(coefficients)[::-1], x),
     random_inputs=lambda rng: {"coefficients": _rvec(rng, rng.randint(2, 6)), "x": _rnum(rng)},
     parse=_safe_parse(_parse_evaluate),
     templates=["Evaluate p(x) = 2x^3 - 6x^2 + 2x - 1 at x = 3.", "Use Horner's method to evaluate x^4 - 3x + 1 at x = 2."],
     examples=[{"inputs": {"coefficients": "2, -6, 2, -1", "x": 3}, "outputs": {"value": 5.0, "derivative": 20.0}},
               {"inputs": {"coefficients": [1, 0, 0, -3, 1], "x": 2}, "outputs": {"value": 11.0, "derivative": 29.0}}],
     questions=[{"q": "Evaluate p(x) = 2x^3 - 6x^2 + 2x - 1 at x = 3.", "outputs": {"value": 5.0, "derivative": 20.0}},
                {"q": "Use Horner's method to evaluate x^4 - 3x + 1 at x = 2.", "outputs": {"value": 11.0}}])


def _table(x, y, min_n=2, distinct=True, increasing=False):
    x, y = _need_vec(x, "x", min_n=min_n, max_n=60), _need_vec(y, "y", min_n=min_n, max_n=60)
    if len(x) != len(y):
        raise ValueError("x and y must have the same number of values")
    X = _FV(x)
    if distinct and len(set(X)) != len(X):
        raise ValueError("the x values must be distinct")
    if increasing and any(a >= b for a, b in zip(X, X[1:])):
        raise ValueError("the x values must be strictly increasing")
    return X, _FV(y)


def lagrange_interpolation(x, y, at):
    X, Y = _table(x, y, min_n=1)
    t = _F(at, "at")
    v = Fraction(0)
    for i, (xi, yi) in enumerate(zip(X, Y)):
        L = Fraction(1)
        for j, xj in enumerate(X):
            if j != i:
                L *= (t - xj) / (xi - xj)
        v += yi * L
    return {"value": float(v)}


def _neville(x, y, t):
    X, Y = _FV(x), _FV(y)
    P = list(Y)
    n = len(X)
    for k in range(1, n):
        for i in range(n - k):
            P[i] = ((t - X[i + k]) * P[i] + (X[i] - t) * P[i + 1]) / (X[i] - X[i + k])
    return float(P[0])


def _random_table(rng, n=None, increasing=True):
    n = n or rng.randint(3, 7)
    x = sorted(rng.sample(range(-10, 11), n)) if rng.random() < 0.6 else sorted(set(round(rng.uniform(-5, 5), 2) for _ in range(n)))
    return x, [round(rng.uniform(-10, 10), 2) for _ in x]


def _parse_at(q):
    m = re.search(r"\bat\s+(?:x\s*=\s*)?(" + _N + r")(?!\d|\.\d)", q.rest)
    return float(m.group(1)) if m else q.named("at")


def _parse_interp(words, excl=(), need_at=True):
    def p(q):
        if not q.has(*words) or q.has(*excl):
            return None
        pts = _points(q) if q.has(r"\bpoints?\b", r"\bdata\b", r"\btable\b") or _pick(q, "vector", "x") else None
        if pts is None:
            xo, yo = _pick(q, "vector", "x"), _pick(q, "vector", "y")
            if xo is not None and yo is not None and xo["name"] == "x" and yo["name"] == "y":
                pts = {"x": xo["text"], "y": yo["text"]}
            elif len(q.vecs) >= 2 and all(v["value"] and len(v["value"]) == 2 for v in q.vecs) and not q.mats:
                pts = {"x": ", ".join(repr(v["value"][0]) for v in q.vecs), "y": ", ".join(repr(v["value"][1]) for v in q.vecs)}
        if pts is None:
            return None
        if need_at:
            at = _parse_at(q)
            if at is None:
                return None
            return dict(pts, at=at)
        return pts
    return p


_add("lagrange_interpolation",
     name="Lagrange interpolation", topic="numerical methods / interpolation",
     purpose="The value at a point of the polynomial of lowest degree through given data points (Lagrange form).",
     inputs=[_I("x", "vector", "x values of the data (distinct)"), _I("y", "vector", "y values of the data"), _I("at", "number", "the point where the interpolant is evaluated")],
     outputs=[_O("value", "interpolated value p(at)")], primary="value",
     conditions=["distinct x values", "x and y of equal length (at most 60)"],
     method="the Lagrange formula sum y_i prod (t - x_j)/(x_i - x_j), exact decimal arithmetic",
     latex=r"p(t) = \sum_i y_i \prod_{j\ne i}\frac{t - x_j}{x_i - x_j}",
     run=lagrange_interpolation,
     verify=lambda i, o: (lambda w: abs(o["value"] - w) / max(1.0, abs(w)))(_neville(i["x"], i["y"], _F(i["at"]))),
     reference=lambda x, y, at: {"value": float(__import__("scipy.interpolate", fromlist=["BarycentricInterpolator"]).BarycentricInterpolator(x, y)(at))},
     wrong=lambda x, y, at: (lambda pts: (lambda k: {"value": pts[k][1] + (pts[k + 1][1] - pts[k][1]) * (at - pts[k][0]) / (pts[k + 1][0] - pts[k][0])})(
         max([j for j in range(len(pts) - 1) if pts[j][0] <= at] or [0])))(sorted(zip(x, y))),
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "at": round(rng.uniform(min(xy[0]), max(xy[0])), 3)})(_random_table(rng, rng.randint(3, 6))),
     parse=_safe_parse(_parse_interp((r"\blagrange\b",))),
     templates=["Use Lagrange interpolation through the points (0, 1), (1, 3), (2, 2) to estimate the value at x = 1.5.",
                "Interpolate the data x = (1, 2, 4), y = (1, 4, 16) with the Lagrange polynomial at 3."],
     examples=[{"inputs": {"x": "0, 1, 2", "y": "1, 3, 2", "at": 1.5}, "outputs": {"value": 2.875}},
               {"inputs": {"x": [1, 2, 4], "y": [1, 4, 16], "at": 3}, "outputs": {"value": 9.0}}],
     questions=[{"q": "Use Lagrange interpolation through the points (0, 1), (1, 3), (2, 2) to estimate the value at x = 1.5.", "outputs": {"value": 2.875}},
                {"q": "Interpolate the data x = (1, 2, 4), y = (1, 4, 16) with the Lagrange polynomial at 3.", "outputs": {"value": 9.0}},
                {"q": "Use Lagrange interpolation through the points (1, 1), (1, 3), (2, 2) to estimate the value at x = 1.5.", "outcome": "INVALID_INPUT"}])


def divided_differences(x, y):
    X, Y = _table(x, y, min_n=1)
    col, out = list(Y), [Y[0]]
    for k in range(1, len(X)):
        col = [(col[i + 1] - col[i]) / (X[i + k] - X[i]) for i in range(len(col) - 1)]
        out.append(col[0])
    return {"coefficients": _fl(out)}


def _reference_divdiff(x, y):
    import sympy as sp
    t = sp.Symbol("t")
    pts = [(sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))) for a, b in zip(x, y)]
    return {"coefficients": [float(sp.Poly(sp.interpolate(pts[:k + 1], t), t).coeff_monomial(t ** k)) if k else float(pts[0][1]) for k in range(len(pts))]}


_add("newton_divided_differences",
     name="Newton divided differences", topic="numerical methods / interpolation",
     purpose="The coefficients f[x0], f[x0,x1], ..., f[x0..xn] of the Newton form of the interpolating polynomial.",
     inputs=[_I("x", "vector", "x values (distinct)"), _I("y", "vector", "y values")],
     outputs=[_O("coefficients", "divided differences f[x0], f[x0,x1], ...: p(t) = c0 + c1 (t - x0) + c2 (t - x0)(t - x1) + ...")],
     primary="coefficients", conditions=["distinct x values", "x and y of equal length (at most 60)"],
     method="the divided-difference table, exact decimal arithmetic", latex=r"f[x_i,\ldots,x_{i+k}] = \frac{f[x_{i+1},\ldots,x_{i+k}] - f[x_i,\ldots,x_{i+k-1}]}{x_{i+k} - x_i}",
     run=divided_differences, verify=_recompute(divided_differences, ["coefficients"]), reference=_reference_divdiff,
     wrong=lambda x, y: (lambda col: {"coefficients": [col[0]] + [
         (lambda c: c[0])([sum((-1) ** (k - j) * math.comb(k, j) * y[j] for j in range(k + 1))]) for k in range(1, len(y))]})(list(map(float, y))),
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1]})(_random_table(rng, rng.randint(3, 6))),
     parse=_safe_parse(_parse_interp((r"\bdivided differences?\b", r"\bnewton (?:form|interpolat)"), need_at=False)),
     templates=["Find the Newton divided differences for the points (1, 1), (2, 4), (4, 16).",
                "Compute the divided differences of the data x = (0, 1, 3), y = (1, 2, 10)."],
     examples=[{"inputs": {"x": "1, 2, 4", "y": "1, 4, 16"}, "outputs": {"coefficients": [1.0, 3.0, 1.0]}},
               {"inputs": {"x": [0, 1, 3], "y": [1, 2, 10]}, "outputs": {"coefficients": [1.0, 1.0, 1.0]}}],
     questions=[{"q": "Find the Newton divided differences for the points (1, 1), (2, 4), (4, 16).", "outputs": {"coefficients": [1.0, 3.0, 1.0]}},
                {"q": "Compute the divided differences of the data x = (0, 1, 3), y = (1, 2, 10).", "outputs": {"coefficients": [1.0, 1.0, 1.0]}}])


def interpolating_polynomial(x, y):
    X, Y = _table(x, y, min_n=1)
    if len(X) > 15:
        raise ValueError("at most 15 points (the coefficients of a higher-degree interpolant are ill conditioned)")
    V = [[t ** k for k in range(len(X))] for t in X]
    return {"coefficients": _fl(_solve_F(V, Y))}


def _reference_interp_poly(x, y):
    import sympy as sp
    t = sp.Symbol("t")
    pts = [(sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))) for a, b in zip(x, y)]
    p = sp.Poly(sp.interpolate(pts, t), t)
    return {"coefficients": [float(p.coeff_monomial(t ** k)) for k in range(len(pts))]}


_add("interpolating_polynomial",
     name="Coefficients of the interpolating polynomial", topic="numerical methods / interpolation",
     purpose="The polynomial c0 + c1 x + ... + c(n-1) x^(n-1) passing exactly through n data points.",
     inputs=[_I("x", "vector", "x values (distinct)"), _I("y", "vector", "y values")],
     outputs=[_O("coefficients", "[c0, c1, ...], lowest power first")], primary="coefficients",
     conditions=["distinct x values", "at most 15 points"],
     method="solve the Vandermonde system V c = y in exact rational arithmetic", latex=r"\sum_k c_k x_i^k = y_i",
     run=interpolating_polynomial,
     verify=lambda i, o: max(abs(math.fsum(c * t ** k for k, c in enumerate(o["coefficients"])) - yy) / max(1.0, math.fsum(abs(c * t ** k) for k, c in enumerate(o["coefficients"])))
                             for t, yy in zip(i["x"], i["y"])) if len(o["coefficients"]) == len(i["x"]) else BAD,
     reference=_reference_interp_poly,
     wrong=lambda x, y: {"coefficients": interpolating_polynomial(x, y)["coefficients"][::-1]},
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1]})(_random_table(rng, rng.randint(2, 6))),
     parse=_safe_parse(_parse_interp((r"\binterpolating polynomial\b", r"\bpolynomial (?:that )?(?:passes|passing|through)\b"), excl=(r"\blagrange\b", r"\bnewton\b", r"\bdivided\b", r"\bfit\b", r"\bleast"), need_at=False)),
     templates=["Find the interpolating polynomial through the points (0, 1), (1, 3), (2, 7).",
                "Find the polynomial passing through the points (-1, 2), (0, 1), (1, 2)."],
     examples=[{"inputs": {"x": "0, 1, 2", "y": "1, 3, 7"}, "outputs": {"coefficients": [1.0, 1.0, 1.0]}},
               {"inputs": {"x": [-1, 0, 1], "y": [2, 1, 2]}, "outputs": {"coefficients": [1.0, 0.0, 1.0]}}],
     questions=[{"q": "Find the interpolating polynomial through the points (0, 1), (1, 3), (2, 7).", "outputs": {"coefficients": [1.0, 1.0, 1.0]}},
                {"q": "Find the polynomial passing through the points (-1, 2), (0, 1), (1, 2).", "outputs": {"coefficients": [1.0, 0.0, 1.0]}}])


def linear_interpolation(x, y, at):
    X, Y = _table(x, y, increasing=True)
    t = _F(at, "at")
    if not X[0] <= t <= X[-1]:
        raise ValueError("the point must lie within the range of the x values (no extrapolation)")
    k = max(j for j in range(len(X) - 1) if X[j] <= t) if t < X[-1] else len(X) - 2
    return {"value": float(Y[k] + (Y[k + 1] - Y[k]) * (t - X[k]) / (X[k + 1] - X[k]))}


_add("linear_interpolation",
     name="Piecewise linear interpolation", topic="numerical methods / interpolation",
     purpose="The value at a point of the broken line joining consecutive data points.",
     inputs=[_I("x", "vector", "x values, strictly increasing"), _I("y", "vector", "y values"), _I("at", "number", "the point (within the data range)")],
     outputs=[_O("value", "interpolated value")], primary="value",
     conditions=["x strictly increasing", "x[0] <= at <= x[-1] (no extrapolation)"],
     method="locate the interval containing the point and interpolate linearly (exact)", latex=r"y = y_k + (y_{k+1}-y_k)\frac{t - x_k}{x_{k+1} - x_k}",
     run=linear_interpolation, verify=_recompute(linear_interpolation, ["value"]),
     reference=lambda x, y, at: {"value": float(__import__("numpy").interp(at, x, y))},
     wrong=lambda x, y, at: {"value": float(y[min(range(len(x)), key=lambda j: abs(x[j] - at))])},
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "at": round(rng.uniform(min(xy[0]), max(xy[0])), 3)})(_random_table(rng, rng.randint(2, 7))),
     parse=_safe_parse(_parse_interp((r"\blinear(?:ly)? interpolat", r"\binterpolate linearly\b"))),
     templates=["Use linear interpolation on the points (0, 0), (1, 2), (3, 3) to estimate the value at x = 2.",
                "Interpolate linearly in the table x = (10, 20, 30), y = (5, 9, 4) at 25."],
     examples=[{"inputs": {"x": "0, 1, 3", "y": "0, 2, 3", "at": 2}, "outputs": {"value": 2.5}},
               {"inputs": {"x": [10, 20, 30], "y": [5, 9, 4], "at": 25}, "outputs": {"value": 6.5}}],
     questions=[{"q": "Use linear interpolation on the points (0, 0), (1, 2), (3, 3) to estimate the value at x = 2.", "outputs": {"value": 2.5}},
                {"q": "Interpolate linearly in the table x = (10, 20, 30), y = (5, 9, 4) at 25.", "outputs": {"value": 6.5}},
                {"q": "Interpolate linearly in the table x = (10, 20, 30), y = (5, 9, 4) at 35.", "outcome": "INVALID_INPUT"}])


def _spline_value(X, Y, t, s0=None, sn=None):
    """Cubic spline through (X, Y) (exact): natural if s0, sn are None, else clamped end slopes; its value at t."""
    n = len(X) - 1
    h = [X[i + 1] - X[i] for i in range(n)]
    M = [[Fraction(0)] * (n + 1) for _ in range(n + 1)]
    r = [Fraction(0)] * (n + 1)
    if s0 is None:
        M[0][0], M[n][n] = Fraction(1), Fraction(1)
    else:
        M[0][0], M[0][1] = 2 * h[0], h[0]
        r[0] = 6 * ((Y[1] - Y[0]) / h[0] - s0)
        M[n][n - 1], M[n][n] = h[n - 1], 2 * h[n - 1]
        r[n] = 6 * (sn - (Y[n] - Y[n - 1]) / h[n - 1])
    for i in range(1, n):
        M[i][i - 1], M[i][i], M[i][i + 1] = h[i - 1], 2 * (h[i - 1] + h[i]), h[i]
        r[i] = 6 * ((Y[i + 1] - Y[i]) / h[i] - (Y[i] - Y[i - 1]) / h[i - 1])
    m = _solve_F(M, r)
    k = max(j for j in range(n) if X[j] <= t) if t < X[-1] else n - 1
    a, b = X[k + 1] - t, t - X[k]
    return m[k] * a ** 3 / (6 * h[k]) + m[k + 1] * b ** 3 / (6 * h[k]) + (Y[k] / h[k] - m[k] * h[k] / 6) * a + (Y[k + 1] / h[k] - m[k + 1] * h[k] / 6) * b


def natural_cubic_spline(x, y, at):
    X, Y = _table(x, y, min_n=3, increasing=True)
    t = _F(at, "at")
    if not X[0] <= t <= X[-1]:
        raise ValueError("the point must lie within the range of the x values (no extrapolation)")
    return {"value": float(_spline_value(X, Y, t))}


def clamped_cubic_spline(x, y, slope_start, slope_end, at):
    X, Y = _table(x, y, min_n=2, increasing=True)
    t = _F(at, "at")
    if not X[0] <= t <= X[-1]:
        raise ValueError("the point must lie within the range of the x values (no extrapolation)")
    return {"value": float(_spline_value(X, Y, t, _F(slope_start, "slope_start"), _F(slope_end, "slope_end")))}


def _parse_spline(clamped):
    def p(q):
        if not q.has(r"\bcubic spline\b", r"\bspline\b"):
            return None
        if clamped != bool(q.has(r"\bclamped\b", r"\bend slopes?\b", r"\bslope_start\b")):
            return None
        if not clamped and q.has(r"\bnot-a-knot\b", r"\bperiodic\b"):
            return None
        base = _parse_interp((r"\bspline\b",))(q)
        if base is None:
            return None
        if clamped:
            s0, s1 = q.named("slope_start"), q.named("slope_end")
            if s0 is None or s1 is None:
                m = re.search(r"\bend slopes?\s+(" + _N + r")\s+and\s+(" + _N + r")", q.rest)
                if not m:
                    return None
                s0, s1 = float(m.group(1)), float(m.group(2))
            base.update(slope_start=s0, slope_end=s1)
        return base
    return p


_add("natural_cubic_spline",
     name="Natural cubic spline interpolation", topic="numerical methods / splines",
     purpose="The value at a point of the natural cubic spline through the data (second derivative zero at both ends).",
     inputs=[_I("x", "vector", "knots, strictly increasing (at least 3)"), _I("y", "vector", "values at the knots"), _I("at", "number", "the point (within the knots)")],
     outputs=[_O("value", "spline value S(at)")], primary="value",
     conditions=["x strictly increasing, at least 3 knots (at most 60)", "x[0] <= at <= x[-1]", "natural end conditions S''(x0) = S''(xn) = 0"],
     method="solve the tridiagonal system for the knot second derivatives exactly, then evaluate the cubic piece",
     latex=r"h_{i-1}M_{i-1} + 2(h_{i-1}+h_i)M_i + h_iM_{i+1} = 6\Big(\frac{y_{i+1}-y_i}{h_i} - \frac{y_i-y_{i-1}}{h_{i-1}}\Big),\ M_0 = M_n = 0",
     run=natural_cubic_spline, verify=_recompute(natural_cubic_spline, ["value"]),
     reference=lambda x, y, at: {"value": float(__import__("scipy.interpolate", fromlist=["CubicSpline"]).CubicSpline(x, y, bc_type="natural")(at))},
     wrong=lambda x, y, at: clamped_cubic_spline(x, y, 0, 0, at),
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "at": round(rng.uniform(min(xy[0]), max(xy[0])), 3)})(_random_table(rng, rng.randint(3, 8))),
     parse=_safe_parse(_parse_spline(False)),
     templates=["Use a natural cubic spline through the points (0, 0), (1, 1), (2, 0) to estimate the value at x = 0.5.",
                "Evaluate the natural cubic spline of the data x = (0, 1, 2, 3), y = (0, 1, 8, 27) at 1.5."],
     examples=[{"inputs": {"x": "0, 1, 2", "y": "0, 1, 0", "at": 0.5}, "outputs": {"value": 0.6875}},
               {"inputs": {"x": [0, 1, 2, 3], "y": [0, 1, 8, 27], "at": 1.5}, "outputs": {"value": 3.15}}],
     questions=[{"q": "Use a natural cubic spline through the points (0, 0), (1, 1), (2, 0) to estimate the value at x = 0.5.", "outputs": {"value": 0.6875}},
                {"q": "Evaluate the natural cubic spline of the data x = (0, 1, 2, 3), y = (0, 1, 8, 27) at 1.5.", "outputs": {"value": 3.15}}])

_add("clamped_cubic_spline",
     name="Clamped cubic spline interpolation", topic="numerical methods / splines",
     purpose="The value at a point of the cubic spline through the data with prescribed slopes at the two ends.",
     inputs=[_I("x", "vector", "knots, strictly increasing"), _I("y", "vector", "values at the knots"),
             _I("slope_start", "number", "slope S'(x0)"), _I("slope_end", "number", "slope S'(xn)"), _I("at", "number", "the point (within the knots)")],
     outputs=[_O("value", "spline value S(at)")], primary="value",
     conditions=["x strictly increasing, at least 2 knots (at most 60)", "x[0] <= at <= x[-1]"],
     method="solve the tridiagonal system with clamped end equations for the knot second derivatives exactly, then evaluate the cubic piece",
     latex=r"2h_0M_0 + h_0M_1 = 6\Big(\frac{y_1-y_0}{h_0} - s_0\Big),\ h_{n-1}M_{n-1} + 2h_{n-1}M_n = 6\Big(s_n - \frac{y_n-y_{n-1}}{h_{n-1}}\Big)",
     run=clamped_cubic_spline, verify=_recompute(clamped_cubic_spline, ["value"]),
     reference=lambda x, y, slope_start, slope_end, at: {"value": float(__import__("scipy.interpolate", fromlist=["CubicSpline"]).CubicSpline(
         x, y, bc_type=((1, float(slope_start)), (1, float(slope_end))))(at))},
     wrong=lambda x, y, slope_start, slope_end, at: natural_cubic_spline(x, y, at) if len(x) >= 3 else clamped_cubic_spline(x, y, 0, 0, at),
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1], "slope_start": _rnum(rng), "slope_end": _rnum(rng),
                                            "at": round(rng.uniform(min(xy[0]), max(xy[0])), 3)})(_random_table(rng, rng.randint(3, 8))),
     parse=_safe_parse(_parse_spline(True)),
     templates=["Use a clamped cubic spline through the points (0, 0), (1, 1), (2, 0) with end slopes 1 and -1 to estimate the value at x = 0.5.",
                "Evaluate the clamped cubic spline of x = (0, 1, 2), y = (0, 1, 8) with slope_start = 0 and slope_end = 12 at 1.5."],
     examples=[{"inputs": {"x": "0, 1, 2", "y": "0, 1, 0", "slope_start": 1, "slope_end": -1, "at": 0.5}, "outputs": {"value": 0.625}},
               {"inputs": {"x": [0, 1, 2], "y": [0, 1, 8], "slope_start": 0, "slope_end": 12, "at": 1.5}, "outputs": {"value": 3.375}}],
     questions=[{"q": "Use a clamped cubic spline through the points (0, 0), (1, 1), (2, 0) with end slopes 1 and -1 to estimate the value at x = 0.5.",
                 "outputs": {"value": 0.625}}])


def trapezoid_rule(x, y):
    X, Y = _table(x, y, increasing=True)
    return {"integral": float(sum(((X[i + 1] - X[i]) * (Y[i] + Y[i + 1]) / 2 for i in range(len(X) - 1)), Fraction(0)))}


def simpson_rule(x, y):
    X, Y = _table(x, y, min_n=3, increasing=True)
    if len(X) % 2 == 0:
        raise ValueError("Simpson's rule needs an odd number of points (an even number of intervals)")
    h = X[1] - X[0]
    if any(X[i + 1] - X[i] != h for i in range(len(X) - 1)):
        raise ValueError("Simpson's rule here needs equally spaced x values")
    s = Y[0] + Y[-1] + 4 * sum(Y[1:-1:2], Fraction(0)) + 2 * sum(Y[2:-1:2], Fraction(0))
    return {"integral": float(h * s / 3)}


def _parse_quad(words, excl=()):
    def p(q):
        if not q.has(*words) or q.has(*excl) or q.mats:
            return None
        if q.has(r"\bf\(x\)\s*=", r"\bp\(x\)\s*=") or _poly_from_question(q):
            return None
        return _parse_interp(words, need_at=False)(q)
    return p


def _random_equal_table(rng, odd=False):
    n = rng.randint(1, 5) * 2 + 1 if odd else rng.randint(2, 9)
    h = rng.choice([1, 0.5, 0.25, 2, 0.1])
    x0 = rng.randint(-5, 5)
    xs = [float(_F(x0) + k * _F(h)) for k in range(n)]
    return {"x": xs, "y": [round(rng.uniform(-10, 10), 2) for _ in xs]}


_add("trapezoid_rule",
     name="Trapezoidal rule for tabulated data", topic="numerical methods / quadrature",
     purpose="The integral of tabulated data by the composite trapezoidal rule (any spacing).",
     inputs=[_I("x", "vector", "x values, strictly increasing"), _I("y", "vector", "y values")],
     outputs=[_O("integral", "trapezoidal estimate of the integral")], primary="integral",
     conditions=["x strictly increasing", "at least 2 points (at most 60)"],
     method="sum of (x_(i+1) - x_i)(y_i + y_(i+1))/2, exact decimal arithmetic", latex=r"\int y\,dx \approx \sum_i \frac{x_{i+1}-x_i}{2}(y_i + y_{i+1})",
     run=trapezoid_rule, verify=_recompute(trapezoid_rule, ["integral"]),
     reference=lambda x, y: {"integral": float(__import__("scipy.integrate", fromlist=["trapezoid"]).trapezoid(y, x))},
     wrong=lambda x, y: {"integral": math.fsum((x[i + 1] - x[i]) * (y[i] + y[i + 1]) for i in range(len(x) - 1))},
     random_inputs=lambda rng: (lambda xy: {"x": xy[0], "y": xy[1]})(_random_table(rng, rng.randint(2, 8))),
     parse=_safe_parse(_parse_quad((r"\btrapezoid(?:al)?\b",))),
     templates=["Use the trapezoidal rule to integrate the data (0, 1), (1, 3), (2, 2), (4, 0).",
                "Estimate the integral of the table x = (0, 0.5, 1), y = (1, 2, 4) by the trapezoid rule."],
     examples=[{"inputs": {"x": "0, 1, 2, 4", "y": "1, 3, 2, 0"}, "outputs": {"integral": 6.5}},
               {"inputs": {"x": [0, 0.5, 1], "y": [1, 2, 4]}, "outputs": {"integral": 2.25}}],
     questions=[{"q": "Use the trapezoidal rule to integrate the data (0, 1), (1, 3), (2, 2), (4, 0).", "outputs": {"integral": 6.5}},
                {"q": "Estimate the integral of the table x = (0, 0.5, 1), y = (1, 2, 4) by the trapezoid rule.", "outputs": {"integral": 2.25}}])

_add("simpson_rule",
     name="Simpson's rule for tabulated data", topic="numerical methods / quadrature",
     purpose="The integral of equally spaced tabulated data by the composite Simpson 1/3 rule.",
     inputs=[_I("x", "vector", "equally spaced, increasing x values (odd count)"), _I("y", "vector", "y values")],
     outputs=[_O("integral", "Simpson estimate of the integral")], primary="integral",
     conditions=["x equally spaced and increasing", "an odd number of points, at least 3 (an even number of intervals)"],
     method="h/3 (y_0 + 4 y_1 + 2 y_2 + 4 y_3 + ... + y_n), exact decimal arithmetic", latex=r"\int y\,dx \approx \frac{h}{3}\Big(y_0 + 4\sum_{\text{odd}} y_i + 2\sum_{\text{even}} y_i + y_n\Big)",
     run=simpson_rule, verify=_recompute(simpson_rule, ["integral"]),
     reference=lambda x, y: {"integral": float(__import__("scipy.integrate", fromlist=["simpson"]).simpson(y, x=x))},
     wrong=lambda x, y: trapezoid_rule(x, y),
     random_inputs=lambda rng: _random_equal_table(rng, odd=True),
     parse=_safe_parse(_parse_quad((r"\bsimpson'?s?\b",))),
     templates=["Use Simpson's rule to integrate the data (0, 1), (1, 2), (2, 5).", "Estimate the integral of the table x = (0, 0.5, 1, 1.5, 2), y = (0, 0.25, 1, 2.25, 4) by Simpson's rule."],
     examples=[{"inputs": {"x": "0, 1, 2", "y": "1, 2, 5"}, "outputs": {"integral": 14 / 3}},
               {"inputs": {"x": [0, 0.5, 1, 1.5, 2], "y": [0, 0.25, 1, 2.25, 4]}, "outputs": {"integral": 8 / 3}}],
     questions=[{"q": "Use Simpson's rule to integrate the data (0, 1), (1, 2), (2, 5).", "outputs": {"integral": 14 / 3}},
                {"q": "Use Simpson's rule to integrate the data (0, 1), (1, 2), (2, 5), (3, 1).", "outcome": "INVALID_INPUT"}])


def _poly_quad_inputs(rng, nmax=8):
    a = rng.randint(-5, 4)
    return {"coefficients": _rvec(rng, rng.randint(1, 6)), "a": a, "b": a + rng.choice([1, 2, 0.5, 3, 2.5]), "n": rng.randint(1, nmax)}


def midpoint_rule(coefficients, a, b, n):
    c = _need_poly(coefficients)
    _finite(a, "a"), _finite(b, "b")
    if not a < b:
        raise ValueError("the interval must have a < b")
    if isinstance(n, bool) or n != int(n) or not 1 <= n <= 1000:
        raise ValueError("n must be a whole number from 1 to 1000")
    A, B, N = _F(a), _F(b), int(n)
    h = (B - A) / N
    C = _FV(c)
    pv = lambda t: sum((cc * t ** (len(C) - 1 - k) for k, cc in enumerate(C)), Fraction(0))
    return {"integral": float(h * sum((pv(A + (k + Fraction(1, 2)) * h) for k in range(N)), Fraction(0)))}


def _parse_poly_quad(words, gauss=False):
    def p(q):
        if not q.has(*words) or q.mats:
            return None
        poly = _poly_from_question(q)
        iv = _parse_interval(q) or (lambda m: [float(m.group(1)), float(m.group(2))] if m else None)(
            re.search(r"\bfrom\s+(" + _N + r")\s+to\s+(" + _N + r")", q.rest))
        m = re.search(r"\b(\d+)\s+(?:sub)?intervals\b", q.rest) or re.search(r"\b(\d+)[- ]points?\b", q.rest)
        n = float(m.group(1)) if m else q.named("n")
        if poly is None or iv is None or n is None:
            return None
        return {"coefficients": poly, "a": iv[0], "b": iv[1], "n": n}
    return p


_QUAD_IN = [_POLY_IN, _I("a", "number", "lower limit"), _I("b", "number", "upper limit")]

_add("midpoint_rule",
     name="Composite midpoint rule for a polynomial", topic="numerical methods / quadrature",
     purpose="Approximate the integral of a polynomial over [a, b] by the composite midpoint rule with n subintervals.",
     inputs=_QUAD_IN + [_I("n", "integer", "number of subintervals", min=1, max=1000)],
     outputs=[_O("integral", "midpoint-rule estimate")], primary="integral",
     conditions=["a < b", "1 <= n <= 1000"], method="h sum p(a + (k + 1/2) h), h = (b - a)/n, exact decimal arithmetic",
     latex=r"\int_a^b p \approx h\sum_{k=0}^{n-1} p\big(a + (k+\tfrac12)h\big)",
     run=midpoint_rule, verify=_recompute(midpoint_rule, ["integral"]),
     reference=lambda coefficients, a, b, n: {"integral": math.fsum(float(__import__("scipy.integrate", fromlist=["fixed_quad"]).fixed_quad(
         lambda t: __import__("numpy").polyval(coefficients, t), a + k * (b - a) / int(n), a + (k + 1) * (b - a) / int(n), n=1)[0]) for k in range(int(n)))},
     wrong=lambda coefficients, a, b, n: {"integral": (b - a) / int(n) * math.fsum(_peval(list(coefficients), a + k * (b - a) / int(n)) for k in range(int(n)))},
     random_inputs=lambda rng: (lambda d: d if len(d["coefficients"]) > 1 else dict(d, coefficients=[1.0] + d["coefficients"]))(_poly_quad_inputs(rng)),
     parse=_safe_parse(_parse_poly_quad((r"\bmidpoint\b",))),
     templates=["Use the midpoint rule with 4 subintervals to integrate x^2 from 0 to 2.", "Approximate the integral of 3x^2 + 1 on [0, 1] by the midpoint rule with n = 2."],
     examples=[{"inputs": {"coefficients": "1, 0, 0", "a": 0, "b": 2, "n": 4}, "outputs": {"integral": 2.625}},
               {"inputs": {"coefficients": [3, 0, 1], "a": 0, "b": 1, "n": 2}, "outputs": {"integral": 1.9375}}],
     questions=[{"q": "Use the midpoint rule with 4 subintervals to integrate x^2 from 0 to 2.", "outputs": {"integral": 2.625}},
                {"q": "Approximate the integral of 3x^2 + 1 on [0, 1] by the midpoint rule with n = 2.", "outputs": {"integral": 1.9375}}])


def _legendre_nodes(n):
    xs, ws = [], []
    for i in range(1, n + 1):
        x = math.cos(math.pi * (i - 0.25) / (n + 0.5))
        for _ in range(100):
            p0, p1 = 1.0, x
            for k in range(2, n + 1):
                p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
            dp = n * (x * p1 - p0) / (x * x - 1) if n > 1 else 1.0
            if n == 1:
                p1, dp = x, 1.0
            dx = p1 / dp
            x -= dx
            if abs(dx) <= 1e-16:
                break
        p0, p1 = 1.0, x
        for k in range(2, n + 1):
            p0, p1 = p1, ((2 * k - 1) * x * p1 - (k - 1) * p0) / k
        dp = n * (x * p1 - p0) / (x * x - 1) if n > 1 else 1.0
        xs.append(x)
        ws.append(2 / ((1 - x * x) * dp * dp))
    order = sorted(range(n), key=lambda k: xs[k])
    xs, ws = [xs[k] for k in order], [ws[k] for k in order]
    for k in range(n // 2):                                  # exact symmetry
        m = (xs[n - 1 - k] - xs[k]) / 2
        w = (ws[k] + ws[n - 1 - k]) / 2
        xs[k], xs[n - 1 - k], ws[k], ws[n - 1 - k] = -m, m, w, w
    if n % 2:
        xs[n // 2] = 0.0
    return xs, ws


def gauss_legendre_rule(n):
    if isinstance(n, bool) or n != int(n) or not 1 <= n <= 40:
        raise ValueError("n must be a whole number from 1 to 40")
    xs, ws = _legendre_nodes(int(n))
    return {"nodes": xs, "weights": ws}


def _verify_gl(i, o):
    """The n-point rule integrates x^k exactly for k = 0..2n-1 (this determines the Gauss rule)."""
    n = int(i["n"])
    xs, ws = o["nodes"], o["weights"]
    if len(xs) != n or len(ws) != n or xs != sorted(xs):
        return BAD
    worst = 0.0
    for k in range(2 * n):
        exact = 0.0 if k % 2 else 2.0 / (k + 1)
        worst = max(worst, abs(math.fsum(w * x ** k for x, w in zip(xs, ws)) - exact) / max(exact, 1e-3 / (k + 1)))
    return worst


_add("gauss_legendre_rule",
     name="Gauss-Legendre quadrature nodes and weights", topic="numerical methods / quadrature",
     purpose="The nodes and weights of the n-point Gauss-Legendre rule on [-1, 1] (exact for polynomials of degree up to 2n - 1).",
     inputs=[_I("n", "integer", "number of points (1 to 40)", min=1, max=40)],
     outputs=[_O("nodes", "nodes x_i in [-1, 1], ascending"), _O("weights", "weights w_i")], primary="nodes",
     conditions=["1 <= n <= 40"], method="Newton's method on the Legendre polynomial P_n (three-term recurrence); w_i = 2 / ((1 - x_i^2) P_n'(x_i)^2)",
     latex=r"\int_{-1}^{1} f(x)\,dx \approx \sum_{i=1}^{n} w_i f(x_i),\quad P_n(x_i) = 0",
     run=gauss_legendre_rule, verify=_verify_gl,
     reference=lambda n: (lambda r: {"nodes": r[0].tolist(), "weights": r[1].tolist()})(__import__("numpy").polynomial.legendre.leggauss(int(n))),
     wrong=lambda n: (lambda N: {"nodes": [-1 + 2 * k / (N - 1) for k in range(N)] if N > 1 else [0.0],
                                 "weights": [2 / (N - 1) * (0.5 if k in (0, N - 1) else 1) for k in range(N)] if N > 1 else [2.0]})(int(n)),
     random_inputs=lambda rng: {"n": rng.randint(1, 20)},
     parse=_safe_parse(lambda q: (lambda m: {"n": float(m.group(1))} if m else ({"n": q.named("n")} if q.named("n") is not None else None))(
         re.search(r"\b(\d+)[- ]point\b", q.rest)) if q.has(r"\bgauss(?:ian)?[- ]legendre\b", r"\bgaussian quadrature\b") and q.has(r"\bnodes?\b", r"\bweights?\b", r"\babscissas?\b")
         and not q.has(r"\bintegra") else None),
     templates=["Find the nodes and weights of the 3-point Gauss-Legendre rule.", "What are the Gauss-Legendre nodes and weights for n = 2?"],
     examples=[{"inputs": {"n": 2}, "outputs": {"nodes": [-1 / math.sqrt(3), 1 / math.sqrt(3)], "weights": [1.0, 1.0]}},
               {"inputs": {"n": 3}, "outputs": {"nodes": [-math.sqrt(0.6), 0.0, math.sqrt(0.6)], "weights": [5 / 9, 8 / 9, 5 / 9]}}],
     questions=[{"q": "Find the nodes and weights of the 3-point Gauss-Legendre rule.", "outputs": {"nodes": [-math.sqrt(0.6), 0.0, math.sqrt(0.6)]}},
                {"q": "What are the Gauss-Legendre nodes and weights for n = 2?", "outputs": {"weights": [1.0, 1.0]}},
                {"q": "What are the Gauss-Legendre nodes and weights for n = 50?", "outcome": "INVALID_INPUT"}])


def gauss_legendre_integral(coefficients, a, b, n):
    c = _need_poly(coefficients)
    _finite(a, "a"), _finite(b, "b")
    if not a < b:
        raise ValueError("the interval must have a < b")
    xs, ws = gauss_legendre_rule(n).values()
    mid, half = (a + b) / 2, (b - a) / 2
    return {"integral": half * math.fsum(w * _peval(c, mid + half * x) for x, w in zip(xs, ws))}


_add("gauss_legendre_integral",
     name="Gauss-Legendre quadrature of a polynomial", topic="numerical methods / quadrature",
     purpose="The n-point Gauss-Legendre approximation of the integral of a polynomial over [a, b] (exact up to degree 2n - 1).",
     inputs=_QUAD_IN + [_I("n", "integer", "number of Gauss points (1 to 40)", min=1, max=40)],
     outputs=[_O("integral", "Gauss-Legendre estimate")], primary="integral",
     conditions=["a < b", "1 <= n <= 40"], method="map the n Gauss-Legendre nodes to [a, b]: (b - a)/2 sum w_i p((a + b)/2 + (b - a)/2 x_i)",
     latex=r"\int_a^b p(x)\,dx \approx \frac{b-a}{2}\sum_i w_i\,p\Big(\frac{a+b}{2} + \frac{b-a}{2}x_i\Big)",
     run=gauss_legendre_integral,
     verify=lambda i, o: (lambda w: abs(o["integral"] - w) / max(1.0, abs(w), (i["b"] - i["a"]) * max(abs(c) for c in i["coefficients"]) * max(1.0, abs(i["a"]), abs(i["b"])) ** (len(i["coefficients"]) - 1)))(
         _gl_check(i["coefficients"], i["a"], i["b"], int(i["n"]))),
     reference=lambda coefficients, a, b, n: {"integral": float(__import__("scipy.integrate", fromlist=["fixed_quad"]).fixed_quad(
         lambda t: __import__("numpy").polyval(coefficients, t), a, b, n=int(n))[0])},
     wrong=lambda coefficients, a, b, n: (lambda xs, ws: {"integral": math.fsum(w * _peval(list(coefficients), x) for x, w in zip(xs, ws))})(*_legendre_nodes(int(n))),
     random_inputs=lambda rng: _poly_quad_inputs(rng, 6),
     parse=_safe_parse(_parse_poly_quad((r"\bgauss(?:ian)?[- ]legendre\b", r"\bgaussian quadrature\b"))),
     templates=["Use 2-point Gauss-Legendre quadrature to integrate x^3 + x^2 from 0 to 2.", "Integrate x^4 on [-1, 1] by Gaussian quadrature with n = 3."],
     examples=[{"inputs": {"coefficients": "1, 1, 0, 0", "a": 0, "b": 2, "n": 2}, "outputs": {"integral": 4 + 8 / 3}},
               {"inputs": {"coefficients": [1, 0, 0, 0, 0], "a": -1, "b": 1, "n": 3}, "outputs": {"integral": 0.4}},
               {"inputs": {"coefficients": [1, 0, 0, 0, 0], "a": -1, "b": 1, "n": 2}, "outputs": {"integral": 2 / 9}}],
     questions=[{"q": "Use 2-point Gauss-Legendre quadrature to integrate x^3 + x^2 from 0 to 2.", "outputs": {"integral": 4 + 8 / 3}},
                {"q": "Integrate x^4 on [-1, 1] by Gaussian quadrature with n = 3.", "outputs": {"integral": 0.4}}])


def _gl_check(c, a, b, n):
    """The same rule evaluated by a second route: exact integral of the Lagrange interpolant at the Gauss nodes is
    not available in plain floats, so the nodes are mapped piecewise (two halves of the symmetric rule)."""
    xs, ws = _legendre_nodes(n)
    mid, half = (a + b) / 2, (b - a) / 2
    left = math.fsum(w * _peval(list(c), mid + half * x) for x, w in zip(xs, ws) if x < 0)
    right = math.fsum(w * _peval(list(c), mid + half * x) for x, w in zip(xs, ws) if x >= 0)
    return half * (left + right)


def numerical_derivative(y, h):
    y = _need_vec(y, "y", min_n=2, max_n=200)
    _finite(h, "h")
    if h <= 0:
        raise ValueError("the spacing h must be positive")
    Y, H = _FV(y), _F(h)
    n = len(Y)
    d = [(Y[1] - Y[0]) / H] + [(Y[i + 1] - Y[i - 1]) / (2 * H) for i in range(1, n - 1)] + [(Y[-1] - Y[-2]) / H]
    return {"derivative": _fl(d)}


_add("numerical_derivative",
     name="Derivative of equally spaced data (finite differences)", topic="numerical methods / differentiation",
     purpose="Estimate dy/dx at every sample of equally spaced data: central differences inside, one-sided differences at the ends.",
     inputs=[_I("y", "vector", "samples y_0, ..., y_n (at least 2)"), _I("h", "number", "spacing between samples")],
     outputs=[_O("derivative", "estimated dy/dx at each sample")], primary="derivative",
     conditions=["h > 0", "at least 2 samples (at most 200)"],
     method="(y_(i+1) - y_(i-1))/(2h) inside; (y_1 - y_0)/h and (y_n - y_(n-1))/h at the ends (exact decimal arithmetic)",
     latex=r"y'_i \approx \frac{y_{i+1} - y_{i-1}}{2h}",
     run=numerical_derivative, verify=_recompute(numerical_derivative, ["derivative"]),
     reference=lambda y, h: {"derivative": __import__("numpy").gradient(__import__("numpy").asarray(y, float), float(h)).tolist()},
     wrong=lambda y, h: {"derivative": [(y[i + 1] - y[i]) / h for i in range(len(y) - 1)] + [(y[-1] - y[-2]) / h]},
     random_inputs=lambda rng: {"y": _rvec(rng, rng.randint(3, 10)), "h": rng.choice([1, 0.5, 0.1, 2, 0.25])},
     parse=_safe_parse(lambda q: {"y": (_pick(q, "vector", "y") or q.vecs[0])["text"], "h": q.named("h")} if len(q.vecs) == 1 and not q.mats
                       and q.has(r"\bderivative\b", r"\bdifferentiate\b", r"\bfinite differences?\b") and q.named("h") is not None
                       and not _poly_from_question(q) else None),
     templates=["Estimate the derivative of the samples y = (0, 1, 4, 9, 16) with spacing h = 1.", "Differentiate the data [1, 2, 4, 8] numerically with h = 0.5."],
     examples=[{"inputs": {"y": "0, 1, 4, 9, 16", "h": 1}, "outputs": {"derivative": [1.0, 2.0, 4.0, 6.0, 7.0]}},
               {"inputs": {"y": [1, 2, 4, 8], "h": 0.5}, "outputs": {"derivative": [2.0, 3.0, 6.0, 8.0]}}],
     questions=[{"q": "Estimate the derivative of the samples y = (0, 1, 4, 9, 16) with spacing h = 1.", "outputs": {"derivative": [1.0, 2.0, 4.0, 6.0, 7.0]}},
                {"q": "Differentiate the data [1, 2, 4, 8] numerically with h = 0.", "outcome": "INVALID_INPUT"}])


# --- END OF PROCEDURES ---
