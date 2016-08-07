#include <stdlib.h>
#include "point_light.h"
#include "vector3.h"
#include "color.h"

point_light_t*
point_light_new(int num)
{
	point_light_t* lights;
	lights = (point_light_t*) malloc(sizeof(point_light_t) * num);

	for(int i = 0 ; i < num; ++i)
	{
		v3_set_xyz(&lights[i].position, 5, 5, -5);
		color_set_argb(&lights[i].color, 1.0f, 1.0f, 1.0f, 1.0f);
	}
	return lights;
}

void
point_light_destroy(point_light_t **point_light)
{
	if (*point_light)
	{
		free(*point_light);
        *point_light = NULL;
	}
}
