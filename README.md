# Goodomain

A tiny toy to find good domain from your favorite word.

## Usage

```bash
$ git clone https://github.com/7sdream/goodomain
$ cd goodomain
$ cargo run --release -- <your word>
```

## Example

```bash
cargo run --release -- 7sdream
7.sd/ream
7sd.re/am
7sdre.am
```

See `cargo run --release -- --help` for more options.

## TLD list file update

The `build.rs` will auto download the latest TLD list file and use it.

But due to cargo limit, if no file changed, `build.rs` will do not be executed again.

So if you want to update TLD list file, try `touch build.rs` and retry.

You can use `--version` option to get current TLD list file version

## LICENSE

WTFPL.
