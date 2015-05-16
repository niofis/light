#ifndef world_H
#define world_H

#include "triangle_list.h"

struct world
{
	struct camera* camera;
	//struct triangle* triangles;
	struct sphere* spheres;
	struct point_light* point_lights;
	struct material* materials;
	struct triangle_list* triangles;
	int num_spheres;
	int num_point_lights;
	int num_materials;
};

struct world* world_new();
struct world* world_cornell();
void world_del(struct world*);

#endif
