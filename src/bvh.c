#include <stdlib.h>
#include "list.h"
#include "bvh.h"

typedef enum {X_AXIS, Y_AXIS, Z_AXIS} axis_t;

bvh_t *
bvh_build(

bvh_t*
bvh_new(const list_t *triangles)
{
  bvh_t *bvh = (bvh_t*)malloc(sizeof(bvh_t));

  return bvh;
}
