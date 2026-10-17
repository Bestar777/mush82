# Mush8
A native Mach-O iOS/macOS port of Mario Kart 8 Deluxe, with LAN, online and wireless multiplayer

## How does this work ?
the Switch and Apple Silicon both run arm64, so the game's code can almost run as is. a few instructions can't run outside a Switch (the x18 register, system calls, system registers), so this project rewrites them with a tool called  `nxpatch`
the rest, most importance peace is `nxhost` : it plays the role of the Switch's OS (Horizon) for the game, its services (files, saves, controllers, audio, network...) and its graphics API (NVN), which is turned into Metal. The game's shaders are translated to Metal ones and compiled ahead of time too

On macOS, the patched game code is loaded when you launch it. On iOS, and because we don't have JIT, we can't afforc to do that : game code is linked into the app when you build it

On top of that, Mush8 adds LAN Play over the internet ([lan-play.com](https://www.lan-play.com) servers) and a wireless mode for nearby devices

## Game files
Because Nintendo ninjas are always watching, you will need your own dump :
- `prod.keys`
- the base game (`.nsp`)
- on macOS, update 3.0.5 or 4.0.1 (`.nsp`; the executable build is checked); the iOS workflow still targets 4.0.1
- optionally, the Booster Course Pass (`.nsp`) and a `Firmware` folder (fonts, Miis)

Mush8 will then asks for them : on macOS pick the folder holding them in the launcher, on iOS import them from the app (or copy them in Files → On My iPhone → Mush8). File names don't matter

Keep a separate game-data folder for each update: `work/` contains the extracted
and rewritten code, shader cache and saves for that folder. Do not replace the
update NSP underneath an already prepared folder. Symlinks to your original
NSP, keys and Firmware directory work on macOS; copying the large packages is
unnecessary. The launcher shows the extracted update's display version, and the
runtime checks the main, SDK and video module build IDs before entering game code.

The `codex/mk8dx-3.0.5` branch adds macOS 3.0.5 compatibility. On the local
Apple M5 Mac, 3.0.5 has completed a three-lap 50cc Mario Kart Stadium race,
including keyboard input, audio generation, results, exit and reopening.
See [the validation record](docs/validation/macos-3.0.5-2026-10-17.md) for evidence
and limits; iOS, multiplayer and the complete track roster on 3.0.5 are unverified.

## Requirements
- Apple Silicon Mac, macOS 14 or later
- Rust toolchain
- Xcode 26 or later

For iOS, also :
- `rustup` with `rustup toolchain install 1.98.1` and `rustup target add aarch64-apple-ios --toolchain 1.98.1`
- `xcodegen` (`brew install xcodegen`)
- an Apple ID to sign the app on iOS (a free one works), or AltStore / SideStore to install the `.ipa`
- a device on iOS/macOS 26 or later, with 6 GB of RAM or more (8 GB is comfy)

## Building
The easy way :
```
./build.sh
```
It installs what's missing (Homebrew, Rust...), then asks whether you want the Mac or the iOS version and walks you through the rest. The Mac `.app` ends up in `~/Downloads/Mush8Output`, the iOS one goes straight to your plugged-in iPhone / iPad

By hand, macOS :
```
./make-app.sh
```
gives `build/Mush8.app`. Open it, choose your game files folder and hit Prepare (the first time, it extracts the game and compiles its shaders, a few minutes)

`./script/build_and_run.sh` builds, verifies the app signature and opens the
launcher (also available as the Codex Run action). `--build-only` builds without
opening, `--verify` checks launcher startup, `--debug` starts LLDB, and `--logs`
opens the launcher and follows the game log. Quit the game before rebuilding.

iOS :
```
./make-ios.sh --install
```
The signing team is taken from the account signed into Xcode (Settings > Accounts), `TEAM=<your team id>` overrides it, `BUNDLE_ID` sets your own app id, `FREE=1` signs with the entitlements a free account can get. The first build compiles every shader for iOS, so it takes a while

Or make an `.ipa` to install with AltStore / SideStore (no signing needed on the Mac) :
```
./make-ios.sh --ipa
```
it ends up in `build/Mush8.ipa`

## Multiplayer
Pick a mode in the launcher, under Multiplayer :
- LAN : players on the same network, a Switch can join too
- Online : any lan-play.com server (`host:port`), Switches on that server included
- Wireless : nearby Mush8 players, no router needed (keep Wi-Fi on)

In the game, the main menu then shows LAN Play, Online Play or Wireless Play

## Controls
Any controller works (Switch Pro, Joy-Con, DualSense, Xbox...), each one is a player. On macOS the keyboard works too (WASD stick, arrows d-pad, X/Return A, Z/Backspace B, Q/E L/R, 1/3 ZL/ZR, Tab +), on iOS there are touch controls when no controller is connected

The touchscreen works too : tap the game on iOS, click it on macOS

Motion controls (gyro steering) are off by default, turn them on in the launcher

An iPhone can be a controller too : open Mush8 on it and tap "Use as a controller for a Mac" (no game files needed). It finds the Mac running Mush8 on the same Wi-Fi and joins as the next player with its touch controls (and its motion, if Motion controls is on in the Mac launcher). Turn "iPhone controllers" on in the Mac launcher (it is by default), or on an iPad playing the game

## Limitations
- free Apple IDs and the `.ipa` (AltStore / SideStore) don't get the extended virtual addressing entitlement, so the game runs with a smaller 5 GB memory layout there (a paid account gets the full one)
- Nintendo's online play is gone, use Online (lan-play) instead
- 5 to 8 players split screen is Switch 2 only, so it's hidden
- no amiibo (yet)
- the two WebM videos (credits, Switch 2 dialog) play as a slideshow on iOS

Contributions are welcome !
