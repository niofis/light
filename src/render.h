#ifndef RENDER_H
#define RENDER_H

#include "color.h"


typedef struct result
{
	struct color color;
	struct vector3 point;
	int hit;
};


typedef struct hit_result
{
	struct vector3 hit_point;
	struct vector3 normal;
	float distance;
	int hit;
};

int render(struct job_desc* job);




#endif