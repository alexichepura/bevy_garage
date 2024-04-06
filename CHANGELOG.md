# Changelog

## Version 0.9.0 (2024-04)
- bevy 0.13

## Version 0.8.0 (2023-11)
- bevy 0.12

## Version 0.7.0 (2023-07)
- bevy 0.11

## Version 0.6.0 (2023-05-24)

- added simple example
- spec components for car and wheel
- refactor car spawn
- split into components: CarTrack, CarSensors
- event to spawn car: SpawnCarOnTrackEvent
- HID renamed to Player

## Version 0.5.0 (2023-05-15)

- cargo update and prisma-client-rust 0.6.8
- crates for car, track, dqn
- plugins for dqn
- split config into car_config, track_config, dqn_config
- car spawn event
- better neural network, 30 actions per sec
- smaller wheels and torque
- refactor to bundles for components
- fixes for progress calc and lap count

## Version 0.4.0 (2023-04-26)

- bevy 0.10.1
- ground and asphalt shaders
- use depth bias instead of ear clip for ground
- simple engine sound with fundsp
- virtual joystick for mobile touch

## Version 0.3.0 (2023-03-23)

- bevy 0.10
- dfdx 0.11
- dash
- new car model

## Version 0.2.0

- rename to bevy_garage

## Version 0.1.0

- init bevy_rapier_car_sim
