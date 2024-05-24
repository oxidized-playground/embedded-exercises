# Bonus exercise 2 - A little C++ in Rust

This exercise lets you compile a Rust project that automatically builds and includes a C++
library. The C++ library will be called from the main loop as seen in main.rs.

In the build.rs (which gets called automatically when building Rust) you can see that the C++
library will be built by cargo.

# How to compile me

- cargo build

# How to run me

- cargo run

The output should look like this:

Hi I'm a rust calculator!
4 + 5: 9
4 - 5: -1
4 * 5: 20
I am a C calculator!
4 + 5: 9
4 - 5: -1
4 * 5: 20
I am a C++ calculator!
4 + 5: 9
4 - 5: -1
4 * 5: 20


# Important stuff

This project actually builds a Rust executable that calls a C and a C++ library. 
The C library can almost directly be called from Rust. But in Rust we still need to use the extern C to be able to
link to the library. The C++ library however needs to be wrapped in a C interface to be able to call it from Rust. 
Which involves a lot of pointers. This makes the C library a lot simpler to use than an actual C++ library. Assuming you need
to create objects that is.

Note that calling the C and C++ libraries requires the use of the unsafe specifier. This is because we are calling
functions that are not guaranteed to be safe. In this case we are calling functions from a C and C++ library which
could potentially do anything.

In the Rust executable the C and C++ libraries are called by separate Rust objects that both implement the calculator 
trait. This makes it possible to call the C and C++ libraries in the same way from the main loop.