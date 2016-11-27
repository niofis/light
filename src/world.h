#pragma once
#include "includes.h"

typedef struct
{
	camera_t *camera;
	list_t *primitives;
	list_t *lights;
	list_t *materials;
    bvh_t *bvh;
} world_t;

world_t*
  world_new();

world_t*
  world_cornell();

world_t*
  world_demo();

world_t*
  world_from_model(const char *file);

void
  world_destroy(world_t **world);
