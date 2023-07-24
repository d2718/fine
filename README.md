# fine
A friendlier version of find.

```text
Usage: fine [OPTIONS] [PATTERN]...

Arguments:
  [PATTERN]...  The pattern(s) to match file paths against

Options:
  -d, --dir <BASE>  Base directory in which to search [default: .]
  -r, --regex       Use regex (instead of glob) matching
  -a, --absolute    Print absolute paths [default: relative to BASE]
  -h, --help        Print help
  -V, --version     Print version
```

## Installation

It should just `cargo build --release`.

## Use

By default, `fine` searches in and below the current directory for filenames
that match the supplied set of globs:

```text
dan@lauDANum:~/dev/fine$ fine *.rs
./src/opt.rs
./src/main.rs
```

You can supply multiple patterns:

```text
dan@lauDANum:~/dev/fine$ fine *.rs Cargo.*
./src/opt.rs
./src/main.rs
./Cargo.toml
./Cargo.lock
```

Specify a different directory with `-d`:

```text
dan@lauDANum:~/dev/fine$ fine -d /usr/share/fonts terminus*
/usr/share/fonts/opentype/terminus
/usr/share/fonts/opentype/terminus/terminus-bold-oblique.otb
/usr/share/fonts/opentype/terminus/terminus-bold.otb
/usr/share/fonts/opentype/terminus/terminus-oblique.otb
/usr/share/fonts/opentype/terminus/terminus-normal.otb
```

By default, `fine` prints paths relative to the specified base directory.
Force it to print the full path with `-a`:

```text
dan@lauDANum:~/dev/fine$ fine *.rs -a
/home/dan/dev/fine/src/opt.rs
/home/dan/dev/fine/src/main.rs
```

Specify regex patterns instead of globs with `-r`:

```text
dan@lauDANum:~/dev/fine$ fine -d target/debug -r 'clap_.*a[0-2]'
target/debug/deps/clap_lex-a87e359c0a25297d.d
target/debug/deps/clap_builder-8a1806fd13db2c47.d
target/debug/deps/libclap_lex-a87e359c0a25297d.rmeta
target/debug/deps/libclap_builder-8a1806fd13db2c47.rlib
target/debug/deps/libclap_builder-8a1806fd13db2c47.rmeta
target/debug/.fingerprint/clap_lex-a87e359c0a25297d
target/debug/.fingerprint/clap_builder-8a1806fd13db2c47
```

Match your pattern agains the entire path (instead of just the final
element) with `-p`:

```rust
dan@lauDANum:~/dev/fine$ fine -d ~/.config *helix*
/home/dan/.config/helix
dan@lauDANum:~/dev/fine$ fine -d ~/.config -p *helix*
/home/dan/.config/helix
/home/dan/.config/helix/themes
/home/dan/.config/helix/themes/zzd_rose_pine.toml
/home/dan/.config/helix/runtime
/home/dan/.config/helix/languages.toml
/home/dan/.config/helix/config.toml
```

## The Future

  * filter by file type
  * optimization, probably (I've tried to do things in a
    not-obviously-stupid fashion, but otherwise there's none.)
  * more organized error handling

## &c.

This is not meant to be anything like a feature-complete rewrite
of `find(1)`. It's meant to alleviate the need to search out the
Coreutils docs just to use the 80-20 (or maybe 90-10) use case of
finding a file.

```text
dan@lauDANum:~/dev/fine$ info find
bash: info: command not found
```

Great.