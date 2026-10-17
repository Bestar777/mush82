# macOS 3.0.5 validation — 2026-10-17

## Build and scope

Branch: `codex/mk8dx-3.0.5`, based on `1b2e9b8`. Changes are local and have not
been committed or pushed. Machine: Apple M5, 32 GiB RAM, macOS 27.0.1,
Xcode 27.1, Rust/Cargo 1.96.0.

The user-owned update was extracted with `nxfs`, which read display version
`3.0.5` from its Control NCA. Executable identities (first 20 NSO ID bytes):

| Module | Build ID |
| --- | --- |
| main | `fe941ed5ba14be5d505698da1bbf4fe700000000` |
| sdk | `ba520906609c503227871a314a94fb5f6ccbe444` |
| subsdk0 | `d9de423efc1512328c3c05378c7cb03173facde3` |

The SDK/video modules match the existing supported builds. The generic ARM64
rewriter handles 3.0.5 without transplanting 4.0.1's main-module address patches.
The host now checks the supported module combination and compares the prepared
main module's ID with the selected update NSP before entering guest code.
`GetDisplayVersion` now returns the identified version instead of a hardcoded
`4.0.1`; the launcher also displays the extracted Control NCA version.

Game inputs, DLC, keys and firmware remain in the user's original NS directory.
Only symlinks are used in `~/Library/Application Support/Mush8/3.0.5`.
Prepared code, shader caches and this run's independent saves live under that
folder's `work/`. No NSGames/Astris state was modified. Keys and game-derived
files are excluded from Git.

The tested `build/Mush8.app/Contents/MacOS/nxhost` SHA-256 is
`a16bf4ce31c1f1840365ed626d91a971e300b02a2af32c8c46bc660aeed2491f`.

## Executed checks

- `cargo test -p nxfs --lib`: 4 passed, including supported-build recognition
  and rejection of unknown/mixed module combinations.
- `nxpatch verify` against all three original 3.0.5 modules:
  main 3,626; subsdk0 8,362; sdk 7,069 equivalent patches. Total 19,057,
  zero failures and zero untested patches.
- `nxhost --precompile`: 14,024 shader translations compiled in 109 seconds.
  One additionally scanned Geometry-stage program was unsupported; this was
  retained in the log and did not prevent the tested race from rendering.
- `./script/build_and_run.sh --build-only`: release build and
  `codesign --verify --deep --strict` passed.
- `git diff --check` and shell syntax check passed.

## Actual gameplay

The packaged app was launched through its SwiftUI launcher and Play button.
The host log identifies native ARM64 execution of 3.0.5; the corrected title
screen visibly says `Ver. 3.0.5`.

Verified transitions and actions: title → initial Mii selection → Chinese main
menu → single player → Grand Prix → 50cc → Mario → standard kart, wheels and
glider → Mushroom Cup → Mario Kart Stadium → all three laps → results.
Return supplied A/acceleration, D supplied steering, and Q supplied item use.
Anti-gravity driving, coins, item pickup/use and the finish/results screen were
observed. This race finished in 12th place; no guest fatal/panic was logged.

During the third lap, FPS samples 366–395 averaged 60.007 FPS (range
59.5–60.7), with a worst frame of 27.6 ms. These are presentation measurements
for that observed interval, not an all-track performance claim. The complete
run includes menu/loading transitions and a 3,837 ms maximum frame interval
during first-use scene/pipeline work. Results/menu presentation later reached
about 120 FPS; those samples are not used as the race performance measurement.

AudioQueue produced 48 kHz stereo signed 16-bit PCM without the fallback
"no output device" error. An eight-second race excerpt beginning 150 seconds
into the captured session had peak 10,514 and RMS 1,740.208, confirming
non-silent generated audio. Physical speaker listening was not independently
verified.

Command-Q normally ended the game, and a process inventory confirmed the
Player was absent. Reopening retained the initialized Mii save (80,736-byte
`userdata.dat`) and reached the main menu without initial Mii selection.
After the completed race, Command-Q and another launch again reached the
3.0.5 title. The final running game has no diagnostic audio/FPS environment
enabled and is left at the title screen for the user.

## Local evidence and limits

Raw logs and screenshots are ignored under `.local/logs/`:
`extract-3.0.5.log`, `rewrite-3.0.5.log`, `verify-all-3.0.5.log`,
`precompile-3.0.5.log`, `nxfs-tests.log`, `build-3.0.5-final.log`,
`gameplay-runtime-3.0.5.log`, `race-metrics.json`, `gameplay-metrics.json`,
`title-3.0.5-final.jpg`, `race-driving-20s.jpg`,
`race-final-lap-antigravity.jpg`, `race-finish.jpg`,
`reopened-title-3.0.5.jpg`, `title-audio.wav`, and `race-audio.wav`.

This proves a complete local single-player race and the observed lifecycle.
It does not verify every track, a full four-race cup, multiplayer, Nintendo
online services, physical controllers/gyro, iOS, or long-duration stability.
4.0.1 recognition is covered by the unit test, but its gameplay was not rerun
because that update was not present locally.

To run again, open `build/Mush8.app`; the selected 3.0.5 data folder is persisted.
For another update, prepare a separate folder instead of replacing an NSP
under existing extracted code. Quit Mush8 before invoking the build/run script.
To roll back source changes, return to the original branch; do not delete the
data folder or saves as part of a source rollback.
