import
  vector,
  hit

type Ray* = tuple[origin: Vector3, direction: Vector3]

proc point*(r: Ray, d: float32): Vector3 = r.origin + (r.direction * d)

proc reflect*(ray: Ray, hit: Hit): Ray =
  #let t = ray.direction * hit.normal
  #let dir = ray.direction - (hit.normal * t)
  #result.direction = dir
  #result.origin = hit.point + ray.direction * 0.001f
  #result.direction = ray.direction - (hit.normal * 2.0f) * (ray.direction * hit.normal)
  #result.origin = hit.point + result.direction * 0.001f
  result.direction = ray.direction - hit.normal * (2.0f * (ray.direction.dot(hit.normal)))
  result.origin = hit.point + result.direction * 0.001f

