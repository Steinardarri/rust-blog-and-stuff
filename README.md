# My personal blog written using [Rust](https://www.rust-lang.org)!

This is my personal website I built using [Yew](https://yew.rs) and [Preline](https://preline.co)

I used [Max Mohr's](https://github.com/maxjmohr/personal_website) amazing website as a template

## How to run

### Quick install if on Linux:
run `build_release.sh`

---

We will need the standard [Rust toolchain](https://www.rust-lang.org/learn/get-started), including `rustup`, `rustc` and `cargo`

Yew uses [WebAssembly](https://webassembly.org) and needs Rust to have it's target

`rustup target add wasm32-unknown-unknown`

We also need the wasm-opt package to optimizes our WebAssembly modules

`cargo install wasm-opt`

To run the project first of all you need to have `trunk` installed on your machine

`cargo install trunk`

then

`trunk serve --open` which builds the website then looks out for edits and refreshes the page when they are found

or

`trunk serve`

and open your browser on [http://localhost:8008](http://localhost:8008)

To build for release

`trunk build --release`

## Preline & Tailwind

This website uses Preline to manage styles

To install, we will need [Node.js](https://nodejs.org/en/download) to use it's package manager

First we get Tailwind itself

`npm install -D tailwindcss`

`npm install -D @tailwindcss/forms`

`npm install tailwind-typewriter`

then we set up Preline to manage it

`npm install preline`

