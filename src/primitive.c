#include "includes.h"

int 
prm_intersect(primitive_t *prm, ray_t *ray, intersection_t *result)
{
  return 0;
}

void
prm_destroy(primitive_t **pprm)
{
  primitive_t *prm = *prm;
  if(prm != 0) {
    if(prm->type == TRIANGLE) {
      triangle_del(prm->obj);
    }
    else {
      free(prm->obj);
    }
    free(prm);
  }
  *pprm = NULL;
}

