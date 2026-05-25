use ni_number::{clear_cache, ni_number_digits, NI_F32, NI_F64};
use pyo3::prelude::*;

/// Compute η_ν and return a decimal string with `n` digits after the point.
#[pyfunction]
fn ni_digits(n: u32) -> String {
    ni_number_digits(n)
}

/// Return η_ν as f64 (~15 digits, instant).
#[pyfunction]
fn ni_f64() -> f64 {
    NI_F64
}

/// Return η_ν as f32 (~7 digits, instant).
#[pyfunction]
fn ni_f32() -> f32 {
    NI_F32
}

/// Clear the internal cache.
#[pyfunction]
fn clear() {
    clear_cache();
}

/// Return the first 50 digits as a static string.
#[pyfunction]
fn ni_50_digits() -> &'static str {
    ni_number::NI_50_DIGITS
}

/// Python module definition.
#[pymodule]
fn ni_number_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ni_digits, m)?)?;
    m.add_function(wrap_pyfunction!(ni_f64, m)?)?;
    m.add_function(wrap_pyfunction!(ni_f32, m)?)?;
    m.add_function(wrap_pyfunction!(clear, m)?)?;
    m.add_function(wrap_pyfunction!(ni_50_digits, m)?)?;
    m.add("NI_F64", NI_F64)?;
    m.add("NI_F32", NI_F32)?;
    Ok(())
}
