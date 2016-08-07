#include <stdlib.h>
#include "triangle.h"

triangle_t*
triangle_new(int count)
{
	triangle_t* triangles;

	triangles = (triangle_t*) malloc(sizeof(triangle_t) * count);

	return triangles;
}

void
triangle_destroy(triangle_t **triangle)
{
	if(*triangle)
	{
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
	if(det > -0.01f && det < 0.01f)
	{
		return 0;
	}

	inv_det = 1.0f / det;

	v3_sub(&tvec, &ray->origin, &triangle->pt1);

	u = v3_dot(&tvec, &pvec) * inv_det;
	if(u < 0.0f || u > 1.0f)
	{
		return 0;
	}

	v3_cross(&qvec, &tvec, edge1);
	
	v = v3_dot(&ray->direction, &qvec) * inv_det;
	if(v < 0.0f || u + v > 1.0f)
	{
		return 0;
	}

	t = v3_dot(edge2, &qvec) * inv_det;

	result->hit = 1;
	result->distance = t;

	/*
	v3_copy(&result->normal, &triangle->normal);
	v3_copy(&result->hit_point, &r->direction);
	v3_mul_scalar(&result->hit_point, t);
	v3_add(&result->hit_point, &result->hit_point, &r->origin);
	*/
	return 1;
}

