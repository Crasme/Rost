# Rost
A small os written in rust

## Dependancies and setup

```bash
sudo apt-get update
sudo apt-get install -y make qemu-system-x86_64 python3
sudo curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
rustup default nightly
```

## Compile & Run

```bash
# Simple compilation
make build

# Compile and run
make build run

# Show all commands
make
```

