#ifndef CAMERA_H
#define CAMERA_H

#include "vector3.h"

struct camera
{
	struct vector3 left_top;
	struct vector3 left_bottom;
	struct vector3 right_top;
	struct vector3 eye;
};

struct camera* camera_new();
void camera_del(struct camera*);

#endif
