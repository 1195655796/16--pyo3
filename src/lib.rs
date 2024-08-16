use pyo3::prelude::*;
use roaring::RoaringBitmap;

#[pyclass]
pub struct RoaringBitmapWrapper {
    bitmap: RoaringBitmap,
}

#[pymethods]
impl RoaringBitmapWrapper {
    #[new]
    pub fn new() -> Self {
        RoaringBitmapWrapper {
            bitmap: RoaringBitmap::new(),
        }
    }

    pub fn add(&mut self, value: u32) {
        self.bitmap.insert(value);
    }

    pub fn contains(&self, value: u32) -> bool {
        self.bitmap.contains(value)
    }

    pub fn len(&self) -> usize {
        self.bitmap.len().try_into().expect("invalid length")
    }
}

#[pymodule]
fn roaring_bitmap_py(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<RoaringBitmapWrapper>()?;
    Ok(())
}