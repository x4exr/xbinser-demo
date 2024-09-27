const xbinser = require("xbinser/src/lib");
const DEBUG = true;

interface FrameBuffer {
    frame: number[]
}

let decoder = new xbinser.Decoder({
    frame: "[x:u8]"
});

let canvas = document.createElement("canvas");
let fps = document.createElement("p");

let frame_count = 0;

setInterval(() => {
    fps.innerText = `${frame_count}fps`;
    frame_count = 0;
}, 1000);

canvas.width = 1000;
canvas.height = 200;

let context = canvas.getContext("2d");
let image_data = context.getImageData(0, 0, canvas.width, canvas.height);

function load_image(buffer: Uint8ClampedArray) {
    let data = new ImageData(buffer, canvas.width, canvas.height);
    context.putImageData(data, 0, 0);
}

let socket = new WebSocket("ws://localhost:1084");

socket.onopen = () => {
    if (DEBUG) { console.log("Connected to server"); }
    document.body.appendChild(canvas);
    document.body.appendChild(fps);
}

socket.onclose = (error) => {
    console.error(error);
    setTimeout(() => window.location.reload(), 1000);
}

socket.onerror = (error) => {
    console.error(error);
    setTimeout(() => window.location.reload(), 1000);
}

socket.onmessage = async (message) => {
    frame_count += 1;

    let buffer = new Uint8Array(await message.data.arrayBuffer());
    let decoded = decoder.decode(0n, buffer)[0] as FrameBuffer;

    console.log(decoded.frame.length);

    load_image(decoded.frame as any as Uint8ClampedArray);
}