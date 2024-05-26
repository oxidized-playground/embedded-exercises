# Hello Embedded

Build the example project with `cargo build` to see if the build environment is configured. 
If not make sure your have sourced the esp-toolchain with `source ~/export-esp.sh`.

Then connect the ESP32-S1 (TTGO) to the host-machine and execute `cargo run`, this will flash the example to the device.

## Throuble shooting

* Can't flash the device with `Error: espflash::serial_error`:
    Check the permissions and the usb passthrough of the VM. Check if the device pops up in `/dev` as `ttyACM0`.
    If this is correct give the user permission to access the device with `sudo chown $USER /dev/ttyACM0`
* `cargo build` fails to complete the build:
    This usually means that the build environment isn't correctly initialized or you're in the wrong project folder.
    Make sure you've sourced the esp-toolchain with `source ~/export-esp.sh` and that you're in the right folder i.e. exercise_2_hello_embedded.

# EXTRA INFO: Environment setup
## TTGO ESP32 prerequisites

Install Rust (with rustup) if you don't have rustup installed yet, follow the instructions on the [rustup.rs](https://rustup.rs) site.
If installed make sure u have updated rust to atleast version > 1.66 by running `rustup update`.

### Install Cargo Sub-Commands
```sh
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash
```

### Python
Make sure to have python installed (3.x)

> **Note**
>
> If you are running macOS or Linux then `libuv` must also be installed for `espflash` and `cargo-espflash`; 
> this is available via most popular package managers. If you are running Windows you can ignore this step.
> ```
> # macOS
> brew install libuv
> # Debian/Ubuntu/etc.
> apt-get install libuv-dev
> # Fedora
> dnf install systemd-devel
> ```
> Also, the `espflash` and `cargo-espflash` commands shown below, assume that version `2.0` or
> greater.

#### Install Rust & Clang toolchains for Espressif SoCs (with espup)
```sh
espup install
# Unix
. $HOME/export-esp.sh
```
> **Warning**
>
> Make sure you source the generated export file, as shown above, in every terminal before building any application as it contains the required environment variables. This is not needed for Windows

See the [Installation chapter of The Rust on ESP Book](https://esp-rs.github.io/book/installation/index.html) for more details.


## Desktop prerequisites

The simulator uses SDL2 and its development libraries which must be installed to build and run it.

#### Linux
```
sudo apt install libsdl2-dev
```
Or simmilar for other distro's

#### macOS (brew)
```
brew install sdl2
```

Users on Apple silicon or with custom installation directories will need to set ```LIBRARY_PATH``` for the linker to find the installed SDL2 package:
```
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```
More information can be found in the SDL2 documentation.

#### Windows
The simplest method is copying the binaries as shown [here](https://github.com/Rust-SDL2/rust-sdl2#windows-msvc).

If this doesn't work see the [Rust-SDL2 crate's README](https://github.com/Rust-SDL2/rust-sdl2) for instructions.

> **Windows Note**
>
> Make sure you put the libs inside the stable-x86_64-pc-windows-msvc toolchain and not in the esp toolchain.
> Also put the .dll file in the `desktop` folder so that it's next to the cargo.toml 

## Prepare core-application
Navigate to the core-application folder and call
`cargo build`

## Possible problems

- Make sure in Windows that your paths are not getting too long. If they are too long, the build process will not work correctly and you will get linker errors.

- When running `cargo build` in the esp32-s1 folder you get this error: `Error: Too long output directory`. 
Unfortunately you are using Windows and that means that you will have to copy the `esp-rs-std-display` folder over to `C:/` and rename it to something like `a`. Then rename the `esp23-s1` folder to something like `b` and run `cargo build` from the b directory

## Project setup
This example makes use of a template project which can be created by running `cargo generate esp-rs/esp-idf-template`.