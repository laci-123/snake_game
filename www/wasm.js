"use strict";

let wasm = null;
const main_canvas = document.getElementById("main-canvas");
const ctx = main_canvas.getContext("2d");

function color_to_hex_string(color) {
    const r = ((color >> (0 * 8)) & 0xFF).toString(16).padStart(2, '0');
    const g = ((color >> (1 * 8)) & 0xFF).toString(16).padStart(2, '0');
    const b = ((color >> (2 * 8)) & 0xFF).toString(16).padStart(2, '0');
    const a = ((color >> (3 * 8)) & 0xFF).toString(16).padStart(2, '0');
    return "#" + r + g + b + a;
}

function js_fill_rect(x, y, width, height, color) {
    ctx.fillStyle = color_to_hex_string(color); 
    ctx.fillRect(x, y, width, height);
}

let prev_timestamp = null;
function next_frame(timestamp) {
    if (prev_timestamp !== null) {
        const dt = (timestamp - prev_timestamp) * 0.001;
        // wasm.instance.exports.update(dt);
        wasm.instance.exports.render();
    }
    prev_timestamp = timestamp;
    window.requestAnimationFrame(next_frame);
}


WebAssembly.instantiateStreaming(fetch('browser_snake.wasm'), {
    env: {
        js_fill_rect
    }
}).then((w) => {
    wasm = w;
    window.requestAnimationFrame(next_frame);
});
