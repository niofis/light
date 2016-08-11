#include <stdlib.h>
#include <math.h>
#include "triangle.h"
#define EPSILON 0.000001f

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
	v3_sub(&triangle->edge1, &triangle->pt2, &triangle->pt1);
	v3_sub(&triangle->edge2, &triangle->pt3, &triangle->pt1);
	v3_cross(&triangle->normal, &triangle->edge1, &triangle->edge2);
	v3_normalize(&triangle->normal);
}

int
triangle_intersects_old(const triangle_t *triangle, const ray_t *ray, intersection_t *result)
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

	v3_sub(&tvec, &ray->origin, &triangle->pt1);

	u = v3_dot(&tvec, &pvec) * inv_det;
	if(u < 0.0f || u > 1.0f)
		return 0;

	v3_cross(&qvec, &tvec, edge1);
	
	v = v3_dot(&ray->direction, &qvec) * inv_det;
	if(v < 0.0f || u + v > 1.0f)
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

int
triangle_intersects(const triangle_t *triangle, const ray_t *ray, intersection_t *result)
{
  //from: http://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/barycentric-coordinates

	const v3_t* v0v1;
    const v3_t* v0v2;
	v3_t tvec;
	v3_t pvec;
	v3_t qvec;
    v3_t N;
	float u;
	float v;
    float t;

	result->hit = 0;

	//v3_sub(&edge1, &triangle->pt2, &triangle->pt1);
	//v3_sub(&edge2, &triangle->pt3, &triangle->pt1);
	v0v1 = &triangle->edge1;
	v0v1 = &triangle->edge2;

    v3_cross(&N, v0v1, v0v2);
    float denom = v3_dot(&N, &N);

    float NdotRayDirection = v3_dot(&N, &ray->direction);
    if(fabs(NdotRayDirection) < EPSILON)
      return 0;

    float d = v3_dot(&N, &triangle->pt1);

    t = (v3_dot(&N, &ray->origin) + d) / NdotRayDirection;
    if(t < 0)
      return 0;

    v3_t P;

    v3_mul_scalar(&P, &ray->direction, t);
    v3_add(&P, &P, &ray->origin);

    v3_t C;
    v3_t edge0;

    v3_sub(&edge0, &triangle->pt2, &triangle->pt1);

    v3_t vp0;
    v3_sub(&vp0, &P, &triangle->pt1);

    v3_cross(&C, &edge0, &vp0);
    if(v3_dot(&N, &C) < 0)
      return 0;

    v3_t edge1;
    v3_sub(&edge1, &triangle->pt3, &triangle->pt2);

    v3_t vp1;
    v3_sub(&vp1, &P, &triangle->pt2);

    v3_cross(&C, &edge1, &vp1);

    u = v3_dot(&N, &C);
    if(u < 0)
      return 0;

    v3_t edge2;
    v3_sub(&edge2, &triangle->pt1, &triangle->pt3);

    v3_t vp2;
    v3_sub(&vp2, &P, &triangle->pt3);

    v3_cross(&C, &edge2, &vp2);

    v = v3_dot(&N, &C);
    if(v < 0)
      return 0;

    result->hit = 1;
      result->distance = t;

printf("%f ", t);
	return 1;
}


void
triangle_scale_uni(triangle_t *tr, float scale)
{
  v3_mul_scalar(&tr->pt1, &tr->pt1, scale);
  v3_mul_scalar(&tr->pt2, &tr->pt2, scale);
  v3_mul_scalar(&tr->pt3, &tr->pt3, scale);
}
