#ifndef RENDER_H
#define RENDER_H

#include "material.h"
#include "vector3.h"
#include "color.h"
#include "job.h"
#include "ray.h"

struct intersection
{
	struct vector3 hit_point;
	struct vector3 normal;
	struct material* material;
	float distance;
	int hit;
};

int render(struct job_desc* job);
#endif
