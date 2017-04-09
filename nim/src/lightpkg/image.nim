proc writePPM(data: seq[seq[Color]]) =
  let ppm = open("nimrb.ppm", fmWrite)
  ppm.write(format("P3\n$# $#\n255\n",WIDTH, HEIGHT))
  for row in data:
    for c in row:
      ppm.write(format("$# $# $# ",
        int(floor(c.r * 255.99'f32)),
        int(floor(c.g * 255.99'f32)),
        int(floor(c.b * 255.99'f32))))
    ppm.write("\n")
  ppm.close()
