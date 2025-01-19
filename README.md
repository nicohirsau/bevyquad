# bevyquad

[![Github Actions](https://github.com/not-fl3/bevyquad/workflows/CI/badge.svg)](https://github.com/not-fl3/bevyquad/actions?query=workflow%3A)
[![Docs](https://docs.rs/bevyquad/badge.svg?version=0.4.5)](https://docs.rs/bevyquad/0.4.5/bevyquad/index.html)
[![Crates.io version](https://img.shields.io/crates/v/bevyquad.svg)](https://crates.io/crates/bevyquad)
[![Discord chat](https://img.shields.io/discord/710177966440579103.svg?label=discord%20chat)](https://discord.gg/WfEp6ut)

`bevyquad` is a simple and easy to use game library for Rust programming language, heavily inspired by [raylib](https://github.com/raysan5/raylib).

## Features

* Same code for all supported platforms, no platform dependent defines required.
* Efficient 2D rendering with automatic geometry batching.
* Minimal amount of dependencies: build after `cargo clean` takes only 16s on x230(~6 years old laptop).
* Immediate mode UI library included.
* Single command deploy for both WASM and Android.

## Supported Platforms

* PC: Windows/Linux/macOS;
* HTML5;
* Android;
* IOS.

## Build Instructions

### Setting Up a bevyquad Project

bevyquad is a normal rust dependency, therefore an empty bevyquad project may be created with:

```bash
# Create empty cargo project
cargo init --bin
```

Add bevyquad as a dependency to Cargo.toml:
```toml

[dependencies]
bevyquad = "0.4"
```

Put some bevyquad code in `src/main.rs`:
```rust
use bevyquad::prelude::*;

#[bevyquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
```

And to run it natively:
```bash
cargo run
```

For more examples take a look at [bevyquad examples folder](https://github.com/not-fl3/bevyquad/tree/master/examples)

### Linux

```bash
# ubuntu system dependencies
apt install pkg-config libx11-dev libxi-dev libgl1-mesa-dev libasound2-dev

# fedora system dependencies
dnf install libX11-devel libXi-devel mesa-libGL-devel alsa-lib-devel

# arch linux system dependencies
 pacman -S pkg-config libx11 libxi mesa-libgl alsa-lib
```

### Windows

On windows both MSVC and GNU target are supported, no additional dependencies required.

Also cross-compilation to windows from linux is supported:

```sh
rustup target add x86_64-pc-windows-gnu

cargo run --target x86_64-pc-windows-gnu
```

### WASM

```sh
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown
```

This will produce .wasm file in `target/debug/wasm32-unknown-unknown/CRATENAME.wasm` or in `target/release/wasm32-unknown-unknown/CRATENAME.wasm` if built with `--release`.

And then use the following .html to load it:

<details><summary>index.html</summary>

```html
<html lang="en">

<head>
    <meta charset="utf-8">
    <title>TITLE</title>
    <style>
        html,
        body,
        canvas {
            margin: 0px;
            padding: 0px;
            width: 100%;
            height: 100%;
            overflow: hidden;
            position: absolute;
            background: black;
            z-index: 0;
        }
    </style>
</head>

<body>
    <canvas id="glcanvas" tabindex='1'></canvas>
    <!-- Minified and statically hosted version of https://github.com/not-fl3/bevyquad/blob/master/js/mq_js_bundle.js -->
    <script src="https://not-fl3.github.io/miniquad-samples/mq_js_bundle.js"></script>
    <script>load("CRATENAME.wasm");</script> <!-- Your compiled wasm file -->
</body>

</html>
```
</details>

One of the ways to server static .wasm and .html:

```sh
cargo install basic-http-server
basic-http-server .
```

### IOS

To run on the simulator:

```
mkdir MyGame.app
cargo build --target x86_64-apple-ios --release
cp target/release/mygame MyGame.app
# only if the game have any assets
cp -r assets MyGame.app
cat > MyGame.app/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key>
<string>mygame</string>
<key>CFBundleIdentifier</key>
<string>com.mygame</string>
<key>CFBundleName</key>
<string>mygame</string>
<key>CFBundleVersion</key>
<string>1</string>
<key>CFBundleShortVersionString</key>
<string>1.0</string>
</dict>
</plist>
EOF

xcrun simctl install booted MyGame.app/
xcrun simctl launch booted com.mygame
```

For details and instructions on provisioning for real iphone, check [https://bevyquad.rs/articles/ios/](https://bevyquad.rs/articles/ios/)

<details>
<summary>Tips</summary>
Adding the following snippet to your Cargo.toml ensures that all dependencies compile in release even in debug mode. In bevyquad, this has the effect of making images load several times faster and your applications much more performant, while keeping compile times miraculously low.

```toml
[profile.dev.package.'*']
opt-level = 3
```
</details>

## async/await

While bevyquad attempts to use as few Rust-specific concepts as possible, `.await` in all examples looks a bit scary.
Rust's `async/await` is used to solve just one problem - cross platform main loop organization.

<details>
<summary>Details</summary>


The problem: on WASM and android it's not really easy to organize the main loop like this:
```
fn main() {
    // do some initialization

    // start main loop
    loop {
        // handle input

        // update logic

        // draw frame
    }
}
```

It is fixable on Android with threads, but on web there is not way to "pause" and "resume" WASM execution, so no WASM code should block ever.
While that loop is blocking for the entire game execution!
The C++ solution for that problem: https://kripken.github.io/blog/wasm/2019/07/16/asyncify.html

But in Rust we have async/await. Rust's `futures` are basically continuations - `future`'s stack may be stored into a variable to pause/resume execution of future's code at a later point.

async/await support in bevyquad comes without any external dependencies - no runtime, no executors and futures-rs is not involved. It's just a way to preserve `main`'s stack on WASM and keep the code cross platform without any WASM-specific main loop.
</Details>

## Community

- [Quads Discord server](https://discord.gg/WfEp6ut) - a place to chat with the library's devs and other community members.
- [Awesome Quads](https://github.com/ozkriff/awesome-quads) - a curated list of links to miniquad/bevyquad-related code & resources.

# Platinum sponsors

bevyquad is supported by:

[SourceGear](https://www.sourcegear.com/)
