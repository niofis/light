import world

type Resolution* = tuple[width, height: int]
type Section* = tuple[x, y, width, height: int]

type 
  Job* = ref object of RootObj
    world*: World
    resolution: Resolution
    section: Section

proc newJob*(world: World = newWorld(), resolution: Resolution = (0,0), section: Section = (0,0,0,0)): Job =
  return Job(world: world, resolution: resolution, section: section)

