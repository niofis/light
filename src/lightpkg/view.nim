import  sdl2/sdl,
        sdl2/sdl_gfx_primitives as gfx,
        times,
        strutils,
        sequtils

const
  WindowFlags = 0
  RendererFlags = sdl.RendererAccelerated

type
  View* = ref ViewObj
  ViewObj = object
    window*: sdl.Window
    renderer*: sdl.Renderer
    title*: string
    width*: int
    height*: int
    update*: proc(pixels:var seq[uint32])

proc newView*(title:string, width:int, height:int, update: proc(pixels:var seq[uint32])): View =
  var view = View()
  view.title = title
  view.width = width
  view.height = height
  view.update = update
  view.window = nil
  view.renderer = nil
  return view

proc init*(view: View): bool =
  if sdl.init(sdl.InitVideo) != 0:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't initialize SDL: %s",
                    sdl.getError())
    return false

  view.window = sdl.createWindow(
    view.title,
    sdl.WindowPosUndefined,
    sdl.WindowPosUndefined,
    view.width,
    view.height,
    WindowFlags)
  if view.window == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create window: %s",
                    sdl.getError())
    return false

  view.renderer = sdl.createRenderer(view.window, -1, RendererFlags)
  if view.renderer == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create renderer: %s",
                    sdl.getError())
    return false

  return true;

proc exit(view: View) =
  view.renderer.destroyRenderer()
  view.window.destroyWindow()
  sdl.quit()

proc events(pressed: var seq[sdl.Keycode]): bool =
  result = false
  var e: sdl.Event
  if pressed != nil:
    pressed = @[]

  while sdl.pollEvent(addr(e)) != 0:
    if e.kind == sdl.Quit:
      return true
    elif e.kind == sdl.KeyDown:
      if pressed != nil:
        pressed.add(e.key.keysym.sym)
      if e.key.keysym.sym == sdl.K_Escape:
        return true


proc start*(view: View) =
  var
    done = false
    pressed: seq[sdl.Keycode] = @[]
    pixels = newSeq[uint32](view.width * view.height)

  let
    totalWBytes = view.width * sizeof(uint32)

  if init(view) == false:
    return

  var texture = view.renderer.createTexture(
              sdl.PIXEL_FORMAT_ARGB8888,
              sdl.TEXTUREACCESS_STREAMING,
              view.width,
              view.height)

  if texture == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create texture: %s",
                    sdl.getError())
    return

  while not done:
    let start = epochTime()

    view.update(pixels)

    discard texture.updateTexture(nil, addr(pixels[0]), totalWBytes)
    discard view.renderer.renderCopy(texture, nil, nil)
    let fps = 1.0/(epochTime() - start)
    discard view.renderer.stringRGBA(0,
                0,
                (fps.formatFloat).cstring,
                255.uint8,
                255.uint8,
                255.uint8,
                255.uint8)
    view.renderer.renderPresent()
    done = events(pressed)

  exit(view)
