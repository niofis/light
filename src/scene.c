#include <stdlib.h>
#include "scene.h"
#include "material.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"

struct scene* scene_new()
{
	struct scene *scn;

	scn = (struct scene*) malloc(sizeof(struct scene));

	scn->camera = camera_new();

	scn->num_materials =  3;
	scn->materials = material_new(scn->num_materials);
	color_init(&(scn->materials[0].color), 1.0f, 1.0f, 0.0f, 0.0f);
	color_init(&(scn->materials[1].color), 1.0f, 0.0f, 1.0f, 0.0f);
	color_init(&(scn->materials[2].color), 0.0f, 0.0f, 0.0f, 1.0f);

	scn->num_spheres = 3;
	scn->spheres = sphere_new(scn->num_spheres);
	
	v3_init(&scn->spheres[0].center, 2.0f, 0.0f, 0.0f);
	scn->spheres[0].radius = 1.0f;
	scn->spheres[0].material = &(scn->materials[0]);
	
	v3_init(&scn->spheres[1].center, -2.0f, 0.0f, 0.0f);
	scn->spheres[1].radius = 1.0f;
	scn->spheres[1].material = &(scn->materials[1]);

	v3_init(&scn->spheres[2].center, 0.0f, 2.0f, 0.0f);
	scn->spheres[2].radius = 1.0f;
	scn->spheres[2].material = &(scn->materials[2]);


	scn->num_point_lights = 1; //2;
	scn->point_lights = point_light_new(scn->num_point_lights);
	
	//scn->point_lights[1].position.x = -5.0f;


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
		point_light_del(scn->point_lights);
		scn->point_lights = 0;
		scn->num_point_lights = 0;
	}

	if(scn->materials)
	{
		for(int i = 0; i < scn->num_materials; ++i)
		{
			material_del(&(scn->materials[i]));
		}
		scn->materials = 0;
	}
}
