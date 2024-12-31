use polars::export::num::Pow;
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_plan::prelude::FieldsMapper;
use serde::Deserialize;

fn keep_dtype_output(input_fields: &[Field]) -> PolarsResult<Field> {
    FieldsMapper::new(input_fields).map_dtype(|dtype| match dtype {
        DataType::Decimal(a, b) => DataType::Decimal(*a, *b),
        DataType::Float32 => DataType::Float32,
        _ => DataType::Float64,
    })
}

fn float_dtype_output(input_fields: &[Field]) -> PolarsResult<Field> {
    FieldsMapper::new(input_fields).map_dtype(|dtype| match dtype {
        DataType::Float32 => DataType::Float32,
        _ => DataType::Float64,
    })
}

#[derive(Deserialize)]
struct RdKwargs {
    decimals: u32,
}

/// Rounds half-way cases to the number with an even least significant digit. Same with numpy's behavior.
/// Ref: https://github.com/pola-rs/polars/blob/rs-0.44.2/crates/polars-ops/src/series/ops/round.rs#L9
#[polars_expr(output_type_func=keep_dtype_output)]
fn rd(inputs: &[Series], kwargs: RdKwargs) -> PolarsResult<Series> {
    let s = &inputs[0];
    let decimals = kwargs.decimals;
    if let Ok(ca) = s.f32() {
        return if decimals == 0 {
            let s = ca.apply_values(|val| val.round_ties_even()).into_series();
            Ok(s)
        } else {
            let multiplier = 10f32.pow(decimals as f32);
            let s = ca
                .apply_values(|val| (val * multiplier).round_ties_even() / multiplier)
                .into_series();
            Ok(s)
        };
    }
    if let Ok(ca) = s.f64() {
        return if decimals == 0 {
            let s = ca.apply_values(|val| val.round_ties_even()).into_series();
            Ok(s)
        } else {
            let multiplier = 10f64.pow(decimals as f64);
            let s = ca
                .apply_values(|val| (val * multiplier).round_ties_even() / multiplier)
                .into_series();
            Ok(s)
        };
    }
    if let Some(ca) = s.try_decimal() {
        let precision = ca.precision();
        let scale = ca.scale() as u32;
        if scale <= decimals {
            return Ok(ca.clone().into_series());
        }

        let decimal_delta = scale - decimals;
        let multiplier = 10i128.pow(decimal_delta);
        let threshold = multiplier / 2;

        let ca = ca
            .apply_values(|v| {
                // We use rounding=ROUND_HALF_EVEN
                let rem = v % multiplier;
                let is_v_floor_even = ((v - rem) / multiplier) % 2 == 0;
                let threshold = threshold + i128::from(is_v_floor_even);
                let round_offset = if rem.abs() >= threshold {
                    multiplier
                } else {
                    0
                };
                let round_offset = if v < 0 { -round_offset } else { round_offset };
                v - rem + round_offset
            })
            .into_decimal_unchecked(precision, scale as usize);

        return Ok(ca.into_series());
    }
    polars_ensure!(s.dtype().is_numeric(), InvalidOperation: "qf.rd can only be used on numeric types" );
    Ok(s.clone())
}

#[derive(Deserialize)]
struct OneKwarg {
    n: u32,
}
#[polars_expr(output_type_func=float_dtype_output)]
fn ema(inputs: &[Series], kwargs: OneKwarg) -> PolarsResult<Series> {
    let s = &inputs[0];
    let options = EWMOptions::default()
        .and_span(kwargs.n as usize)
        .and_adjust(false);
    ewm_mean(s, options)
}
