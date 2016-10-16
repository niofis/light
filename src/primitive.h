#pragma once

#define SPHERE      1
#define TRIANGLE    2

#include "includes.h"

struct primitive
{

	int type;
	void* obj;
	struct primitive* next;
};
typedef struct primitive primitive_t;


int 
  prm_intersect(primitive_t *prm, ray_t *ray, intersection_t *result);

void
  prm_destory(primitive_t **prm);
