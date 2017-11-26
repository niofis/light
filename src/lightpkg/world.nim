import  vector,
        camera,
        solid,
        sphere,
        triangle,
        material


type World* = tuple[camera: Camera, objects: seq[Solid]]

proc newWorld*(): World =
  var world: World
  world.camera = (eye: (0'f32, 4.5'f32, -75'f32),
                  lt:  (-8'f32, 9'f32, -50'f32),
                  rt:  (8'f32, 9'f32, -50'f32),
                  lb:  (-8'f32, 0'f32, -50'f32))

  world.objects = newSeq[Solid]()
 
  #Floor
  world.objects.add(newSphere((0'f32, -10002'f32, 0'f32), 9999'f32,
                      material.White, false))

  #Left wall
  world.objects.add(newSphere((-10012'f32, 0'f32, 0'f32), 9999'f32,
                      material.Red, false))

  #Right wall
  world.objects.add(newSphere((10012'f32, 0'f32, 0'f32), 9999'f32,
                      material.Green, false))

  #Back wall
  world.objects.add(newSphere((0'f32, 0'f32, 10012'f32), 9999'f32,
                      material.White, false))

  #Ceiling
  world.objects.add(newSphere((0'f32, 10012'f32, 0'f32), 9999'f32,
                      material.White, true))

  #Miscelaneus
  world.objects.add(newSphere((-5'f32, 0'f32, -2'f32), 2'f32,
                      material.Blue, false))

  world.objects.add(newSphere((0'f32, 5'f32, 1'f32), 4'f32,
                      material.Yellow, false))

  world.objects.add(newSphere((8'f32, 5'f32, 1'f32), 2'f32,
                      material.Magenta, false))

  world.objects.add(newTriangle((-2'f32, -2'f32, -20'f32),
                                (0'f32, 0'f32, -10'f32),
                                (2'f32, -2'f32, -20'f32), material.Orange, false))
  return world


