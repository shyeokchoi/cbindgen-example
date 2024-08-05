#include "include/bindings.h"
#include <stdio.h>
#include <vector>

int main()
{
    ffi::test_hello();

    std::vector<const char*> strings { "Hello,", "from", "C++" };
    auto strings_wrapped = ffi::VecWrapper<const char*> {
        .data = strings.data(),
        .len = strings.size(),
        .capacity = strings.capacity()
    };
    ffi::print_strings(&strings_wrapped);
}
