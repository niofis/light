#pragma once
#include "includes.h"

typedef struct
{
	int width;
	int height;
	int *buffer;
	world_t *world;
} job_t;

job_t*
  job_new();

void
  job_destroy(job_t **job);
