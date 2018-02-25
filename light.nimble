# Package

version       = "0.1.0"
author        = "niofis"
description   = "Path tracer"
license       = "MIT"

srcDir        = "src"
bin           = @["light"]

# Dependencies

requires "nim >= 0.16.0", "sdl2_nim >= 2.0.5.0"

skipFiles = @["how.nim"]
skipDirs = @["sdl2/private"]