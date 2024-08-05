#[repr(C)]
#[derive(Clone)]
pub struct VecWrapper<T> {
    pub data: *const T,
    pub len: usize,
    pub capacity: usize,
}

impl<T: std::clone::Clone> VecWrapper<T> {
    pub fn to_vec(&self) -> Vec<T> {
        let slice = unsafe { std::slice::from_raw_parts(self.data as *mut T, self.len) };
        slice.to_vec()
    }
}
