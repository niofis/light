#include <stdlib.h>
#include "point_light.h"
#include "vector3.h"
#include "color.h"

struct point_light* point_light_new()
{
	struct point_light* light;
	light = (struct point_light*) malloc(sizeof(struct point_light));

	v3_init(&light->position, 5, 5, -5);
	color_init(&light->color, 1.0f, 1.0f, 1.0f, 1.0f);

	return light;
}

void point_light_delete(struct point_light* point_light)
{
	if (point_light)
	{
		free(point_light);
	}
}