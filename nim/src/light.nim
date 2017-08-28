import  lightpkg/renderer,
        lightpkg/job,
        lightpkg/image,
        lightpkg/view,
        sequtils


const
  Title = "Light"
  ScreenW = 800
  ScreenH = ((ScreenW / 16) * 9).int

proc update(job: Job): auto =
  result = proc (pixels: var openArray[uint32]) =
    #do the rendering
    let res = render(job, PathTracing)
    #let res = render(job, RayTracing)
    #let res = render(job, NullTracing)
    for p in 0..<ScreenH * ScreenW:
      pixels[p] = res[p].toARGB()

proc main() =
  let jb = newJob(resolution = (ScreenW, ScreenH))
  let view = newView(Title, ScreenW, ScreenH, update(jb))

  if view.init() == false:
    return

  view.start()

main()

