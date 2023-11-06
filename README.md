# pbrt-rust

[![build and test](https://github.com/w3ntao/pbrt-rust/actions/workflows/build-and-test.yml/badge.svg)](https://github.com/w3ntao/pbrt-rust/actions/workflows/build-and-test.yml)


A rust ray tracer inspired by [Ray Tracing in One Weekend Series](https://raytracing.github.io/)

## build and run

download stanford dragon model:
```
$ cd pbrt-rust
$ wget 'https://casual-effects.com/g3d/data10/research/model/dragon/dragon.zip'
$ mkdir models; unzip dragon.zip -d models/
```

run
```
$ cargo run --release
```

## preview

![](https://github.com/w3ntao/pbrt-rust-gallery/blob/main/test_case_rt_weekend_dragon_pt_1936.png)

![](https://github.com/w3ntao/pbrt-rust-gallery/blob/main/test_case_cornell_box_next_event_estimation_1936.png)

![](https://github.com/w3ntao/pbrt-rust-gallery/blob/main/test_case_cornell_box_specular_next_event_estimation_1936.png)

![](https://github.com/w3ntao/pbrt-rust-gallery/blob/main/test_case_cornell_box_metal_dragon_next_event_estimation_1936.png)

![](https://github.com/w3ntao/pbrt-rust-gallery/blob/main/test_case_smallpt_1936.png)
