#include <stdlib.h>
#include "aabb.h"


#define min(a,b) (a<b?a:b)
#define max(a,b) (a>b?a:b)
#define min3(a,b,c) (a<b?(a<c?a:c):(b<c?b:c))
#define max3(a,b,c) (a>b?(a>c?a:c):(b>c?b:c))

void
aabb_update_centroid(aabb_t *bb)
{
  bb->centroid.x = (bb->min.x + bb->min.x) / 2.0f;
  bb->centroid.y = (bb->min.y + bb->min.y) / 2.0f;
  bb->centroid.z = (bb->min.z + bb->min.z) / 2.0f;
}

aabb_t*
aabb_new_from_triangle(triangle_t* triangle)
{
  aabb_t *bb = (aabb_t*) malloc(sizeof(aabb_t));

  bb->min.x = min3(triangle->v0.x, triangle->v1.x, triangle->v2.x);
  bb->min.y = min3(triangle->v0.y, triangle->v1.y, triangle->v2.y);
  bb->min.z = min3(triangle->v0.z, triangle->v1.z, triangle->v2.z);

  bb->max.x = max3(triangle->v0.x, triangle->v1.x, triangle->v2.x);
  bb->max.y = max3(triangle->v0.y, triangle->v1.y, triangle->v2.y);
  bb->max.z = max3(triangle->v0.z, triangle->v1.z, triangle->v2.z);

  aabb_update_centroid(bb);
  
  return bb;
}

void
aabb_fit_triangle(aabb_t *bb, triangle_t *triangle)
{
  bb->min.x = min3(triangle->v0.x, triangle->v1.x, triangle->v2.x);
  bb->min.y = min3(triangle->v0.y, triangle->v1.y, triangle->v2.y);
  bb->min.z = min3(triangle->v0.z, triangle->v1.z, triangle->v2.z);

  bb->max.x = max3(triangle->v0.x, triangle->v1.x, triangle->v2.x);
  bb->max.y = max3(triangle->v0.y, triangle->v1.y, triangle->v2.y);
  bb->max.z = max3(triangle->v0.z, triangle->v1.z, triangle->v2.z);

  aabb_update_centroid(bb);
}

void
aabb_combine(aabb_t *dest, aabb_t *bb1, aabb_t *bb2)
{
  dest->min.x = min(bb1->min.x, bb2->min.x);
  dest->min.y = min(bb1->min.y, bb2->min.y);
  dest->min.z = min(bb1->min.z, bb2->min.z);
  
  dest->max.x = max(bb1->max.x, bb2->max.x);
  dest->max.y = max(bb1->max.y, bb2->max.y);
  dest->max.z = max(bb1->max.z, bb2->max.z);

  aabb_update_centroid(dest);
}

void
aabb_destroy(aabb_t **bb)
{
  if(*bb) {
    free(*bb);
    *bb = NULL;
  }
}
