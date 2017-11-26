import  sequtils,
        threadpool,
        cpuinfo

let num_cpus = countProcessors()

proc pmap_1*[T, S](data: seq[T], op: proc (x: T): S {.closure,gcsafe.}): seq[S]{.inline.} =
  data.mapIt(FlowVar[S], spawn op(it)).mapIt(^it)

proc worker[T, S](data: seq[T], op: proc (x: T): S {.closure.}): seq[S]{.inline.} =
  data.map(op)

proc pmap*[T, S](data: seq[T], op: proc (x: T): S {.closure.} ): seq[S]{.inline.} =
  let
    num_blocks = num_cpus * 2
    seg_size = int(data.len / num_blocks)

  var 
    segments = newSeq[seq[T]](num_blocks)
    i = 0
    start_idx = 0
    end_idx = seg_size

  while end_idx < data.len:
      segments[i] = data[start_idx..<end_idx]
      inc(i)
      start_idx = end_idx
      end_idx = end_idx + seg_size

  if start_idx < data.len:
    segments[i] = data[start_idx..<data.len]

  segments.map(proc (segment: seq[T]): auto =
    spawn worker(segment, op)).mapIt(^it).concat
  
  #data.map(op)

  #data.mapIt(FlowVar[S], spawn op(it)).mapIt(^it)
  #  newSeq(result, data.len)
  #  var vals = newSeq[FlowVar[S]](data.len)

  #  for i in 0..data.high:
  #      vals[i] = spawn op(data[i])
  #  sync()

  #  for i in 0..data.high:
  #      var res = ^vals[i]
  #      result[i] = res
