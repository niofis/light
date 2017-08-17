import  lightpkg/renderer,
        lightpkg/job,
        lightpkg/image,
        lightpkg/view,
        sequtils


const
  Title = "Light"
  ScreenW = 800
  ScreenH = ((ScreenW / 16) * 9).int

proc update() =
  #do the rendering
  #let res = render(jb, RayTracing)
  #let res = render(jb, NullTracing)
  #for p in 0..<ScreenH * ScreenW:
  #  pixels[p] = res[p].toARGB()
  echo "updating view"

proc main() =
  var view = newView(Title, ScreenW, ScreenH, update)

  if view.init() == false:
    return

  view.start()

  #let jb = newJob(resolution = (ScreenW, ScreenH))

main()

