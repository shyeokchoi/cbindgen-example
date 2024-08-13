# cbindgen-example
This is a demonstration of how to call Rust functions from C++ code using [cbindgen](https://github.com/mozilla/cbindgen).  
`cbindgen` is a tool that generates header files, enabling interoperability between Rust and C++.

`CMake` is used as the build system. You can explore the CMakeLists.txt file in the root of the repository to see how to:  

1. Build a static library (in Linux, a .a file) from the Rust code using Cargo.  
2. Generate a header file using cbindgen.   
3. Link the generated static library to create an executable.  

# How to Run
> This setup has only been tested on Linux.  
> It might not work on Windows, as the static library file format differs from Linux.  

```bash
Copy code
mkdir build
cd build
cmake ..
make
./cbindgen-example
```
