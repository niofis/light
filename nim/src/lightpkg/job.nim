import world

type Resolution* = tuple[width, height: int]
type Section* = tuple[x, y, width, height: int]

type 
  Job* = ref object of RootObj
    world*: World
    resolution: Resolution
    section: Section

proc newJob*(world: ref World = nil, resolution: Resolution = (0,0), section: Section = (0,0,0,0))
  return Job(world, resolution, section)

