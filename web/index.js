import { Image, Scene } from "../pkg/ray_tracer"
import { memory } from "../pkg/ray_tracer_bg"


let canvas = document.getElementById("viewer"),
    ctx    = canvas.getContext("2d")

let image  = Image.new(canvas.width, canvas.height),
    pixels = new Uint8ClampedArray(memory.buffer, image.as_ptr(), image.len()),
    data   = new ImageData(pixels, canvas.width, canvas.height)

let scene = Scene.new()

scene.render(image)
ctx.putImageData(data, 0, 0)