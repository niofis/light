import color
export color

type Material* = tuple[refraction: float32, reflection: float32, color: Color]
  
const Black* = (refraction: 1.0'f32, reflection: 0'f32, color: (1'f32,0'f32,0'f32,0'f32))
const Red* = (refraction: 1.0'f32, reflection: 0'f32, color: (1'f32,1'f32,0'f32,0'f32))
const White* = (refraction: 1.0'f32, reflection: 0'f32, color: (1'f32,1'f32,1'f32,1'f32))
const Glass* = (refraction: 1.0'f32, reflection: 0'f32, color: (1'f32,1'f32,1'f32,1'f32))
