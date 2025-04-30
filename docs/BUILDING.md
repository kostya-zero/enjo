# Building Enjo

If you want to build Enjo manually, this guide will be useful.

### Prerequisites

To build Enjo you need to follow the requirements:

- The latest version of the Rust toolchain.
- C/C++ compiler
- - **Windows**: The latest version of _Visual Studio Build Tools 2022_ and _Windows SDK_.
- - **Linux** and **macOS**: Latest version of _GCC_ or _Clang_.

If you already have installed Rust toolchain and C/C++ compiler, you can go to [Building](#building) section

> [!NOTE]
> On Windows we recommend to use LLD as linker to speed up compilation. Learn more at [Tweaks](#tweaks) section.

### Install tools

You can go to [official Rust install page](https://www.rust-lang.org/tools/install) and follow the instructions on the website. After installation, make sure that Rust toolchain and C compiler are installed correctly by running them through terminal.

### Downloading source code

If you have `git` installed you can use it to download the source code.

```shell
git clone https://github.com/kostya-zero/enjo.git
```

If not, you can download source code as ZIP. Extract its contents to wherever you want.

### Building

You can run building by running `cargo` with `build` argument. Also add `--release` argument to make optimized build.

```shell
cargo build --release
```

The compiled binary will be located in `target/release/`.

#### Using Clang/LLD on Linux/macOS (Optional)

If you are using _Linux_ or _macOS_ and want to compile Enjo using `clang` as the C compiler and `lld` as the linker for potentially faster link times, you can set the `RUSTFLAGS` environment variable for your current terminal session:

```shell
# Make sure clang and lld are installed
export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
# Then run: cargo build --release
```
For a more permanent configuration, see the [Tweaks](#tweaks) section about using `.cargo/config.toml`.

### Tweaks

These optional tweaks can improve compilation speed or runtime performance.

#### Recommended: Using `.cargo/config.toml`

For persistent build configuration across sessions, it's recommended to create a file named `config.toml` inside a `.cargo` directory in the root of the project (`enjo/.cargo/config.toml`). This file allows you to specify various build options, including linkers and compilation flags.

Examples for common tweaks are shown below. You can combine flags if needed.

#### Faster Linking with LLD

Using the LLD linker (from the LLVM project) can significantly speed up the linking phase of compilation (often 1.5x to 2x faster).

**1. Install LLD:**
   - **Windows:** Ensure LLVM tools are installed. You can often select this as an individual component ("C++ Clang tools for Windows") in the Visual Studio Installer or downloading official LLVM toolchain installer. Verify that `lld-link.exe` is in your system's PATH.
   - **Linux:** Install `lld` using your package manager (e.g., `sudo apt install lld`, `sudo dnf install lld`). You also need `clang`.
   - **macOS:** Install LLVM via Homebrew (`brew install llvm`). You also need `clang` (usually comes with Xcode Command Line Tools).

**2. Configure Cargo:**
   Add the following to your `.cargo/config.toml` file, adjusting the target triple if necessary:

   ```toml
   # Cargo.toml

   # For Windows MSVC target
   [target.x86_64-pc-windows-msvc]
   rustflags = ["-C", "link-arg=-fuse-ld=lld-link"]

   # For Linux targets (replace with your specific target if needed)
   [target.x86_64-unknown-linux-gnu]
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=lld"]

   # For macOS targets (replace with your specific target if needed)
   [target.x86_64-apple-darwin]
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=lld"]
   ```

#### Native Compilation

This optimizes the build for the specific CPU architecture of the machine you are compiling on. This can potentially increase runtime performance but makes the resulting binary less portable (it might not run on machines with older CPUs).

Add the following to your `Cargo.toml`:

```toml
[target.*]
rustflags = ["-C", "target-cpu=native"]
```

You can combine this with other flags, for example:

```toml
# Add linker settings for your platform as shown above
[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "target-cpu=native"]
```
