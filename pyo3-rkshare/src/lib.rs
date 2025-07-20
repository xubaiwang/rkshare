use std::sync::Arc;

use arrow::array::ArrayRef;
use pyo3::prelude::*;
use pyo3_arrow::{PyArray, error::PyArrowResult};

#[pymodule]
mod rkshare {
    use super::*;

    #[pymodule]
    mod akshare {
        use super::*;

        #[pyfunction]
        fn double(x: usize) -> usize {
            x * 2
        }

        #[pyfunction]
        pub fn take(py: Python, value: u32) -> PyArrowResult<PyObject> {
            let output_array = arrow::array::UInt32Array::from_iter([value; 3]);
            let array_ref: ArrayRef = Arc::new(output_array);
            let py_array = PyArray::from_array_ref(array_ref);
            Ok(py_array.to_arro3(py)?.unbind())
        }

        #[pymodule_init]
        fn init(m: &Bound<'_, PyModule>) -> PyResult<()> {
            Python::with_gil(|py| {
                py.import("sys")?
                    .getattr("modules")?
                    .set_item("rkshare.akshare", m)
            })
        }
    }
}
