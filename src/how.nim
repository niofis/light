let world =
  World
    Camera
      width
      height
      pitch?
      direction
    Lights
      Light
        type
        position
        color
        intensity
    Solids
      Sphere
        center
        radius
        Color
      Triangle
        v1
        v2
        v3
        Color
      Mesh
        source
        Material

render(world)
  let primRays = getPrimRays(world.viewport);

