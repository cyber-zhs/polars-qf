mod base;

use pyo3::prelude::*;
use pyo3::pymodule;
use pyo3_polars::PolarsAllocator;

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();

#[pymodule(name = "_polars_qf_core")]
fn polars_qf(_m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
