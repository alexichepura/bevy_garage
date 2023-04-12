# bevy_garage - space about cars and driving

Renamed from bevy_rapier_car_sim

![Bevy Garage 0.3 screenshot 2023-03-25](https://user-images.githubusercontent.com/5582266/227719005-d22da207-188c-4a6e-9582-68aa8616e9ca.jpg)

- game engine <https://bevyengine.org>
- rigid body physics <https://rapier.rs>
- Wasm demo <https://alexi.chepura.space/bevy-garage> (desktop only)
- xr bevy fork <https://github.com/kcking/bevy>
- iOS, WIP gyroscope using <https://github.com/madsmtm/objc2>
- kenney assets <https://www.kenney.nl/assets/car-kit>
- neural networks <https://github.com/coreylowman/dfdx>
- api server and client <https://github.com/tokio-rs/axum>
- db client <https://github.com/Brendonovich/prisma-client-rust>

## Run

```sh
brew install llvm # macos
sudo apt-get install lld # ubuntu
sudo pacman -S lld # arch
```

```sh
cargo run --release
# or faster compile
cargo run --release --features bevy/dynamic_linking
```

<https://bevyengine.org/learn/book/getting-started/setup/>

## Key bindings

- UP, DOWN, LEFT, RIGHT - drive
- 1, 2, 3, 4, 5 - camera views
- 0 - free camera with WASDQE(SHIFT) control and mouse
- R - debug mode
- SHIFT+SPACE - respawn at random position
- N - toggle brain

## History

- Deep Q learning NN - <https://www.youtube.com/watch?v=aN49ZP4PS-c>
- First attempts with NN - <https://www.youtube.com/watch?v=mVk1otSmySM>

![Screenshot 2023-02-10](https://user-images.githubusercontent.com/5582266/218020620-d50663a7-a38f-4431-8abf-8d794e552b6f.jpeg)
![screenshot 2023-01-19](https://user-images.githubusercontent.com/5582266/214000445-8fa5ac99-2412-416e-9905-8640c8d51502.jpg)
![old screenshot 1](https://user-images.githubusercontent.com/5582266/188065552-f1abd35e-10f9-43fa-935e-3530f3292dde.png)
![old screenshot 2](https://user-images.githubusercontent.com/5582266/180704095-2d4d6819-0b35-4653-b8e6-a3a50f793a9c.png)
![old screenshot 3](https://user-images.githubusercontent.com/5582266/177758958-3ac7a6da-b178-45bf-a9f4-edb25de3008e.jpg)

## License

Bevy Garage is free, open source and permissively licensed!
Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) to include both.

## Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
