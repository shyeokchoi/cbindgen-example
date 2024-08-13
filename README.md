# cbindgen-example
This is a demonstration of how to call Rust functions from C++ code using [cbindgen](https://github.com/mozilla/cbindgen).    

`cbindgen` is a tool that automatically generates header files, making it easier to integrate Rust with C++ code.  

By using `cbindgen`, developers can avoid the time-consuming and error-prone process of manually writing header files which work as a glue between Rust and C++.  

`CMake` is used as the build system.

# What is included in this example
## Rust Foreign Function Interface  
Basic `extern` functions and `repr[C]` structs.  

## CMakeLists.txt
You can explore the CMakeLists.txt file in the root of the repository to see how to:  

1. Build a static library (in Linux, a .a file) from the Rust code using Cargo.  
2. Generate a header file using cbindgen.   
3. Link the generated static library to create an executable.  

## Manipulating the generated header to avoid compile error
Since `cbindgen` doesn't currently add forward declarations, you might encounter compile errors as your header file becomes larger and more complex.  
To address this issue, you can find a workaround example, at `rust-part/build.rs`.  

# How to Run
> This setup has only been tested on Linux.  
> It might not work on Windows, as the static library file format differs from Linux.  

```bash
mkdir build
cd build
cmake ..
make
./cbindgen-example
```
