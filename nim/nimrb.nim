import strutils
import math
import random

import  vector,
        ray,
        camera,
        color,
        material,
        sphere,
        ray,
        hit,
        world

export  vector,
        ray,
        camera,
        color,
        material,
        sphere,
        ray,
        hit,
        world
 
const 
  WIDTH = 640
  HEIGHT = 360
  SAMPLES = 5
  MAXDEPTH = 5


proc rnd2(): float32 = float32(2'f32 * random(1'f32)) - 1'f32

proc rndDome(normal: Vector3): Vector3 =
  var d:float32
  var p:Vector3

  d = -1'f32

  while d < 0:
    p = ((rnd2(), rnd2(), rnd2())).unit
    d = p.dot(normal)
  return p

proc trace(w: World, r: Ray, depth: int): Color =
  var did_hit = false
  var hit = nohit
  var out_color = color.Black
  var sp:Sphere

  for s in w.spheres:
    let lh = s.hit(r)
    
    if lh.distance < hit.distance:
      sp = s
      did_hit = true
      out_color = s.material.color
      hit = lh

  if did_hit == true and depth < MAXDEPTH:
    if sp.is_light == false:
      let nray = (origin: hit.point, direction: rnd_dome(hit.normal))
      let ncolor = trace(w, nray, depth + 1)
      let at = nray.direction.dot(hit.normal)
      out_color = out_color * (ncolor * at)

  if did_hit == false or depth >= MAXDEPTH:
    out_color = color.Black

  return out_color

proc writePPM(data: seq[seq[Color]]) =
  let ppm = open("nimrb.ppm", fmWrite)
  ppm.write(format("P3\n$# $#\n255\n",WIDTH, HEIGHT))
  for row in data:
    for c in row:
      ppm.write(format("$# $# $# ",
        int(floor(c.r * 255.99'f32)),
        int(floor(c.g * 255.99'f32)),
        int(floor(c.b * 255.99'f32))))
    ppm.write("\n")
  ppm.close()

proc render*(): seq[seq[Color]] =
  var data = newSeq[seq[Color]]()
  let world = newWorld()
  let vdu = (world.camera.rt - world.camera.lt) / float32(WIDTH)
  let vdv = (world.camera.lb - world.camera.lt) / float32(HEIGHT)

  randomize()
  
  for y in 0..(HEIGHT-1):
    var row = newSeq[Color]()
    for x in 0..(WIDTH-1):
      var color = color.Black
      var ray:Ray

      ray.origin = world.camera.eye

      for i in 1..SAMPLES:
        ray.direction = ((world.camera.lt + (vdu * (float32(x) + float32(random(1'f32))) +
                        vdv * (float32(y) + float32(random(1'f32))))) -
                        world.camera.eye).unit
        color = color + trace(world, ray, 0)

      color = color / float32(SAMPLES)
      row.add(color)
    data.add(row)
    return data
