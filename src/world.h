#ifndef world_H
#define world_H


struct world
{
	struct camera* camera;
	struct triangle* triangles;
	struct sphere* spheres;
	struct point_light* point_lights;
	struct material* materials;
	int num_triangles;
	int num_spheres;
	int num_point_lights;
	int num_materials;
};

struct world* world_new();
void world_del(struct world*);

#endif
