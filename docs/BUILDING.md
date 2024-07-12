# Building Enjo

If you want to build Enjo manually, this guide will be useful.

### Prerequisites

To build Enjo you need to follow the requirements:

- The latest version of the Rust toolchain.
- C/C++ compiler
- - **Windows**: The latest version of *Visual Studio Build Tools 2022* and *Windows SDK*.
- - **Linux** and **macOS**: Latest version of *GCC* or *Clang*.

If you already have installed Rust toolchain and C/C++ compiler, you can go to [Building](#building) section

> ⚠️ On Windows we recommend to use LLD as linker to speed up compilation. Learn more at [Tweaks](#tweaks) section.

### Install tools

You can go to [official Rust install page](https://www.rust-lang.org/tools/install) and follow the instructions on the website.

> ⚠️ Make sure that Rust toolchain and C compiler are installed correctly by running them through terminal.

### Downloading source code

If you have `git` installed you can use it to download the source code.

```shell
git clone https://git.kostyazero.com/kostya-zero/enjo
```

If not, you can download source code as ZIP. Extract its contents to wherever you want.

### Building

You can run building by running `cargo` with `build` argument. Also add `--release` argument to make optimized build.

```shell
cargo build --release
```

If you are using *Linux* or *macOS* and want to compile Enjo using `clang` as C compiler, you need to create a new environment variable called `RUSTFLAGS` and pass some value to it:

```shell
export RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=lld"
```

### Tweaks

Some tweaks that might be helpful.

#### Use LLD as linker on Windows

Using LLD can speed up compilation by 1.5 or 2 times. Make sure you install the LLVM toolchain by running `lld-link` in terminal. Then pass this to the PowerShell prompt:

```pwsh
$env:RUSTFLAGS = "-C link-arg=-fuse-ld=lld-link"
```

#### Native compilation

If you want to natively compile Enjo, use this in your command prompt:

```shell
# Bash, zsh, fish, ...
export RUSTFLAGS="-C target-cpu=native"

# PowerShell
$env:RUSTFLAGS = "-C target-cpu=native"
```
