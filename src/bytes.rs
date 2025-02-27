pub trait ToBytes {
    fn to_bytes(&self) -> &[u8];
}
impl<T> ToBytes for [T] {
    fn to_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr() as *const u8, size_of_val(self))
        }
    }
}
