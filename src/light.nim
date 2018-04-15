import  lightpkg/renderer,
        lightpkg/job,
        lightpkg/image,
        lightpkg/view,
        sequtils


const
  Title = "Light"
  ScreenW = 800
  ScreenH = ((ScreenW / 16) * 9).int

var times = 5
  
proc update(job: Job): auto =
  var imgBuffer = newImage(ScreenW, ScreenH)
  result = proc (pixels: var seq[uint32]) =
    if times == 0:
      return
    #do the rendering
    #let res = render(job, PathTracing)
    let res = render(job, RayTracing)
    #let res = render(job, NullTracing)
    imgBuffer.add(res)
    
    for p in 0..<ScreenH * ScreenW:
      pixels[p] = (imgBuffer.pixels[p] / imgBuffer.count.float32).toARGB()

    times = times - 1

proc main() =
  let jb = newJob(resolution = (ScreenW, ScreenH))
  let view = newView(Title, ScreenW, ScreenH, update(jb))

  view.start()

main()

