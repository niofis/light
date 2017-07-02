import material, hit

type Solid* = ref object of RootObj
  material*: Material
  hit*: Hit,
  isLight*: bool
