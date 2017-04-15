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
  world.spheres.add((center: (0'f32, -10002'f32, 0'f32), radius: 9999'f32,
                      material: material.White, is_light: false))

  #Left wall
  world.spheres.add((center: (-10012'f32, 0'f32, 0'f32), radius: 9999'f32,
                      material: material.Red, is_light: false))

  #Right wall
  world.spheres.add((center: (10012'f32, 0'f32, 0'f32), radius: 9999'f32,
                      material: material.Green, is_light: false))

  #Back wall
  world.spheres.add((center: (0'f32, 0'f32, -10012'f32), radius: 9999'f32,
                      material: material.White, is_light: false))

  #Ceiling
  world.spheres.add((center: (0'f32, 10012'f32, 0'f32), radius: 9999'f32,
                      material: material.White, is_light: true))

  world.spheres.add((center: (-5'f32, 0'f32, 2'f32), radius: 2'f32,
                      material: material.Blue, is_light: false))

  world.spheres.add((center: (0'f32, 5'f32, -1'f32), radius: 4'f32,
                      material: material.Yellow, is_light: false))

  world.spheres.add((center: (8'f32, 5'f32, -1'f32), radius: 2'f32,
                      material: material.Magenta, is_light: false))
  return world


