import  sequtils,
        threadpool,
        cpuinfo

let num_cpus = countProcessors()

proc pmap_1*[T, S](data: seq[T], op: proc (x: T): S {.closure,gcsafe.}): seq[S]{.inline.} =
  data.mapIt(FlowVar[S], spawn op(it)).mapIt(^it)

proc worker[T, S](data: seq[T], op: proc (x: T): S {.closure.}): seq[S]{.inline.} =
  data.map(op)

proc pmap*[T, S](data: seq[T], op: proc (x: T): S {.closure.} ): seq[S]{.inline.} =
  let segments = data.distribute(num_cpus)

  segments.map(proc (segment: seq[T]): auto =
    spawn worker(segment, op)).mapIt(^it).concat
