import  strutils,
        math,
        random,
        pmap,
        sequtils,
        pmap,
        threadpool

import  vector, ray, camera, color, material, sphere, ray, hit, world, job, solid
export  vector, ray, camera, color, material, sphere, ray, hit, world, job, solid

const MAXDEPTH = 5

randomize()

proc rnd2(): float32 = (2.0 * random(1.0)) - 1.0

proc rndDome(normal: Vector3): Vector3 =
  var d:float32
  var p:Vector3

  d = -1.0

  while d < 0:
    p = ((rnd2(), rnd2(), rnd2())).unit
    d = p.dot(normal)
  return p

proc pathTrace2*(w: World, r: Ray, depth: int): Color =
  var did_hit = false
  var hit = nohit
  var out_color = color.Black
  var sp:Solid

  for s in w.objects:
    let lh = s.hit(r)
    
    if lh.distance < hit.distance:
      sp = s
      did_hit = true
      out_color = s.material.color
      hit = lh
  
  if did_hit == true and depth < MAXDEPTH:
    if sp.isLight == false:
      if sp.material.reflection > 0.0f:
        let nray = r.reflect(hit)
        let ncolor = pathTrace2(w, nray, depth + 1)
        out_color = (out_color * (1 - sp.material.reflection)) + (ncolor * sp.material.reflection)
      else:
        let nray = (origin: hit.point, direction: rnd_dome(hit.normal))
        let ncolor = pathTrace2(w, nray, depth + 1)
        let at = nray.direction.dot(hit.normal)
        out_color = out_color * (ncolor * at)
      

  if did_hit == false or depth >= MAXDEPTH:
    out_color = color.Black

  return out_color

proc pathTrace*(w: World, r: Ray, depth: int): Color =
  let res =
    w.objects
      .mapIt((solid: it, ht: it.hit(r)))
      .foldl(if a.ht.distance < b.ht.distance: a else: b)

  if res.ht == nohit or depth >= MAXDEPTH:
    return color.Black

  if res.solid.isLight:
    return res.solid.material.color

  if res.solid.material.reflection > 0:
    let nray = r.reflect(res.ht)
    let ncolor = pathTrace(w, nray, depth)
    return (res.solid.material.color * (1 - res.solid.material.reflection)) + (ncolor * res.solid.material.reflection)

  let nray = (origin: res.ht.point, direction: rnd_dome(res.ht.normal))
  let ncolor = pathTrace(w, nray, depth + 1)
  let at = nray.direction.dot(res.ht.normal)
  return res.solid.material.color * (ncolor * at)
