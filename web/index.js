import { Image, Scene, Pt, RGB } from "../pkg/ray_tracer"
import { memory } from "../pkg/ray_tracer_bg"


let canvas = document.getElementById("viewer"),
    ctx    = canvas.getContext("2d")

let image  = Image.new(canvas.width, canvas.height),
    pixels = new Uint8ClampedArray(memory.buffer, image.as_ptr(), image.len()),
    data   = new ImageData(pixels, canvas.width, canvas.height)

let scene = Scene.new()

scene.add_sphere(Pt.new(-18, 0,  60), 10, RGB.new(0xf7, 0x5c, 0x03))
scene.add_sphere(Pt.new(  0, 0, 120), 10, RGB.new(0x22, 0x74, 0xa5))
scene.add_sphere(Pt.new( 18, 0,  60), 10, RGB.new(0xf1, 0xc4, 0x0f))

for (let d = 0; d < 360; d += 20) {
  let rad = d * Math.PI / 180,
      t   = 60 * Math.PI / 180,
      r   = 15

  for (let ox of [-18, 18]) {
    let x = ox + r * Math.sin(rad) * Math.cos(t),
        y =  0 + r * Math.sin(rad) * Math.sin(t),
        z = 60 + r * Math.cos(rad)

    scene.add_sphere(Pt.new(x, y, z), 1.4, RGB.new(0x33, 0xff, 0xcc))
  }
}

scene.add_point_light(Pt.new(-9, -20, 90), 900)
scene.add_point_light(Pt.new(28, 20, 40), 400)

scene.render(image)
ctx.putImageData(data, 0, 0)
