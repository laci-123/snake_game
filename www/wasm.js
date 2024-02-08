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

function string_from_ptr(mem_buffer, ptr, len) {
    const bytes = new Uint8Array(mem_buffer, ptr, len);
    return (new TextDecoder()).decode(bytes);
}

function js_fill_circle(x, y, r, color) {
    ctx.fillStyle = color_to_hex_string(color); 
    ctx.beginPath();
    ctx.arc(x, y, r, 0, 2 * Math.PI);
    ctx.fill();
}

function js_fill_text(text_ptr, text_len, x, y, color, font_size) {
    ctx.fillStyle = color_to_hex_string(color); 
    ctx.font = font_size + "px Times New Roman";
    ctx.textAlign = "center";
    const text = string_from_ptr(wasm.instance.exports.memory.buffer, text_ptr, text_len);
    ctx.fillText(text, x, y);
}

let prev_timestamp = null;
function next_frame(timestamp) {
    if (prev_timestamp !== null) {
        ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
        const dt = (timestamp - prev_timestamp) * 0.001;
        wasm.instance.exports.update(dt);
        wasm.instance.exports.render();
    }
    prev_timestamp = timestamp;
    window.requestAnimationFrame(next_frame);
}

main_canvas.addEventListener("keydown", (e) => {
    if (e.key === "ArrowRight") {
        wasm.instance.exports.input(0);
    }
    else if (e.key === "ArrowUp") {
        wasm.instance.exports.input(1);
    }
    else if (e.key === "ArrowLeft") {
        wasm.instance.exports.input(2);
    }
    else if (e.key === "ArrowDown") {
        wasm.instance.exports.input(3);
    }
});


WebAssembly.instantiateStreaming(fetch('browser_snake.wasm'), {
    env: {
        js_fill_circle,
        js_fill_text
    }
}).then((w) => {
    wasm = w;
    window.requestAnimationFrame(next_frame);
});

main_canvas.focus();
