# RGB - Bifröst

Bifröst, the RGB "bridge" Server

* [Installation](#installation)
* [Configuration](#configuration)
* [Running](#running)

## Installation

1. Install Cargo: `curl -sSf https://static.rust-lang.org/rustup.sh | sh`
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

RGB, like Bitcoin, has a "home" data directory, which is used to store the database of proofs and might contain a configuration file (`rgb-server.conf`).

By default, the data directory is `$HOME/.rgb-server`. This can be overridden by adding the `--datadir <NEWDIR>` (or `-d <NEWDIR>`) to each command.

If the directory does not exist, RGB will create it.

## Running

`./target/debug/bifrost -p 80`
