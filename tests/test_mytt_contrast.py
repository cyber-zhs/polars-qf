import pandas as pd
import polars as pl
import pytest
import polars_qf as qf

assert qf


def test_simple():
    val = [1.111, 2.222, 3.333, 4.444, 5.555, 6.666, 7.777, 8.888, 9.999, 10.0]
    close_series = pl.Series("S", val)
    df = pl.DataFrame([close_series])
    df = df.select(pl.col("S").qf.rd(1).cast(pl.String))
    rd_str = df["S"].to_list()
    assert rd_str == [
        "1.1",
        "2.2",
        "3.3",
        "4.4",
        "5.6",
        "6.7",
        "7.8",
        "8.9",
        "10.0",
        "10.0",
    ]


@pytest.mark.parametrize(
    "func_name,args",
    [
        ("rd", ()),
        ("rd", (2,)),
        ("ema", (5,)),
    ],
)
def test_single_series(df_single_float, func_name, args):
    import mytt

    mytt_func = getattr(mytt, func_name.upper())
    series_pd = pd.Series(df_single_float["S"].to_numpy())
    expect = mytt_func(series_pd, *args)
    output_expr = getattr(pl.col("S").qf, func_name)(*args)
    df_single = df_single_float.select(output_expr)
    result = df_single["S"].to_numpy()
    assert len(result) == len(expect)
    for i in range(len(result)):
        assert result[i] == pytest.approx(expect[i])
