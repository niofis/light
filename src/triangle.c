#include <stdlib.h>
#include "triangle.h"

struct triangle* triangle_new(int count)
{
	struct triangle* triangles;

	triangles = (struct triangle*) malloc(sizeof(struct triangle) * count);

	return triangles;
}

void triangle_del(struct triangle* triangle)
{
	if(triangle)
	{
		free(triangle);
	}
}

int triangle_intersects(struct triangle* triangle, struct ray* ray, struct intersection* result)
{
	return 0;
}

