use pyo3::types::{PyModule, PyModuleMethods};
use pyo3::{Bound, PyResult, Python, pymodule};
use pyo3_polars::PolarsAllocator;

mod expressions;

#[global_allocator]
static ALLOC: PolarsAllocator = PolarsAllocator::new();

#[pymodule]
fn _internal(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
	m.add("__version__", env!("CARGO_PKG_VERSION"))?;
	Ok(())
}
