#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct Char16(u16);

#[repr(C)]
pub struct CString16(pub *const Char16);
