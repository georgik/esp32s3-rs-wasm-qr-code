# QR Code Generator in Rust compiled to WASM for ESP32-S3 running esp-wasmachine

This examples demonstrates an option to compile Rust application into WASM and run it on ESP32-S3-BOX
which is running [esp-wasmachine](https://github.com/espressif/esp-wasmachine).

## Getting started

- install [ESP-IDF 4.4.5](https://github.com/espressif/esp-idf)
- build and verify [esp-wasmachine](https://github.com/espressif/esp-wasmachine)

```
cd esp-wasmachine
idf.py build
idf.py storage-flash
idf.py flash
idf.py monitor
ls wasm
iwasm ./wasm/hello_world.wasm
```

## Building the application

- build QR Code generator for WASM

```
cargo build --release --target wasm32-wasi
```

- copy the application into filesystem and flash it

```
rm esp-wasmachine/build/storage.bin
cp esp32s3-rs-wasm-qr-code/target/wasm32-wasi/release/qrcode.wasm esp-wasmachine/main/fs_image/wasm
cd esp-wasmachine
idf.py build
idf.py storage-flash
```

## Test

Run the application

```
idf.py monitor
iwasm ./wasm/qrcode.wasm
```

The result should be displayed on console as QR code.
