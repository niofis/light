#pragma once

#include "vector3.h"
#include "material.h"
#include "ray.h"
#include "render.h"

typedef struct
{
	struct vector3 pt1;
	struct vector3 pt2;
	struct vector3 pt3;
	struct vector3 v1;
	struct vector3 edge1;
	struct vector3 edge2;
	struct vector3 normal;
	struct material* material;
} triangle;

triangle* triangle_new(int count);
void triangle_destroy(triangle **tr);
int triangle_intersects(triangle *tr, struct ray *ray, struct intersection *result);
void triangle_update(triangle *tr);
