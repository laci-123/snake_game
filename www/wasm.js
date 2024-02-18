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

function js_fill_text(text_ptr, text_len, x, y, color, font_size, alignment) {
    ctx.fillStyle = color_to_hex_string(color); 
    ctx.font = font_size + "px Luminary";
    if (alignment === 0) {
        ctx.textAlign = "center";
    }
    else if (alignment === 1) {
        ctx.textAlign = "left";
    }
    else {
        ctx.textAlign = "right";
    }
    const text = string_from_ptr(wasm.instance.exports.memory.buffer, text_ptr, text_len);
    ctx.fillText(text, x, y);
}

function js_random_between(min, max) {
    return Math.random() * (max - min) + min;
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
    else if (e.key === " ") {
        wasm.instance.exports.input(4);
    }
    else if (e.key === "r") {
        wasm.instance.exports.input(5);
    }
});

function resize_canvas(e) {
    let rect = main_canvas.getBoundingClientRect();
    const correction_factor = 0.95; // Determined experimentally. Probably needed because of some rounding error.
    main_canvas.width  = rect.width * window.devicePixelRatio;
    main_canvas.height = rect.height * window.devicePixelRatio * correction_factor;
    wasm.instance.exports.resize_canvas(main_canvas.width, main_canvas.height);
}

window.addEventListener("resize", resize_canvas);

WebAssembly.instantiateStreaming(fetch('browser_snake.wasm'), {
    env: {
        js_fill_circle,
        js_fill_text,
        js_random_between
    }
}).then((w) => {
    wasm = w;
    window.requestAnimationFrame(next_frame);
    main_canvas.focus();
    resize_canvas(null);
});
