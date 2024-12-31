from __future__ import annotations
from typing import Any, Callable
from polars_qf import _quant_factors
import polars as pl


@pl.api.register_expr_namespace("qf")
class QuantFactors:
    def __init__(self, expr: pl.Expr):
        self._expr = expr

    def __getattr__(self, attr: str) -> Callable[..., pl.Expr]:
        if attr in _quant_factors.__all__:

            def func(*args: Any, **kwargs: Any) -> pl.Expr:
                return getattr(_quant_factors, attr)(self._expr, *args, **kwargs)

            return func
        raise AttributeError(f"{self.__class__} has no attribute {attr}")
