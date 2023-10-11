# Image rendering 

This is a simple application used for benchmarking image rendering in Makepad, it consists of a grid of animated images. 

The images either rotate, scale or fade out:

https://github.com/mobilerust/makepad_image_manipulation/assets/22042418/fc0dd5d1-ce2d-45a1-a8a5-05f1b13a9079

You can find the benchmark results and how to run your own measurements in [this document](https://www.notion.so/wyewrks/Benchmark-Image-rendering-performance-under-Upscaling-f3ffdd6c75ef4748969afa037397bd0a)

We also compared Makepad's results against Flutter and Native. You can find their implementations under [/benchmark_comparisons](benchmark_comparisons)

## Build Instructions


## 1. Setup Makepad

### Clone the Makepad repository
```
git clone git@github.com:makepad/makepad.git
```

### Change to latest 'rik' branch
```
git branch rik
```

### Install makepad subcommand for cargo
```
cd ~/makepad
cargo install --path ./tools/cargo_makepad
```

## 2. Get Project

### Clone the makepad_image_manipulation repo
```
git clone https://github.com/project-robius/makepad_image_manipulation
```

## 3. Android Build

### Install Android toolchain (First time)
```
rustup toolchain install nightly
cargo makepad android install-toolchain
```

### Install app on Android devivce or Android emulator
Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a device connected properly
```
cd ~/makepad_image_manipulation
cargo makepad android run -p makepad_image_manipulation --release
```

## 4. iOS Setup & Install

### Install IOS toolchain (First time)
```
rustup toolchain install nightly
cargo makepad ios install-toolchain
```

### Install app on Apple devivce or iOS simulator

### iOS Setup

For iOS, the process is slightly more complicated. The steps involved are:
1. Enable your iPhone's Developer Mode, please see instructions here: [Enable Developer Mode](https://www.delasign.com/blog/how-to-turn-on-developer-mode-on-an-iphone/)
1. Setup an Apple Developer account
1. Setup an empty skeleton project in XCode
    1. File -> New -> Project to create a new "App"
    1. Set the Product Name as **`Animation`**  (used in --org later)
    1. Set the Organization Identifier to a value of your choice, for this example we will use **`rs.robius`**. (used in --app later)
    1. Setup the Project Signing & Capabilities to select the proper team account 
1. In XCode, Build/Run this project to install and run the app on the simulator and device
1. Once the simulator and device has the "skeleton" app installed and running properly, then it is ready for Makepad to install its application.

### Makepad Install
We will run the `cargo makepad ios` command, similar to Android build above, but there are some 3 to 4 additional parameters that need to be filled in:

`--org-id`

This is the <string> value of the ApplicationIdentifierPrefix <key> in the `**.mobileprovision` file located in the `~/Library/MobileDevice/Provisioning Profiles` directory.
It should be a 10 digit alphanumeric value.

`--org`
    
First few parts of the organization identifier (which makes up the Bundle Identifier). Usually in the form of **com.somecompany** or **org.someorg**
This is the same value used to setup the initial skeleton app above. For this example:
> `rs.robius`
    
`--app`

The name of the application or the project. This is the same as the Product Name used to setup the initial skeleton app above. In this case:
> `Animation`
    
`--ios-version` (optional)
    
defaults to 17. Set it to 16 or other values if the device is not running iOS 17.

### Example

For this example, we have the Bundle Identifier of **`rs.robius.Animation`**

### Install app on IOS simulator
```
cd ~/makepad_image_manipulation
cargo makepad ios --org=rs.robius --app=Animation run-sim -p makepad_image_manipulation --release
```

### Install app on IOS device
```
cd ~/makepad_image_manipulation
cargo makepad ios --ios-version=16 --org-id=<ORGIDVALUE> --org=rs.robius --app=Animation run-device -p makepad_image_manipulation --release
```

## 5. WASM Build

*Coming Soon*

