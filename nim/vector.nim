import math

type Vector3* = tuple[x: float32, y: float32, z: float32]

proc `+`*(a, b: Vector3): Vector3 = (x: a.x + b.x, y: a.y + b.y, z: a.z + b.z)
proc `*`*(a, b: Vector3): Vector3 = (x: a.x * b.x, y: a.y * b.y, z: a.z * b.z)
proc `*`*(a: Vector3, s: float32): Vector3 = (x: a.x * s, y: a.y * s, z: a.z * s)
proc `-`*(a, b: Vector3): Vector3 = (x: a.x - b.x, y: a.y - b.y, z: a.z - b.z)
proc `/`*(a: Vector3, s: float32): Vector3 = (x: a.x / s, y: a.y / s, z: a.z / s)
proc dot*(a, b: Vector3): float32 = a.x * b.x + a.y * b.y + a.z * b.z
proc norm*(a: Vector3): float32 = sqrt(a.dot(a))
proc unit*(a: Vector3): Vector3 = a / a.norm

const zero* = (0'f32, 0'f32, 0'f32)

