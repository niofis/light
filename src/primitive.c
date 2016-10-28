#include "includes.h"

int 
prm_intersect(primitive_t *prm, ray_t *ray, intersection_t *result)
{
  if(!prm)
    return 0;
  if(prm->type == TRIANGLE)
    return triangle_intersects((triangle_t*) prm->obj, ray, result);
  if(prm->type == SPHERE)
    return sphere_intersects((sphere_t*) prm->obj, ray, result);
  return 0;
}

void
prm_destroy(primitive_t **pprm)
{
  primitive_t *prm = *prm;
  if(prm != 0) {
    if(prm->type == TRIANGLE) {
      triangle_del(&prm->obj);
    }
    if(prm->type == SPHERE) {
      sphere_del(&prm->obj);
    }
    else {
      free(prm->obj);
    }
    free(prm);
  }
  *pprm = NULL;
}
