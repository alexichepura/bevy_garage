#/usr/bin/env bash

# https://dev.to/wadecodez/exploring-rust-for-native-ios-game-development-2bna
# brew install dasel

APP_NAME="$(cat Cargo.toml | dasel -r toml '.package.name')"
BUNDLE_ID="$(cat Cargo.toml | dasel -r toml '.package.metadata.bundle.identifier')"

cargo bundle --target aarch64-apple-ios-sim
xcrun simctl boot "iPhone 14"  
open /Applications/Xcode.app/Contents/Developer/Applications/Simulator.app 
xcrun simctl install booted "target/aarch64-apple-ios-sim/debug/bundle/ios/$APP_NAME.app"
xcrun simctl launch --console booted "$BUNDLE_ID"
