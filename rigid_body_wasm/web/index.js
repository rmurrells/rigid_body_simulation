import {init} from "rigid-body-wasm";
import {memory} from "rigid-body-wasm/rigid_body_wasm_bg";

var canvas = document.getElementById('canvas');
canvas.width = 800;
canvas.height = 600;
var ctx = canvas.getContext('2d');
var id = canvas.getContext('2d').getImageData(
    0, 0, canvas.width, canvas.height,
);
var rigid_body_wasm = init(canvas.width, canvas.height, 7);

function main() {
    setInterval(run, 1000./60);
}

function run() {
    rigid_body_wasm.tick();
    draw();
}

function draw() {
    ctx.putImageData(
	new ImageData(
	    new Uint8ClampedArray(
		memory.buffer,
		rigid_body_wasm.pixel_buffer(),
		canvas.width*canvas.height*4
	    ),
	    canvas.width,
	),
	0, 0,
    );
}

document.addEventListener('keydown', function(e) {
    rigid_body_wasm.on_key(e.keyCode, true);
});
document.addEventListener('keyup', function(e) {
    rigid_body_wasm.on_key(e.keyCode, false);
});

document.addEventListener('mousedown', function(e) {
    rigid_body_wasm.on_mouse_button(e.button, true);
});
document.addEventListener('mouseup', function(e) {
    rigid_body_wasm.on_mouse_button(e.button, false);
});

canvas.addEventListener('wheel', function(e) {
    rigid_body_wasm.on_mouse_wheel(e.deltaX, e.deltaY);
});

canvas.addEventListener('mousemove', function(e) {
    const rect = canvas.getBoundingClientRect();
    rigid_body_wasm.on_mouse_move(
	event.clientX-rect.left,
	event.clientY-rect.top,
    );
});

main();
