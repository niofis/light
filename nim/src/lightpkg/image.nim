import math, strutils, sequtils, color

type Image* = ref object of RootObj
  width*: int
  height*: int
  pixels*: seq[Color]
  count*: int

proc newImage*(width: int, height: int, fill: Color = color.Black): Image =
  var img = Image(width: width, height:height, count:0)
  img.pixels = newSeq[Color](width * height)
  for px in 0..<img.pixels.len:
    img.pixels[px] = fill
  return img

proc add*(image: Image, frame: seq[Color]) =
  for i in 0..<image.pixels.len:
    image.pixels[i] = image.pixels[i] + frame[i]
  image.count = image.count + 1


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
