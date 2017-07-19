import  vector,
        camera,
        sphere,
        material


type World* = tuple[camera: Camera, spheres: seq[Sphere]]

proc newWorld*(): World =
  var world: World
  world.camera = (eye: (0'f32, 4.5'f32, 75'f32),
                  lt:  (-8'f32, 9'f32, 50'f32),
                  rt:  (8'f32, 9'f32, 50'f32),
                  lb:  (-8'f32, 0'f32, 50'f32))

  world.spheres = newSeq[Sphere]()
 
  #Floor
  world.spheres.add(newSphere((0'f32, -10002'f32, 0'f32), 9999'f32,
                      material.White, false))

  #Left wall
  world.spheres.add(newSphere((-10012'f32, 0'f32, 0'f32), 9999'f32,
                      material.Red, false))

  #Right wall
  world.spheres.add(newSphere((10012'f32, 0'f32, 0'f32), 9999'f32,
                      material.Green, false))

  #Back wall
  world.spheres.add(newSphere((0'f32, 0'f32, -10012'f32), 9999'f32,
                      material.White, false))

  #Ceiling
  world.spheres.add(newSphere((0'f32, 10012'f32, 0'f32), 9999'f32,
                      material.White, true))

  world.spheres.add(newSphere((-5'f32, 0'f32, 2'f32), 2'f32,
                      material.Blue, false))

  world.spheres.add(newSphere((0'f32, 5'f32, -1'f32), 4'f32,
                      material.Yellow, false))

  world.spheres.add(newSphere((8'f32, 5'f32, -1'f32), 2'f32,
                      material.Magenta, false))
  return world


