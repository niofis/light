import  lightpkg/renderer,
        lightpkg/job,
        lightpkg/image,
        lightpkg/view,
        sequtils


const
  Title = "Light"
  ScreenW = 800
  ScreenH = ((ScreenW / 16) * 9).int

proc main() =
  let jb = newJob(resolution = (ScreenW, ScreenH))

  proc update(pixels: var openArray[uint32]) =
    #do the rendering
    let res = render(jb, PathTracing)
    #let res = render(jb, RayTracing)
    #let res = render(jb, NullTracing)
    for p in 0..<ScreenH * ScreenW:
      pixels[p] = res[p].toARGB()

  var view = newView(Title, ScreenW, ScreenH, update)

  if view.init() == false:
    return

  view.start()

  echo "Done"

main()

