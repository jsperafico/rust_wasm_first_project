# wasm_first_project

The current code was developed using [oficial rust + wasm tutorial](https://rustwasm.github.io/book), but without npm or nodejs.
If you are on Visual Code, please use the extension "Five Server (Live Server)" to make sure wasm mime type isn't an issue.

Install `wasm-pack` by using:
```
cargo install wasm-pack
````


To generate vanilla code, please use:
```
wasm-pack build --target web
```