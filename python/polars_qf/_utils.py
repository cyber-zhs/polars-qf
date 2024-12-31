from typing import TYPE_CHECKING, Union
from pathlib import Path

LIB = Path(__file__).parent

if TYPE_CHECKING:
    import sys

    import polars as pl

    if sys.version_info >= (3, 10):
        from typing import TypeAlias
    else:
        from typing_extensions import TypeAlias

    IntoExprColumn: TypeAlias = Union[pl.Expr, str, pl.Series]
