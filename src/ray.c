#include <stdlib.h>
#include "ray.h"

ray_t*
ray_new()
{
	ray_t* ray;
	ray = (ray_t*) malloc(sizeof(ray_t));
	v3_set_xyz(&ray->origin, 0.0f, 0.0f, 0.0f);
	v3_set_xyz(&ray->direction, 0.0f, 0.0f, 0.0f);
	return ray;
}

void
ray_delete(ray_t **r)
{
	if (*r)
	{
		free(*r);
        *r = NULL;
	}
}

