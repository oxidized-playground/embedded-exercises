# Bonus Exercise 1: A little Rust in C++
This exercise lets you compile a C++ project that automatically
builds and includes a Rust library. The Rust library will be called from the main 
loop as seen in main.cpp.

This project uses Corrosion-rs as an extension to CMake to build the Rust library from CMake.
https://github.com/corrosion-rs/corrosion

# How to compile me

Prerequisites: 

Have CMake installed
Have a C++ compiler installed. (more than likely you already have this when you installed Rust)

- cd to the root of the project
- mkdir build
- cd build
- cmake ..
- cmake --build .
- Note: This will automatically build the Rust library already. 

# How to run me

Depending on your system the executable will be located in different places:
- We're looking for a .exe file on Windows, and a file without an extension on Linux.
- If you are working on Windows, the executable will probably be located in the Debug folder.

Just execute the file from the command line. The output should look like this:

I am a C++ calculator!
4 + 5 = 9
4 - 5 = -1
4 * 5 = 20
Hi I'm a rust calculator!
4 + 5 = 9
4 - 5 = -1
4 * 5 = 40

