# Quant Factor Plugin for Polars.

Add technical analysis and quant factor to polars.

## Quick Start

### Python
```python
import polars as pl
import polars_qf as qf
# avoid `qf` unused warnings
assert qf
val = [2.222, 4.444, 5.555, 6.666, 7.777]
df = pl.DataFrame([pl.Series("S", val)])
expr = pl.col("S").qf.rd(1)
df = df.select(expr)
```

### Rust & Js & R
not support yet, but in planning.

## Functions List
TODO