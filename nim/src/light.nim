import  lightpkg/renderer,
        lightpkg/job,
        lightpkg/image,
        sdl2/sdl,
        sdl2/sdl_gfx_primitives as gfx,
        times,
        strutils



const
  Title = "Light"
  ScreenW = 1280
  ScreenH = 720
  WindowFlags = 0
  RendererFlags = sdl.RendererAccelerated

type
  App = ref AppObj
  AppObj = object
    window*: sdl.Window
    renderer*: sdl.Renderer

proc init(app: App): bool =
  if sdl.init(sdl.InitVideo) != 0:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't initialize SDL: %s",
                    sdl.getError())
    return false

  app.window = sdl.createWindow(
    Title,
    sdl.WindowPosUndefined,
    sdl.WindowPosUndefined,
    ScreenW,
    ScreenH,
    WindowFlags)
  if app.window == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create window: %s",
                    sdl.getError())
    return false

  app.renderer = sdl.createRenderer(app.window, -1, RendererFlags)
  if app.renderer == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create renderer: %s",
                    sdl.getError())
    return false

  return true;

proc exit(app: App) =
  app.renderer.destroyRenderer()
  app.window.destroyWindow()
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


proc main() =
  var
    app = App(window: nil, renderer: nil)
    done = false
    pressed: seq[sdl.Keycode] = @[]
    pixels: array[ScreenW * ScreenH, uint32]

  if init(app) == false:
    return

  let jb = newJob()

  #savePPM("image.ppm", res)
  #echo res
  

  var texture = app.renderer.createTexture(
              sdl.PIXEL_FORMAT_ARGB8888,
              sdl.TEXTUREACCESS_STREAMING,
              ScreenW,
              ScreenH)

  if texture == nil:
    sdl.logCritical(sdl.LogCategoryError,
                    "Can't create texture: %s",
                    sdl.getError())
    return

  

  while not done:
    var start= cpuTime()
    #do the rendering

    let res = render(jb)

    #for y in 0..<res.high:
    #    for x in 0..<res[0].high:
    #      pixels[y * ScreenW + x] = 0
    #        res[y][x].toARGB()


    discard texture.updateTexture(nil, addr(pixels), ScreenW * sizeof(uint32))
    discard app.renderer.renderCopy(texture, nil, nil)
    let fps = 1.0/(cpuTime() - start)
    discard app.renderer.stringRGBA(0,
                0,
                (fps.formatFloat).cstring,
                255.uint8,
                255.uint8,
                255.uint8,
                255.uint8)
    app.renderer.renderPresent()
    done = events(pressed)

  exit(app)

main()

