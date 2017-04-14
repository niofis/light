import lightpkg/renderer, lightpkg/job, lightpkg/image

let jb = newJob()

let res = render(jb)
savePPM("image.ppm", res)
