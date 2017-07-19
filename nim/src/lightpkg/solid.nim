import material, hit

type Solid* = ref object of RootObj
  material*: Material
  isLight*: bool
