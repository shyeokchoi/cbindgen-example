cmake_minimum_required(VERSION 3.15.0)
project(
  cbindgen-example
  VERSION 0.1.0
  LANGUAGES CXX)

set(RUST_DIRECTORY "${CMAKE_SOURCE_DIR}/rust-part")
set(GENERATED_HDR_LOCATION "${CMAKE_BINARY_DIR}/include")
set(GENERATED_HDR_PATH "${GENERATED_HDR_LOCATION}/bindings.h")

# Build the Rust project
add_custom_target(
  build_rust_library
  COMMAND cargo build --release
  WORKING_DIRECTORY ${RUST_DIRECTORY})

# Generate header file using cbindgen
add_custom_target(
  generate_header ALL
  COMMAND cbindgen --config ${RUST_DIRECTORY}/cbindgen.toml --crate rust-part
          --output ${GENERATED_HDR_PATH}
  WORKING_DIRECTORY ${RUST_DIRECTORY}
  DEPENDS build_rust_library)

# format the header file with clang-format
add_custom_target(
  format_header ALL
  COMMAND clang-format -i *.h
  WORKING_DIRECTORY ${GENERATED_HDR_LOCATION})

# Specify the Rust Library File
set(RUST_LIB "${RUST_DIRECTORY}/target/release/librust_part.a")

# Add the C++ main file
add_executable(cbindgen-example main.cpp)

# Include the generated header files
target_include_directories(cbindgen-example PRIVATE ${CMAKE_BINARY_DIR}/include)

# Link the Rust library add_dependencies
target_link_libraries(cbindgen-example ${RUST_LIB})
