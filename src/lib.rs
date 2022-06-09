
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
// fn do_something() {
//     Python::with_gil(|py| {
//         let builtins = PyModule::import(py, "builtins")?;
//         let total: i32 = builtins.getattr("sum")?.call1((vec![1, 2, 3],))?.extract()?;
//         assert_eq!(total, 6);
//         Ok(())
//     });
// }

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}