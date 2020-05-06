# Amethyst Pong

## Description

This is an implementation of the pong example from the Amethyst Game Engine's Manuel. I am using it to help learn about rust through building a simple game. Once I am done with this example game, I will start developing a seperate simple game and update this repo.

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
