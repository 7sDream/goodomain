# Goodomain

A tiny toy to find good domain from your favorite word.

## Web App

Check [GitHub Page](https://7sdream.github.io/goodomain/?q=7sDream) of this project.

Source code in `web/app`.

## CLI tool

### Install from source

```bash
cargo install --path crates/goodomain-cli
```

### Usage

```bash
$ goodomain 7sdream
7.sd/ream
7sd.re/am
7sdre.am
```

See `goodomain --help` for more options.

## As a Library

- Rust: See `crates/goodomain`
- NodeJS: See `bindings/goodomain-wasm`, the WebApp uses it.

## LICENSE

GPLv3, check `LICENSE.md`.
