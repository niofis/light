#include <stdlib.h>
#include "ray.h"

struct ray* ray_new()
{
	struct ray* ray;
	ray = (struct ray*) malloc(sizeof(struct ray));
	v3_init(&ray->origin, 0.0f, 0.0f, 0.0f);
	v3_init(&ray->direction, 0.0f, 0.0f, 0.0f);
	return ray;
}

void ray_delete(struct ray* ray)
{
	if (ray)
	{
		free(ray);
	}
}

