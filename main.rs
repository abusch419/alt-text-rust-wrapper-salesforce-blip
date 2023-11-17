use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
fn get_caption(img_path: String) -> PyResult<String> {
    Python::with_gil(|py| {
        let sys = PyModule::import(py, "sys")?;
        sys.get("path")?.call_method1("append", ("/image_captioner",))?;
        let captioner = PyModule::import(py, "image_captioner")?;
        let caption = captioner.call1("get_caption", (img_path,))?;
        Ok(caption.extract()?)
    })
}
