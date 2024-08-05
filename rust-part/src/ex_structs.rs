#[repr(C)]
pub struct StructA {
    _structB: *const StructB,
}

#[repr(C)]
pub struct StructB {
    _structA: *const StructA,
}
