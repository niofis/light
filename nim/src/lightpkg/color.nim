type Color* = tuple[a: float32, r: float32, g: float32, b:float32]
proc `*`*(a, b: Color): Color = (a: a.a * b.a, r: a.r * b.r, g: a.g * b.g, b: a.b * b.b)
proc `*`*(a: Color, s: float32): Color = (a: a.a, r: a.r * s, g: a.g * s, b: a.b * s)
proc `+`*(a, b: Color): Color = (a: a.a + b.a, r: a.r + b.r, g: a.g + b.g, b: a.b + b.b)
proc `/`*(a: Color, s: float32): Color = (a: a.a, r: a.r / s, g: a.g / s, b: a.b / s)

const Black*:Color = (0'f32,0'f32,0'f32,0'f32)