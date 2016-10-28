#include "includes.h"

#define min(a,b) (a<b?a:b)
#define max(a,b) (a>b?a:b)
//#define min3(a,b,c) (a<b?(a<c?a:c):(b<c?b:c))
//#define max3(a,b,c) (a>b?(a>c?a:c):(b>c?b:c))
#define min3(a,b,c) min(min(a,b),c)
#define max3(a,b,c) max(max(a,b),c)

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

aabb_t*
aabb_new_from_sphere(sphere_t *sphere)
{
  aabb_t *bb = (aabb_t*) malloc(sizeof(aabb_t));
  
  bb->min.x = sphere->center.x - sphere->radius;
  bb->min.y = sphere->center.y - sphere->radius;
  bb->min.z = sphere->center.z - sphere->radius;

  bb->max.x = sphere->center.x + sphere->radius;
  bb->max.y = sphere->center.y + sphere->radius;
  bb->max.z = sphere->center.z + sphere->radius;

  return bb;
}

aabb_t*
aabb_new_from_primitive(primitive_t *prm)
{
  if(!prm)
    return NULL;
  if(prm->type == TRIANGLE)
    return aabb_new_from_triangle((triangle_t*) prm->obj);
  if(prm->type == SPHERE)
    return aabb_new_from_sphere((sphere_t*) prm->obj);
  return NULL;
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

int 
aabb_intersect(aabb_t *bb, ray_t *ray)
{
  float tmin = -1e16f, tmax = 1e16;

  /*
     for (int i = 0; i < 3; ++i) {
     if (ray.dir[i] != 0.0) {
     double t1 = (b.min[i] - r.origin[i])/r.dir[i];
     double t2 = (b.max[i] - r.origin[i])/r.dir[i];

     tmin = max(tmin, min(t1, t2));
     tmax = min(tmax, max(t1, t2));
     } else if (ray.origin[i] <= b.min[i] || ray.origin[i] >= b.max[i]) {
     return false;
     }
     }
     */

  if (ray->direction.x != 0.0f) {
    float t1 = (bb->min.x - ray->origin.x) / ray->direction.x;
    float t2 = (bb->max.x - ray->origin.x) / ray->direction.x;
    tmin = min3(tmin,t1,t2);
    tmax = max3(tmax,t1,t2);
  }

  if (ray->direction.y != 0.0f) {
    float t1 = (bb->min.y - ray->origin.y) / ray->direction.y;
    float t2 = (bb->max.y - ray->origin.y) / ray->direction.y;
    tmin = min3(tmin,t1,t2);
    tmax = max3(tmax,t1,t2);
  }

  if (ray->direction.z != 0.0f) {
    float t1 = (bb->min.z - ray->origin.z) / ray->direction.z;
    float t2 = (bb->max.z - ray->origin.z) / ray->direction.z;
    tmin = min3(tmin,t1,t2);
    tmax = max3(tmax,t1,t2);
  }

  return tmax > tmin && tmax > 0.0;
}
