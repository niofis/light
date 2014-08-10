#ifndef TRIANGLE_H
#define TRIANGLE_H

#include "vector3.h"
#include "material.h"
#include "ray.h"
#include "render.h"

struct triangle
{
	struct vector3 pt1;
	struct vector3 pt2;
	struct vector3 pt3;
	struct vector3 v1;
	struct vector3 v2;
	struct vector3 v3;
	struct vector3 normal;
	struct material* material;
};

struct triangle* triangle_new(int count);
void triangle_del(struct triangle* triangle);
int triangle_intersects(struct triangle* triangle, struct ray* ray, struct intersection* result);
void triangle_update(struct triangle* triangle);
#endif
