#ifdef DEBUG
#include <stdio.h>
#include "debug.h"
#include "ray.h"

void debug_nl()
{
	printf("\n");
}

void debug_pause()
{
	//system("pause");
}

void ray_dump(struct ray* ray)
{
	printf("{\norigin: \n{\n");
	v3_dump(&ray->origin);
	printf("},\ndirection: \n{\n");
	v3_dump(&ray->direction);
	printf("}\n}\n");
}
#endif
