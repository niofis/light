import  math,
        vector,
        ray,
        hit,
        material

type Sphere* = tuple[center: Vector3, radius: float32, material: Material, is_light: bool]


proc hit*(sp: Sphere, ray: Ray): Hit =
  let oc = ray.origin - sp.center
  let a = dot(ray.direction, ray.direction)
  let b = oc.dot(ray.direction)
  let c = dot(oc, oc) - sp.radius * sp.radius
  let dis = b*b - a*c

  if dis > 0:
    var e = sqrt(dis)
    var t:float32 = (-b - e) / a

    if t > 0.007'f32:
      let pt = ray.point(t)
      let n = (pt - sp.center).unit
      return (distance: t, point: pt, normal: n)

    t = (-b + e) / a

    if t > 0.007'f32:
      let pt = ray.point(t)
      let n = (pt - sp.center).unit
      return (distance: t, point: pt, normal: n)

    return nohit
  
  return nohit
