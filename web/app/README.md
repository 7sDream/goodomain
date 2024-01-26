# Goodomain Web App

## Build

You need build goodomain's WASM binding first, see `/bindings/goodomain-wasm` for prerequisites.

After you can compile it, install `wasm-pack` (using package manager or NPM), then run:

```bash
wasm-pack build ..\..\bindings\goodomain-wasm --release
mv ..\..\bindings\goodomain-wasm\pkg ..
```

to convert it into a NodeJS module and put in `../pkg`, we use this folder as a dependency in `package.json`.

Now you can continue the `npm install` then `npm run dev`.
