# rustGame

A small 2D game engine written in Rust, plus a sample game built on top of it.

## Structure

This is a Cargo workspace with two crates:

- **`simple-engine`** — a lightweight 2D engine built on [`pixels`](https://crates.io/crates/pixels) and [`winit`](https://crates.io/crates/winit).
- **`sample-game`** — a demo spaceship game that uses `simple-engine`.

## Running

```sh
cd <path-to-repo>/sample-game/
cargo run
```

## Controls

- Arrow keys / WASD — move
- Shift — boost
- Spacebar — shoot
- Escape — exit
