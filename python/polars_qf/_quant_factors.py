from __future__ import annotations
from typing import TYPE_CHECKING
import polars as pl
from polars.plugins import register_plugin_function
from ._utils import LIB

if TYPE_CHECKING:
    from ._utils import IntoExprColumn

__all__ = ["rd", "ema"]


def rd(expr: IntoExprColumn, decimals: int = 3) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        args=[expr],
        function_name="rd",
        is_elementwise=True,
        kwargs={"decimals": decimals},
    )


def ema(expr: IntoExprColumn, n: int) -> pl.Expr:
    return register_plugin_function(
        plugin_path=LIB,
        args=[expr],
        function_name="ema",
        is_elementwise=True,
        kwargs={"n": n},
    )
