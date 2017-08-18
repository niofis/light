import world

type Resolution* = tuple[width, height: int]
type Section* = tuple[x, y, width, height: int]

type 
  Job* = ref object of RootObj
    world*: World
    resolution*: Resolution
    section*: Section
    samples*: int

proc newJob*(world: World = newWorld(), resolution: Resolution = (480,272), section: Section = (0,0,480,272), samples: int = 5): Job =
  return Job(world: world, resolution: resolution, section: section, samples: samples)

