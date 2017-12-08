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

proc rayTrace*(w: World, r: Ray, depth: int): Color =
  let hit =
    w.objects
      .mapIt((obj: it, ht: it.hit(r)))
      .filterIt(it.ht != nohit)
      .foldl(if a.ht.distance < b.ht.distance: a else: b)
  return hit.obj.material.color
