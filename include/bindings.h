/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace ffi {

struct StructA;
struct StructB;

template<typename T>
struct VecWrapper
{
    const T *data;
    size_t len;
    size_t capacity;
};

struct StructB
{
    const StructA *_struct_a;
};

struct StructA
{
    const StructB *_struct_b;
};

extern "C" {

void test_hello();

void print_strings_from_value(VecWrapper<const char*> strings);

void print_strings_from_pointer(const VecWrapper<const char*> *strings);

void _dummy(const StructA *a, const StructB *b);

} // extern "C"

} // namespace ffi
