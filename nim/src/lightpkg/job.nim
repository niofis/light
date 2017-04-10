import world

type 
  Job = ref object of RootObj
    world*: World
    resolution: tuple[width, height: int]
    section: tuple[x, y, width, height: int]

