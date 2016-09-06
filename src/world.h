#pragma once

#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "list.h"
#include "bvh.h"

typedef struct
{
	camera_t *camera;
	list_t *triangles;
	list_t *lights;
	list_t *materials;
    bvh_t *bvh;
} world_t;

world_t*
  world_new();

world_t*
  world_cornell();

world_t*
  world_from_model(const char *file);

void
  world_destroy(world_t **world);
