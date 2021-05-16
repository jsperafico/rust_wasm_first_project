# wasm_first_project

The current code was developed using [oficial rust + wasm tutorial](https://rustwasm.github.io/book), but without npm or nodejs.
If you are on Visual Code, please use the extension "Five Server (Live Server)" to make sure wasm mime type isn't an issue.

Install `wasm-pack` by using:
```
cargo install wasm-pack
```


To generate vanilla js code, please use:
```
wasm-pack build --no-typescript --target web
```

Possible targets:
- bundler * default
- nodejs
- web
- no-modules

Make sure to use:
```
<script type="module">
    import init, { desired_method } from './pkg/file.js';
    async function run () {
        await init();
        //then you can use your desired_method
    }
</script>
```

More info check [the official documentation here](https://rustwasm.github.io/wasm-bindgen/examples/without-a-bundler.html).