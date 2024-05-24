cmake_minimum_required(VERSION 3.24)
project(a_little_rust_in_cpp)

set(CMAKE_CXX_STANDARD 17)

include(FetchContent)

# Add Corrosion to the cmake environment
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

# Set the build type to static to build a static library (So you do not need the library file next to the executable to run it)
set(CORROSION_BUILD_TYPE "static")

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(MANIFEST_PATH rust_calculator/Cargo.toml)

# Add the executable
add_executable(a_little_rust_in_cpp main.cpp cpp_calculator.cpp rust_calculator.cpp)

# Link against the Rust library
target_link_libraries(a_little_rust_in_cpp PUBLIC rust_calculator)
