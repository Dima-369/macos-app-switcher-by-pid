# macos-activate-app

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