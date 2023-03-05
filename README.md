# 🦆🌊 golduck

a cli program to run and debug godot scenes, built in rust 🦀

![Made with VHS](https://vhs.charm.sh/vhs-5RWohXblPyOclV8bIBZBhc.gif)

```sh
Usage: golduck.exe [OPTIONS] <COMMAND>

Commands:
  play        Play the project (equivalent to pressing F5 in the editor)
  play-debug  Play the project in debug mode
  run         Run a specific scene, supports fuzzy findig
  debug       Debug a specific scene, supports fuzzy findig
  help        Print this message or the help of the given subcommand(s)

Options:
  -s, --silence   Silence the godot console output
  -e, --no-error  Silence the godot console errors
  -h, --help      Print help
  -V, --version   Print version
```

## Installation

- Crates.io: `cargo install golduck`
- Github: [Releases](https://github.com/wyvernbw/golduck/releases/tag/1.0.0)
