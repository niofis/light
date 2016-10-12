#include "includes.h"

point_light_t*
point_light_new()
{
	point_light_t* light = (point_light_t*) malloc(sizeof(point_light_t));

	v3_set_xyz(&light->position, 5, 5, -5);
	color_set_argb(&light->color, 1.0f, 1.0f, 1.0f, 1.0f);
	
    return light;
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
