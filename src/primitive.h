#pragma once

#define SPHERE      1
#define TRIANGLE    2

#include "triangle.h"

struct primitive
{

	int type;
	void* obj;
	struct primitive* next;
};


int prm_intersect(struct primitive* prm, struct ray* ray, struct intersection* result);
void prm_del(struct primitive* prm);
