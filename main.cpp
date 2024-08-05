#include "include/bindings.h"
#include <stdio.h>
#include <vector>

ffi::VecWrapper<const char*> get_strings()
{
    std::vector<const char*> strings { "Hello,", "from", "C++" };

    return ffi::VecWrapper<const char*> {
        .data = strings.data(),
        .len = strings.size(),
        .capacity = strings.capacity()
    };
}

int main()
{
    ffi::test_hello();

    auto strings = get_strings();
    ffi::print_strings(&strings);
}
