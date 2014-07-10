#ifndef SPHERE_H
#define SPHERE_H
#include "../vector3.h"

typedef struct sphere
{
	struct vector3 center;
	float radius;
};

struct sphere* sphere_new(int count);
void sphere_del(struct sphere* spheres);
//just for test purposes, intersection returns: true or false
struct hit_result sphere_intersects(struct sphere* sphere, struct ray* ray);
#endif
