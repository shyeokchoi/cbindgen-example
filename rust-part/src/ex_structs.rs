pub mod ex_structs {
    struct StructA {
        pub structB: *const StructB,
    }

    struct StructB {
        _dummy: u8,
    }
}
