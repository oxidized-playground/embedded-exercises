# Guide to get programming with Rust on the ESP32

This guide assumes you have Git installed on your laptop.

## Installation of Rust

Install Rust by downloading the Rustup-Init installer from [rust-lang.org](https://www.rust-lang.org/learn/get-started)

The install wizard will ask you if you want a standard install. Go ahead and take that option.
If you don't have it, this will install the Visual Studio installer and with it you need to get
C++ build tools. The easiest is to select "Desktop development with C++".

With that installed you may need to reboot your PC.

## Installation of the ESP32 toolchain.

The ESP32 that's used in the workshop is built on the XTensa architecture. By default this target
is not supported by Rust, but Espressif has built their own support for this architecture in Rust and made
it very easy for us to use it.

Execute the following commands

- `cargo install espup --locked`
- `espup install`
- `cargo install espflash --locked`

## preparation for exercise 3

- `rustup target add wasm32-unknown-unknown`


## Run exercise 1 and 2

Now you should be up and running for the workshop!

