import material, hit, ray

type Solid* = ref object of RootObj
  material*: Material
  isLight*: bool
  #hit*: proc[T](solid: T, ray: Ray): Hit

method hit*(solid: Solid, ray: Ray): Hit {.base.} = nohit
