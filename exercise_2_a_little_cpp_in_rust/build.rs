fn main() {
    // Tell cargo to rerun whenever there was a change
    println!("cargo:rerun-if-changed=cpp_calculator/");

    // Compile the wrapper C code so it can be used from Rust
    cc::Build::new()
        .cpp(true)
        .file("cpp_calculator/cpp_calculator.cpp")
        .compile("cpp_calculator");
}
