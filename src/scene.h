#ifndef SCENE_H
#define SCENE_H


typedef struct scene
{
	struct camera* camera;
	struct triangle* triangles;
	struct sphere* spheres;
	struct point_light* point_lights;
	int num_triangles;
	int num_spheres;
	int num_point_lights;
};

struct scene* scene_new();
void scene_del(struct scene*);

#endif