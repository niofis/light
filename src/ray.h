#ifndef RAY_H
#define RAY_H

#include "vector3.h"

typedef struct ray
{
	struct vector3 direction;
	struct vector3 origin;
};

struct ray* ray_new();
void ray_delete();

#endif