import math

type Color* = tuple[a: float32, r: float32, g: float32, b:float32]

const Black*:Color = (0'f32,0'f32,0'f32,0'f32)
const Red*:Color = (1'f32, 1'f32, 0'f32, 0'f32)
const Green*:Color = (1'f32, 0'f32, 1'f32, 0'f32)
const Blue*:Color = (1'f32, 0'f32, 0'f32, 1'f32)
const Yellow*:Color = (1'f32, 1'f32, 1'f32, 0'f32)
const Magenta*:Color = (1'f32, 1'f32, 0'f32, 1'f32)
const Cyan*:Color = (1'f32, 0'f32, 1'f32, 1'f32)
const White*:Color = (1'f32, 1'f32, 1'f32, 1'f32)
const Orange*:Color = (1'f32, 0.933'f32, 0.46'f32, 0.101'f32)

proc `*`*(a, b: Color): Color = (a: a.a * b.a, r: a.r * b.r, g: a.g * b.g, b: a.b * b.b)
proc `*`*(a: Color, s: float32): Color = (a: a.a, r: a.r * s, g: a.g * s, b: a.b * s)
proc `+`*(a, b: Color): Color = (a: a.a + b.a, r: a.r + b.r, g: a.g + b.g, b: a.b + b.b)
proc `/`*(a: Color, s: float32): Color = (a: a.a, r: a.r / s, g: a.g / s, b: a.b / s)

proc to255(val: float32): uint32 =
  let v = if val <= 1.0: val else: 1.0
  return uint32(floor(v * 255.99))

proc toARGB*(c: Color): uint32 =
  return  (c.a.to255 shl 24) +
          (c.r.to255 shl 16) +
          (c.g.to255 shl 8) +
          (c.b.to255)


