# pbrt-rust

[![build and render](https://github.com/w3ntao/pbrt-rust/actions/workflows/build_and_render.yml/badge.svg?branch=main)](https://github.com/w3ntao/pbrt-rust/actions/workflows/build_and_render.yml)

A physically based ray tracer built in Rust.


## build and render

clone pbrt-rust and scenes repo:
```
$ git clone https://github.com/w3ntao/pbrt-rust.git
$ git clone https://github.com/w3ntao/pbrt-rust-scenes.git
```

build and render test scenes:
```
$ cd pbrt-rust
$ cargo build --release
$ bash render_all.sh
```

## preview

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/dragon_10.png)

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/ganesha.png)

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/killeroo-gold.png)

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/killeroo-simple.png)

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/lte-orb-simple-ball.png)

![](https://github.com/w3ntao/pbrt-rust-scenes-preview/blob/main/lte-orb-silver.png)
