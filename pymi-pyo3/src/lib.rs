use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Formats the difference of two numbers as a string.
#[pyfunction]
fn subtract_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}

#[pyclass]
struct User {
    name: String,
    age: u8,
    status: bool,
}

#[pyfunction]
fn christopher() -> PyResult<User> {
    Ok(User { name:"christopher".to_owned(), age:55, status: true })
}

/// A Python module implemented in Rust.
#[pymodule]
fn pymi_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(subtract_as_string, m)?)?;
    m.add_class::<User>()?;
    m.add_function(wrap_pyfunction!(christopher, m)?)?;
    Ok(())
}
