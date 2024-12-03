# Advent of Code helper

[![asciicast](https://asciinema.org/a/de7BDjSC1PCuaFj2n30ZmxNzl.svg)](https://asciinema.org/a/de7BDjSC1PCuaFj2n30ZmxNzl)

## Installation

```bash
cargo install aoc_helper
```

## Set up

Create a configuration by copy/pasting the following to `~/.aoc_config`. Your session secret can be found in any network request on the aoc website.

```toml
[project]
code_directory = "/Users/me/labs/aoc/src/bin"
input_directory = "/Users/me/labs/aoc/inputs/"

[session]
secret = "YOUR_SECRET"
```

## Usage

Run the `aoc` command and follow the prompts.
