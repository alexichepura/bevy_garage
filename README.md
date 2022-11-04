# 3d car simulation in rust

Sideproject to learn rust, computer graphics and neural networks. Track is scaled Nürburgring GP.
https://www.youtube.com/watch?v=aN49ZP4PS-c

<img width="1020" alt="Screenshot 2022-09-02 at 08 30 26" src="https://user-images.githubusercontent.com/5582266/188065552-f1abd35e-10f9-43fa-935e-3530f3292dde.png">

Deep Q learning is a special guest here that knows how to approximate control function.

Possible improvements:

- Current sensors config can't predict good turns, because it can't see behind the wall.
- Computing gradients online is hard, so next improvement from replay buffer is probably offline training longer with bigger batches.

How to run

```sh
cargo prisma generate
cargo run --release
```

Giants:

- [rust](https://www.rust-lang.org)
- [bevy](https://bevyengine.org)
- [rapier3d](https://rapier.rs)
- [nalgebra](https://nalgebra.org)
- [dfdx](https://github.com/coreylowman/dfdx)
- [prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust)

Car 3d models are from https://www.kenney.nl/assets/car-kit

Old screenshots:
<img width="1277" alt="screenshot" src="https://user-images.githubusercontent.com/5582266/180704095-2d4d6819-0b35-4653-b8e6-a3a50f793a9c.png">
<img width="1279" alt="old screenshot" src="https://user-images.githubusercontent.com/5582266/177758958-3ac7a6da-b178-45bf-a9f4-edb25de3008e.jpg">

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
