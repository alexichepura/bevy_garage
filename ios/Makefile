.PHONY: xcodebuild run install boot-sim generate clean

DEVICE = ${DEVICE_ID}
ifndef DEVICE_ID
	DEVICE=$(shell xcrun simctl list devices 'iOS' | grep -v 'unavailable' | grep -v '^--' | grep -v '==' | head -n 1 | cut -d ' ' -f 7 | sed 's/[()]//g')
endif

run: install
	xcrun simctl launch --console $(DEVICE) com.github.alexichepura.bevy_garage

boot-sim:
	xcrun simctl boot $(DEVICE) || true

install: xcodebuild-simulator boot-sim
	xcrun simctl install $(DEVICE) build/Build/Products/Debug-iphonesimulator/bevy_garage.app

xcodebuild-simulator:
	IOS_TARGETS=x86_64-apple-ios xcodebuild -scheme bevy_garage_ios -configuration Debug -derivedDataPath build -destination "id=$(DEVICE)"

xcodebuild-iphone:
	IOS_TARGETS=aarch64-apple-ios xcodebuild -scheme bevy_garage_ios -configuration Debug -derivedDataPath build -arch arm64

clean:
	rm -r build
	cargo clean
