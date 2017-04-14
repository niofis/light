import math, strutils, sequtils, color

proc savePPM*(filename: string, data: seq[seq[Color]]) =
  let ppm = open(filename, fmWrite)
  ppm.write(format("P3\n$# $#\n255\n",data[0].len, data.len))
  for row in data:
    for c in row:
      ppm.write(format("$# $# $# ",
        int(floor(c.r * 255.99'f32)),
        int(floor(c.g * 255.99'f32)),
        int(floor(c.b * 255.99'f32))))
    ppm.write("\n")
  ppm.close()
