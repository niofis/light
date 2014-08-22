#ifndef PRIMITIVE_H
#define PRIMITIVE_H

#define SPHERE
#define TRIANGLE

#include "triangle.h"
#include "sphere.h"

struct primitive
{

	int type;
	void* obj;
	struct primitive* next;
};


int prm_intersect(struct primitive* prm, struct ray* ray, struct intersection* result);

#endif
