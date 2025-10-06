# Hello Embedded

Build the example project with `cargo build --release` to see if the build environment is configured. 
If not make sure your have sourced the esp-toolchain with `source ~/export-esp.sh`.

Then connect the ESP32-S1 (TTGO) to the host-machine and execute `cargo run --release`, this will flash the example to the device.

## Throuble shooting

* Can't flash the device with `Error: espflash::serial_error`:
    Check the permissions and the usb passthrough of the VM. Check if the device pops up in `/dev` as `ttyACM0`.
    If this is correct give the user permission to access the device with `sudo chown $USER /dev/ttyACM0`
* `cargo build` fails to complete the build:
    This usually means that the build environment isn't correctly initialized or you're in the wrong project folder.
    Make sure you've sourced the esp-toolchain with `source ~/export-esp.sh` and that you're in the right folder i.e. exercise_2_hello_embedded.