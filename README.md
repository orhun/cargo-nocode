## `cargo-nocode`

> [No code](https://github.com/kelseyhightower/nocode) is the best way to write secure and reliable applications. Write nothing; deploy nowhere.

`cargo-nocode` aims to bring the _nocode_ approach to the Rust ecosystem! 🦀

### Quickstart guide

```sh
$ cargo nocode init
# this will create nocode.rs file which follows the nocode guidelines
# now we're ready to write no code!

$ cat nocode.rs
# absolutely empty

$ cargo nocode build
# does nothing

$ cargo nocode run
# does nothing

$ cargo nocode deploy
# does nothing
```

### Usage

```sh
Usage: cargo-nocode <command>

Possible commands:
- init    initialize a nocode application
- build   build the nocode application
- run     run the nocode application
- deploy  deploy the nocode application
```

### Installation

Just close your eyes and pretend.

Or you can install from crates.io:

```sh
cargo install cargo-nocode
```

#### License

<sup>
All code is licensed under <a href="LICENSE">The Apache 2.0 License</a>.
</sup>
