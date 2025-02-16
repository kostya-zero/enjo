# comment this to make it work on linux and macOS
set shell := ["pwsh.exe", "-c"]

all: lint

clean:
    cargo clean

lint:
    cargo clippy

build:
    cargo build

release:
    cargo build --release

run arg="":
    cargo run -- {{arg}}

runq arg="":
    cargo run -q -- {{arg}}

