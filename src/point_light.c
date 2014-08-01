#include <stdlib.h>
#include "point_light.h"
#include "vector3.h"
#include "color.h"

struct point_light* point_light_new(int num)
{
	struct point_light* lights;
	lights = (struct point_light*) malloc(sizeof(struct point_light) * num);

	for(int i = 0 ; i < num; ++i)
	{
		v3_init(&lights[i].position, 5, 5, -5);
		color_init(&lights[i].color, 1.0f, 1.0f, 1.0f, 1.0f);
	}
	return lights;
}

void point_light_del(struct point_light* point_light)
{
	if (point_light)
	{
		free(point_light);
	}
}
