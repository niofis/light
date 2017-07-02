import  strutils,
        math,
        random,
        pmap,
        sequtils,
        pmap,
        threadpool

import  vector, ray, camera, color, material, sphere, ray, hit, world, job
export  vector, ray, camera, color, material, sphere, ray, hit, world, job

const MAXDEPTH = 5

type RenderMethod* = enum
  RayTracing, PathTracing, NullTracing

randomize()

proc rnd2(): float32 = float32(2'f32 * random(1'f32)) - 1'f32

proc rndDome(normal: Vector3): Vector3 =
  var d:float32
  var p:Vector3

  d = -1'f32

  while d < 0:
    p = ((rnd2(), rnd2(), rnd2())).unit
    d = p.dot(normal)
  return p

proc pathTrace(w: World, r: Ray, depth: int): Color =
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
      let ncolor = pathTrace(w, nray, depth + 1)
      let at = nray.direction.dot(hit.normal)
      out_color = out_color * (ncolor * at)

  if did_hit == false or depth >= MAXDEPTH:
    out_color = color.Black

  return out_color

proc rayTrace(w: World, r: Ray, depth: int): Color =
  let hit =
    w.spheres
      .mapIt((obj: it, ht: it.hit(r)))
      .filterIt(it.ht != nohit)
      .foldl(if a.ht.distance < b.ht.distance: a else: b)
  return hit.obj.material.color

proc nullTrace(w:World, r:Ray, depth: int): Color = color.Black

proc getPrimaryRays(camera: Camera): seq[Ray] =
  var rays = newSeq(Ray);
  return rays

proc render*(job: Job, algorithm: RenderMethod): seq[Color] =
  let
    world = job.world
    vdu = (world.camera.rt - world.camera.lt) / float32(job.resolution.width)
    vdv = (world.camera.lb - world.camera.lt) / float32(job.resolution.height)
    width = job.resolution.width
    height = job.resolution.height
    samples = job.samples.float32
    ps = toSeq(0..<width * height)
    hs = toSeq(0..<height)
    ws = toSeq(0..<width)
    trace = case algorithm
      of PathTracing: pathTrace
      of RayTracing: rayTrace
      else: nullTrace

  ps.pmap(proc (p:int): auto =
      let x = p mod width
      let y = p / width
      var clr = color.Black
      var ray:Ray

      ray.origin = world.camera.eye

      for i in 1..job.samples:
        ray.direction = ((world.camera.lt + (vdu * (float32(x) + float32(random(1'f32))) +
                        vdv * (float32(y) + float32(random(1'f32))))) -
                        world.camera.eye).unit
        clr = clr + trace(world, ray, 0)

      clr / samples
  )
