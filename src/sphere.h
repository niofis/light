#ifndef SPHERE_H
#define SPHERE_H
#include "vector3.h"
#include "render.h"
#include "ray.h"

struct sphere
{
	struct vector3 center;
	float radius;
};

struct sphere* sphere_new(int count);
void sphere_del(struct sphere* spheres);
//just for test purposes, intersection returns: true or false
int sphere_intersects(struct sphere* sphere, struct ray* ray, struct trace_data* result);
#endif
