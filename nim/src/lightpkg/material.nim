import color

type Material* = tuple[refraction: float32, reflection: float32, color: Color]
  
const Black* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Black)
const Red* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Red)
const White* = (refraction: 1.0'f32, reflection: 0'f32, color: color.White)
const Green* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Green)
const Blue* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Blue)
const Yellow* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Yellow)
const Cyan* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Cyan)
const Magenta* = (refraction: 1.0'f32, reflection: 0'f32, color: color.Magenta)
const Glass* = (refraction: 1.0'f32, reflection: 0'f32, color: color.White)
