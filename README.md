# RGB - Bifröst
Bifröst, the RGB "bridge" Server.


A server to store proofs for client-side validation, as explained in [RGB Protocol Specification #03 : Networking](https://github.com/rgb-org/spec/blob/master/03-bifrost.md).

* [Installation](#installation)
* [Configuration](#configuration)
* [Running](#running)
* [Interacting with Bifröst](#interacting-with-bifröst)

## Installation

1. Install Rust and Cargo: `curl https://sh.rustup.rs -sSf | sh`
2. Build the project: `cargo build`

When the build is completed, the executable will be located at `./target/debug/bifrost`.

For convenience, it can be useful to temporarily add the directory to your `PATH`, like so:

```
export PATH=$(readlink -f ./target/debug):$PATH
```

Make sure that you can now run the executable with:

```
bifrost --version
```

## Configuration

Bifröst, like its client [Kaleidoscope](https://github.com/rgb-org/kaleidoscope) and Bitcoin,
has a "home" data directory, which is used to store the database of proofs.

By default, the data directory is `$HOME/.rgb-server`. This can be overridden
by adding the `--datadir <NEWDIR>` (or `-d <NEWDIR>`) to each command.

If the directory does not exist, Bifröst will create it.

## Running

```
cargo run -- --port 80
```

## Interacting with Bifröst

Use the client [Kaleidoscope](https://github.com/rgb-org/kaleidoscope) to interact
with this server. In `.rgb/rgb.conf` set `"default_server": "localhost:3000"`.

As proofs are uploaded, they will be stored in `~/.rgb-server`.

