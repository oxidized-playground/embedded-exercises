fn main() {
    // Tell cargo to rerun whenever there was a change
    println!("cargo:rerun-if-changed=c_calculator/");
    println!("cargo:rerun-if-changed=cpp_calculator/");

    // Compile the wrapper C code so it can be used from Rust
    cc::Build::new()
        .cpp(true)
        .file("c_calculator/c_calculator.cpp")
        .compile("c_calculator");

    cc::Build::new()
        .cpp(true)
        .file("cpp_calculator/cpp_calculator.cpp")
        .file("cpp_calculator/cpp_calculator_wrapper.cpp")
        .compile("cpp_calculator");
}
