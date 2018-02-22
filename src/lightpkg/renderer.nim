import  strutils,
        math,
        random,
        pmap,
        sequtils,
        pmap,
        threadpool,
        pathtracing,
        raytracing

import  vector, ray, camera, color, material, sphere, ray, hit, world, job, solid
export  vector, ray, camera, color, material, sphere, ray, hit, world, job, solid

type RenderMethod* = enum
  RayTracing, PathTracing, NullTracing

proc nullTrace*(w:World, r:Ray, depth: int): Color = color.Black

proc getPrimaryRays(camera: Camera): seq[Ray] =
  var rays = newSeq[Ray]();
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
        ray.direction = ((world.camera.lt + (vdu * (float32(x) + float32(random(1.0))) +
                        vdv * (float32(y) + float32(random(1.0))))) -
                        world.camera.eye).unit
        clr = clr + trace(world, ray, 0)

      #return clr
      clr / samples
  )
