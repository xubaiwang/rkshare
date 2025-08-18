use pyo3::prelude::*;
// use pyo3_arrow::error::PyArrowResult;

#[pymodule]
mod rkshare {
    use super::*;

    #[pymodule]
    mod akshare {
        // use pyo3_arrow::PyRecordBatch;

        // use super::*;

        // #[pyfunction]
        // pub fn stock_sse_summary(py: Python) -> PyArrowResult<PyObject> {
        //     let rt = tokio::runtime::Builder::new_current_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap();
        //     let batch = rt.block_on(::rkshare::sse::stock::summary()).unwrap();
        //     let py_batch = PyRecordBatch::new(batch);
        //     Ok(py_batch.to_arro3(py)?.unbind())
        // }

        // #[pyfunction]
        // pub fn stock_sse_deal_daily(py: Python, date: &str) -> PyArrowResult<PyObject> {
        //     let rt = tokio::runtime::Builder::new_current_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap();
        //     let batch = rt
        //         .block_on(::rkshare::sse::stock::deal_daily(date))
        //         .unwrap();
        //     let py_batch = PyRecordBatch::new(batch);
        //     Ok(py_batch.to_arro3(py)?.unbind())
        // }

        // #[pyfunction]
        // pub fn stock_szse_summary(py: Python, date: &str) -> PyArrowResult<PyObject> {
        //     let rt = tokio::runtime::Builder::new_current_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap();
        //     let batch = rt.block_on(::rkshare::szse::stock::summary(date)).unwrap();
        //     let py_batch = PyRecordBatch::new(batch);
        //     Ok(py_batch.to_arro3(py)?.unbind())
        // }

        // #[pyfunction]
        // pub fn stock_szse_area_summary(py: Python, date: &str) -> PyArrowResult<PyObject> {
        //     let rt = tokio::runtime::Builder::new_current_thread()
        //         .enable_all()
        //         .build()
        //         .unwrap();
        //     let batch = rt
        //         .block_on(::rkshare::szse::stock::area_summary(date))
        //         .unwrap();
        //     let py_batch = PyRecordBatch::new(batch);
        //     Ok(py_batch.to_arro3(py)?.unbind())
        // }

        // #[pymodule_init]
        // fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
        //     Python::with_gil(|py| {
        //         py.import("sys")?
        //             .getattr("modules")?
        //             .set_item("rkshare.akshare", m)
        //     })
        // }
    }
}
