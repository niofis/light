#include <stdlib.h>
#include "scene.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"

struct scene* scene_new()
{
	struct scene *scn;

	scn = (struct scene*) malloc(sizeof(struct scene));

	scn->camera = camera_new();
	scn->num_spheres = 1;
	scn->spheres = sphere_new(scn->num_spheres);
	v3_init(&scn->spheres[0].center, 1.0f, 0.0f, 0.0f);
	scn->spheres[0].radius = 1.0f;

	scn->point_lights = point_light_new();
	scn->num_point_lights = 1;

	//scn->triangles = (struct triangle*) malloc(sizeof(struct triangle) * triangles_count);

	return scn;
}

void scene_del(struct scene* scn)
{
	if (scn->camera)
	{
		camera_del(scn->camera);
		scn->camera = 0;
	}

	if (scn->spheres)
	{
		sphere_del(scn->spheres);
		scn->spheres = 0;
		scn->num_spheres = 0;
	}

	if (scn->point_lights)
	{
		point_light_delete(scn->point_lights);
		scn->point_lights = 0;
		scn->num_point_lights = 0;
	}
}
