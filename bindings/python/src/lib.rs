use pyo3::prelude::*;
// use pyo3_arrow::error::PyArrowResult;

#[pymodule]
mod rkshare {
    use std::time::Duration;

    use pyo3_async_runtimes::tokio::future_into_py;

    use super::*;

    #[pyfunction]
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    #[pyfunction]
    fn sleep(py: Python, seconds: u64) -> PyResult<Bound<PyAny>> {
        future_into_py(py, async move {
            tokio::time::sleep(Duration::from_secs(seconds)).await;
            Ok(())
        })
    }

    // #[pymodule_init]
    // fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
    //     Python::with_gil(|py| {
    //         py.import("sys")?
    //             .getattr("modules")?
    //             .set_item("rkshare.akshare", m)
    //     })
    // }
}
