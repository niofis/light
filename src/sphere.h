#ifndef SPHERE_H
#define SPHERE_H
#include "vector3.h"
#include "render.h"
#include "ray.h"
#include "material.h"

struct sphere
{
	struct vector3 center;
	struct material* material;
	float radius;
};

struct sphere* sphere_new(int count);
void sphere_del(struct sphere* sphere);
//just for test purposes, intersection returns: true or false
int sphere_intersects(struct sphere* sphere, struct ray* ray, struct intersection* result);
#endif
