#include "aabb.h"

aabb_t*
aabb_from_triangle(triangle_t* triangle)
{
  aabb_t *bb = (aabb_t*) malloc(sizeof(aabb_t));
  bb->min.x = min(triangle->v0.x, triangle->v1.x);
  bb->min.x = min(bb->min.x, triangle->v2.x)
  return bb;
}

aabb_t*
aabb_add(aabb_t *dest, aabb_t *bb1, aabb_t *bb2)
{
}

void
aabb_destroy(aabb_t *bb)
{
  if(*bb) {
    free(*bb);
    *bb = NULL;
  }
}
