import  vector,
        ray,
        hit,
        material,
        solid

const EPSILON = 0.000005'f32

type Triangle* = ref object of Solid
  v0*: Vector3
  v1*: Vector3
  v2*: Vector3


proc newTriangle*(v0: Vector3 = zero, v1: Vector3 = zero, v2: Vector3 = zero, material: Material = White, isLight: bool = false): Triangle =
  var tr = Triangle(v0: v0, v1: v1, v2: v2)
  tr.material = material
  tr.isLight = isLight
  return tr

method hit*(tr: Triangle, ray: Ray): Hit {.inline.} =
  let edge1 = tr.v1 - tr.v0
  let edge2 = tr.v2 - tr.v0
  let normal = edge1.cross(edge2).unit

  let pvec = ray.direction.cross(edge2)

  let det = edge1.dot(pvec)

  if det > -EPSILON and det < EPSILON:
    return nohit

  let inv_det = 1.0'f32 / det

  let tvec = ray.origin - tr.v0

  let u = tvec.dot(pvec) * inv_det

  if u < 0 or u > 1'f32:
    return nohit

  let qvec = tvec.cross(edge1)

  let v = ray.direction.dot(qvec) * inv_det
  
  if v < 0 or (u + v) > (1.0f + EPSILON):
    return nohit

  let t = edge2.dot(qvec) * inv_det
  if t > EPSILON:
    let pt = ray.point(t)
    return (distance: t, point: pt, normal: normal)

  return nohit
