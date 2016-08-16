#include <stdlib.h>
#include <math.h>
#include "triangle.h"
#define EPSILON 0.000005f

triangle_t*
triangle_new()
{
	triangle_t* triangles;

	triangles = (triangle_t*) malloc(sizeof(triangle_t));

	return triangles;
}

void
triangle_destroy(triangle_t **triangle)
{
	if(*triangle) {
		free(*triangle);
        *triangle = NULL;
	}
}

void
triangle_update(triangle_t* triangle)
{
	v3_sub(&triangle->edge1, &triangle->v1, &triangle->v0);
	v3_sub(&triangle->edge2, &triangle->v2, &triangle->v0);
	v3_cross(&triangle->normal, &triangle->edge1, &triangle->edge2);
	v3_normalize(&triangle->normal);
}

int
triangle_intersects(const triangle_t *triangle, const ray_t *ray, intersection_t *result)
{
	const v3_t* edge1;
    const v3_t* edge2;
	v3_t tvec;
	v3_t pvec;
	v3_t qvec;
	float det;
	float inv_det;
	float t;
	float u;
	float v;

	result->hit = 0;

	//v3_sub(&edge1, &triangle->pt2, &triangle->pt1);
	//v3_sub(&edge2, &triangle->pt3, &triangle->pt1);
	edge1 = &triangle->edge1;
	edge2 = &triangle->edge2;

	v3_cross(&pvec, &ray->direction, edge2);

	det = v3_dot(edge1, &pvec);
	//No culling version
	if(det > -EPSILON && det < EPSILON)
		return 0;

	inv_det = 1.0f / det;

	v3_sub(&tvec, &ray->origin, &triangle->v0);

	u = v3_dot(&tvec, &pvec) * inv_det;
	if(u < 0.0f || u > 1.0f)
		return 0;


	v3_cross(&qvec, &tvec, edge1);
	
	v = v3_dot(&ray->direction, &qvec) * inv_det;
	if(v < 0.0f || u + v > 1.0f + EPSILON) //add EPSILON to offset small precision errors
		return 0;

	t = v3_dot(edge2, &qvec) * inv_det;

    if(t > EPSILON) {
      result->hit = 1;
      result->distance = t;
      return 1;
    }

	/*
	v3_copy(&result->normal, &triangle->normal);
	v3_copy(&result->hit_point, &r->direction);
	v3_mul_scalar(&result->hit_point, t);
	v3_add(&result->hit_point, &result->hit_point, &r->origin);
	*/
	return 0;
}

void
triangle_scale_uni(triangle_t *tr, float scale)
{
  v3_mul_scalar(&tr->v0, &tr->v0, scale);
  v3_mul_scalar(&tr->v1, &tr->v1, scale);
  v3_mul_scalar(&tr->v2, &tr->v2, scale);
}
