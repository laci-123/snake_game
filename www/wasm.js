"use strict";

WebAssembly.instantiateStreaming(fetch('browser_snake.wasm'), {}).then((wasm) => {
    let x = wasm.instance.exports.add(1, 2);
    console.log(x);
});
