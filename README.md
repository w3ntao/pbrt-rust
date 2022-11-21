# pbrt-rust

[![build and test](https://github.com/w3ntao/rust-ray-tracer/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/w3ntao/rust-ray-tracer/actions/workflows/build-and-test.yml)


A rust ray tracer inspired by [Ray Tracing in One Weekend Series](https://raytracing.github.io/)

## build and run

download stanford dragon model:
```
$ cd pbrt-rust
$ wget 'https://casual-effects.com/g3d/data10/research/model/dragon/dragon.zip'
$ mkdir models; unzip dragon.zip -d models/
```

switch to channel nightly and run:
```
$ rustup default nightly; cargo run --release
```

## rendering samples

You may check rendering samples at [pbrt-rust-gallery](https://github.com/w3ntao/pbrt-rust-gallery).
