#ifndef RENDER_H
#define RENDER_H

#include "vector3.h"
#include "color.h"
#include "job.h"


struct result
{
	struct color color;
	struct vector3 point;
	int hit;
};


struct hit_result
{
	struct vector3 hit_point;
	struct vector3 normal;
	float distance;
	int hit;
};

int render(struct job_desc* job);




#endif
