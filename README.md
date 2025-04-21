# weasel-gen

This crate provides the binary `wgen`, which is a [weasel program](https://en.wikipedia.org/wiki/Weasel_program) for displaying simple ascii animations.

TODO: Include gif

## Usage

```
Usage: wgen [OPTIONS] [STRING]...

Arguments:
  [STRING]...  String to generate [default: "Hello, World!"]

Options:
  -s, --stats              Weather or not to show stats at the end
  -d, --delay <DELAY>      Print delay between iterations (milliseconds) [default: 50]
  -a, --alpha              Use only a-z, A-Z
      --alphanum           Use a-z, A-Z, 0-9
  -c, --charset <CHARSET>  Supply a custom character set
  -f, --factor <FACTOR>    TODO [default: 3]
  -h, --help               Print help
```
