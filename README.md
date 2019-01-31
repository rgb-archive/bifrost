# RGB - Bifröst

A server to store proofs for client-side validation, as explained in [RGB Protocol Specification #03 : Networking](https://github.com/rgb-org/spec/blob/master/03-bifrost.md).

## Installation

1. Install Rust and Cargo: `curl https://sh.rustup.rs -sSf | sh`
2. Build the project: `cargo build`

You can now run the executable with:

```
cargo run -- --port 3000
```

## Configuration

Bifröst, like its client [Kaleidoscope](https://github.com/rgb-org/kaleidoscope) and Bitcoin,
has a "home" data directory, which is used to store the database of proofs.

By default, the data directory is `$HOME/.rgb-server`. This can be overridden
by adding the `--datadir <NEWDIR>` (or `-d <NEWDIR>`) to each command.

If the directory does not exist, Bifröst will create it.

## Interacting with Bifröst

Use the client [Kaleidoscope](https://github.com/rgb-org/kaleidoscope) to interact
with this server. In `.rgb/rgb.conf` set `"default_server": "localhost:3000"`.

As proofs are uploaded, they will be stored in `~/.rgb-server`.
