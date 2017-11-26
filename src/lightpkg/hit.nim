import vector

type Hit* = tuple[distance: float32, point: Vector3, normal: Vector3]

const nohit* = (distance: 1e16'f32, point: zero, normal: zero)
