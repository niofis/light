#ifndef RENDER_H
#define RENDER_H

#include "vector3.h"
#include "color.h"
#include "job.h"
#include "ray.h"

struct trace_data 
{
	struct vector3 hit_point;
	struct vector3 normal;
	struct color color;
	struct ray ray; //inbound ray
	float distance;
	int hit;
};

int render(struct job_desc* job);




#endif
