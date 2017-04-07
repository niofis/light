import vector

type Ray* = tuple[origin: Vector3, direction: Vector3]

proc point*(r: Ray, d: float32): Vector3 = r.origin + (r.direction * d)
