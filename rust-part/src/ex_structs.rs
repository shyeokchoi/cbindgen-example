#[repr(C)]
pub struct StructA {
    _struct_b: *const StructB,
}

#[repr(C)]
pub struct StructB {
    _struct_a: *const StructA,
}
