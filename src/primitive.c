#include "includes.h"

intersection_t
prm_intersect(primitive_t *prm, ray_t *ray)
{
  intersection_t nohit;
  if(!prm)
    return nohit;
  if(prm->type == TRIANGLE)
    return triangle_intersects((triangle_t*) prm->obj, ray);
  if(prm->type == SPHERE)
    return sphere_intersects((sphere_t*) prm->obj, ray);
  return nohit;
}

void
prm_destroy(primitive_t **pprm)
{
  primitive_t *prm = *pprm;
  if(prm != 0) {
    if(prm->type == TRIANGLE) {
      triangle_destroy((triangle_t**)&prm->obj);
    }
    if(prm->type == SPHERE) {
      sphere_destroy((sphere_t**)&prm->obj);
    }
    else {
      free(prm->obj);
    }
    free(prm);
  }
  *pprm = NULL;
}

primitive_t*
prm_from_triangle(triangle_t *triangle)
{
  primitive_t *prm = (primitive_t*) malloc(sizeof(primitive_t));
  
  prm->type = TRIANGLE;
  prm->obj = triangle;
  
  return prm;
}

primitive_t*
prm_from_sphere(sphere_t *sphere)
{
  primitive_t *prm = (primitive_t*) malloc(sizeof(primitive_t));
  
  prm->type = SPHERE;
  prm->obj = sphere;

  return prm;
}
