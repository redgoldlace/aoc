# Advent of Code
This is a really silly advent of code harness that I wrote out of boredom. You can see my solutions in the
`src/solutions` directory if you're interested.

So far, these are only solutions for 2021, and I might not use this again next year. Who knows.

## Usage
Clone the repository, `cd` into its directory and use `cargo install --path .` to install.
The CLI is then available as `aoc`, assuming your Rust environment is configured correctly.

If you don't feel like polluting your local environment, you can use `cargo run --` (followed by any number of
arguments) instead.

Past that, the CLI is fairly simple. `--help` should give you most of the information you need:
```
aoc

USAGE:
    aoc.exe --session <SESSION> <SUBCOMMAND>

OPTIONS:
    -h, --help                 Print help information
        --session <SESSION>    [env: AOC_SESSION]

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    run     Run solutions. You may choose to run all solutions, only the latest solution, or a
            specific solution
```
You may specifically be interested in `run all`, which will fetch input for solutions and run them concurrently.

It's important to note that since input differs for everybody, **you must provide a session cookie to use the CLI**. You
may pass your AoC session cookie using either the `--session` flag or set it in the `AOC_SESSION` environment variable.