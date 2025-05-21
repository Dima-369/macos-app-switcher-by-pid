# macos-activate-app

## Motivation

I was using `hs.window.find("Kotlin Emacs"):focus()` with Hammerspoon, but it has a severe performance impact, should any application hang up since Hammerspoon queries all windows and on stuck applications, this can take seconds. If no windows are stuck, it is fast, but this Rust code is usually faster.

## Benchmarks

On MacBook Pro M1 2020.

### This repo

```bash
$ hyperfine 'target/release/macos-app-switcher-by-pid 37529'

Benchmark 1: target/release/macos-app-switcher-by-pid 37529
  Time (mean ± σ):      39.7 ms ±  10.6 ms    [User: 4.0 ms, System: 3.1 ms]
  Range (min … max):    35.6 ms …  96.0 ms    40 runs
 
  Warning: The first benchmarking run for this command was significantly slower than the rest (71.2 ms). This could be caused by (filesystem) caches that were not filled until after the first run. You should consider using the '--warmup' option to fill those caches before the actual benchmark. Alternatively, use the '--prepare' option to clear the caches before each timing run.
```

### Hammerspoon in CLI via `hs`

```bash
$ hyperfine "hs -c 'hs.window.find(\"Kotlin Emacs\"):focus()'"

Benchmark 1: hs -c 'hs.window.find("Kotlin Emacs"):focus()'
  Time (mean ± σ):      89.6 ms ±  13.0 ms    [User: 8.6 ms, System: 8.6 ms]
  Range (min … max):    74.4 ms … 122.6 ms    23 runs
```

### Interpreted Osascript

```bash
$ hyperfine 'osascript -e "tell application \"System Events\"
                                                 set proc to first process whose unix id is 37529
                                                 set frontmost of proc to true
                                                 end tell"'
                                                 
Benchmark 1: osascript -e "tell application \"System Events\"
       set proc to first process whose unix id is 37529
       set frontmost of proc to true
   end tell"
  Time (mean ± σ):     188.7 ms ±  24.2 ms    [User: 23.4 ms, System: 19.3 ms]
  Range (min … max):   150.9 ms … 223.1 ms    13 runs
```

## Overview
**macos-activate-app** is a command-line tool written in Rust that allows macOS users to activate a specific application based on its Process Identifier (**PID**). It employs Cocoa bindings and the Objective-C runtime to interact with macOS system APIs.

## Features
- Activate a running macOS application using its PID.
- Provides error messages for invalid inputs, missing applications, or failed activation attempts.

## Requirements
- macOS system environment.
- Rust toolchain installed.
- macOS application with a valid and running PID.

## Dependencies
- [`cocoa`](https://crates.io/crates/cocoa): Bindings for macOS Cocoa APIs.
- [`objc`](https://crates.io/crates/objc): Bindings to the Objective-C runtime, required for macOS development.

## Installation
1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd macos-app-switcher-by-pid
   ```
2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Usage
Run the executable with the target application's PID as the argument:
```bash
./target/release/macos-activate-app <pid>
```

### Example
Activate an application with PID `1234`:
```bash
./target/release/macos-activate-app 1234
```

## Error Handling
- **Invalid PID**: If the provided PID is not a valid integer.
- **Application Not Found**: If no running application matches the given PID.
- **Activation Failure**: If activation of the application fails for any reason.

## License
This project is distributed under the terms of the MIT license. See `LICENSE` for full details.
