#include <stdlib.h>
#include "triangle.h"

struct triangle* triangle_new(int count)
{
	struct triangle* triangles;

	triangles = (struct triangle*) malloc(sizeof(struct triangle) * count);

	return triangles;
}

void triangle_del(struct triangle* triangle)
{
	if(triangle)
	{
		free(triangle);
	}
}

void triangle_update(struct triangle* triangle)
{
	v3_copy(&triangle->v1, &triangle->pt1);
	v3_sub(&triangle->v2, &triangle->pt2, &triangle->pt1);
	v3_sub(&triangle->v3, &triangle->pt3, &triangle->pt1);
	v3_cross(&triangle->normal, &triangle->v2, &triangle->v3);
	v3_normalize(&triangle->normal);
}

int triangle_intersects(struct triangle* triangle, struct ray* ray, struct intersection* result)
{
	struct vector3 edge1;
	struct vector3 edge2;
	struct vector3 tvec;
	struct vector3 pvec;
	struct vector3 qvec;
	float det;
	float inv_det;
	float t;
	float u;
	float v;

	result->hit = 0;

	v3_sub(&edge1, &triangle->pt2, &triangle->pt1);
	v3_sub(&edge2, &triangle->pt3, &triangle->pt1);

	v3_cross(&pvec, &ray->direction, &edge2);

	det = v3_dot(&edge1, &pvec);
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

	v3_cross(&qvec, &tvec, &edge1);
	
	v = v3_dot(&ray->direction, &qvec) * inv_det;
	if(v < 0.0f || u + v > 1.0f)
	{
		return 0;
	}

	t = v3_dot(&edge2, &qvec) * inv_det;

	result->hit = 1;
	result->distance = t;
	v3_copy(&result->normal, &triangle->normal);
	
	return 1;
}

