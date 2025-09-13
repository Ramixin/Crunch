from typing import Literal


def i() -> int: pass
def pi() -> float: pass

# ===============================
# Numeric Functions
# ===============================

def abs(x: float) -> float: pass
def floor(x: float) -> int: pass
def int_part(x: float) -> int: pass
def fracton_part(x: float) -> float: pass
def round(x: float, digits: int = 0) -> float: pass
def sqrt(x: float) -> float: pass
def cbrt(x: float) -> float: pass
def root(x: float, n: float) -> float: pass
def log(x: float) -> float: pass
def ln(x: float) -> float: pass
def exp(x: float) -> float: pass
def pow10(x: float) -> float: pass
def random() -> float: pass
def rand_int(low: int, high: int) -> int: pass
def rand_binomial(n: int, p: float, rep: int) -> int: pass
def permutation(n: int, r: int) -> int: pass
def combination(n: int, r: int) -> int: pass
def factorial(n: int) -> int: pass
def min(*args: float) -> float: pass
def max(*args: float) -> float: pass
def gcd(a: int, b: int) -> int: pass
def lcm(a: int, b: int) -> int: pass
def remainder(a: int, b: int) -> int: pass

# ===============================
# Trigonometry
# ===============================
def sin(x: float) -> float: pass
def cos(x: float) -> float: pass
def tan(x: float) -> float: pass
def arcsin(x: float) -> float: pass
def arccos(x: float) -> float: pass
def arctan(x: float) -> float: pass
def sinh(x: float) -> float: pass
def cosh(x: float) -> float: pass
def tanh(x: float) -> float: pass
def arcsinh(x: float) -> float: pass
def arccosh(x: float) -> float: pass
def arctanh(x: float) -> float: pass

# ===============================
# Complex Numbers
# ===============================
def conj(x: complex) -> complex: pass
def real(x: complex) -> float: pass
def imag(x: complex) -> float: pass
def angle(x: complex) -> float: pass
def magnitude(x: complex) -> float: pass

# ===============================
# List Operations
# ===============================
def dim(lst: list) -> int: pass
def augment(lst1: list, lst2: list) -> list: pass
def seq(expr, var: str, start: int, end: int, step: int = 1) -> list: pass
def cum_sum(lst: list) -> list: pass
def delta_list(lst: list) -> list: pass
def mean(lst: list) -> float: pass
def median(lst: list) -> float: pass
def sum(lst: list) -> float: pass
def prod(lst: list) -> float: pass
def std_dev(lst: list) -> float: pass
def variance(lst: list) -> float: pass
def sort_up(lst: list) -> list: pass
def sort_down(lst: list) -> list: pass
def min_val(lst: list) -> float: pass
def max_val(lst: list) -> float: pass

# ===============================
# Matrix Operations
# ===============================
def det(matrix: list[list]) -> float: pass
def identity(n: int) -> list[list]: pass
def ref(matrix: list[list]) -> list[list]: pass
def rref(matrix: list[list]) -> list[list]: pass
def transpose(matrix: list[list]) -> list[list]: pass
def inv(matrix: list[list]) -> list[list]: pass
def trace(matrix: list[list]) -> float: pass

# ===============================
# Probability & Stats
# ===============================
def normal_cdf(lower: float, upper: float, mean: float = 0, std: float = 1) -> float: pass
def inv_norm(p: float, mean: float = 0, std: float = 1) -> float: pass
def t_cdf(lower: float, upper: float, df: int) -> float: pass
def inv_t(p: float, df: int) -> float: pass
def chi2_cdf(lower: float, upper: float, df: int) -> float: pass
def f_cdf(lower: float, upper: float, df1: int, df2: int) -> float: pass
def binom_pdf(n: int, p: float, x: int) -> float: pass
def binom_cdf(n: int, p: float, x: int) -> float: pass
def poisson_pdf(lam: float, x: int) -> float: pass
def poisson_cdf(lam: float, x: int) -> float: pass
def geomet_pdf(p: float, x: int) -> float: pass
def geomet_cdf(p: float, x: int) -> float: pass
def normal_pdf(x: float, mean: float = 0, std: float = 1) -> float: pass
def t_pdf(x: float, df: int) -> float: pass
def chi2_pdf(x: float, df: int) -> float: pass
def f_pdf(x: float, df1: int, df2: int) -> float: pass

# ===============================
# I/O & Program Control
# ===============================
def input_int(msg: str = "") -> int: pass
def input_str(msg: str = "") -> str: pass
def prompt(*vars: str) -> None: pass
def disp(*args) -> None: pass
def output(row: int, col: int, value) -> None: pass
def clr_home() -> None: pass
def pause() -> None: pass
def stop() -> None: pass
def ret(val : int = 0) -> None: pass
def call() -> None: pass

# ===============================
# Graphics
# ===============================
def clr_draw() -> None: pass
def axes_mode(on_or_off : bool) -> None: pass
def grid_mode(on_or_off : bool) -> None: pass
def text(x: float, y: float, value: str) -> None: pass
def line(x1: float, y1: float, x2: float, y2: float) -> None: pass
def pt_on(x: float, y: float) -> None: pass
def pt_off(x: float, y: float) -> None: pass
def pt_toggle(x: float, y: float) -> None: pass
def circle(x: float, y: float, r: float) -> None: pass
def shade(y1: float, y2: float) -> None: pass
def plots_mode(on_or_off : bool) -> None: pass

class Matrix:
    def __init__(self, name: Literal["A", "B", "C", "D", "E", "F", "G", "H", "I", "J"]):
        self.name = name  # "A" through "J"

    def set_element(self, row: int, col: int, val: float):
        # Compiles to: val→[A](row,col)
        pass

    def get_element(self, row: int, col: int) -> float:
        # Compiles to: [A](row,col)
        pass

    def dimensions(self) -> tuple[int, int]:
        # Compiles to: dim([A])
        pass

    def determinant(self) -> float:
        # Compiles to: det([A])
        pass

    def inverse(self) -> "Matrix":
        # Compiles to: [A]^-1
        pass

class Complex:
    def __init__(self, real: float = 0.0, imag: float = 0.0):
        self.real = real
        self.imag = imag

def add(self, other: "Complex") -> "Complex":
    pass

def sub(self, other: "Complex") -> "Complex":
    pass

def mul(self, other: "Complex") -> "Complex":
    pass

def div(self, other: "Complex") -> "Complex":
    pass

def conjugate(self) -> "Complex":
    pass

def modulus(self) -> float:
    pass

def argument(self) -> float:
    pass

def __repr__(self) -> str:
    return f"TIComplex({self.real} + {self.imag}i)"