#include <stdlib.h>
#include "scene.h"
#include "material.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "triangle.h"

struct scene* scene_new()
{
	struct scene *scn;

	scn = (struct scene*) malloc(sizeof(struct scene));

	scn->camera = camera_new();

	scn->num_materials =  4;
	scn->materials = material_new(scn->num_materials);
	color_init(&(scn->materials[0].color), 1.0f, 1.0f, 0.0f, 0.0f);
	color_init(&(scn->materials[1].color), 1.0f, 0.0f, 1.0f, 0.0f);
	color_init(&(scn->materials[2].color), 1.0f, 0.0f, 0.0f, 1.0f);
	color_init(&(scn->materials[3].color), 1.0f, 1.0f, 1.0f, 1.0f);
	
	scn->num_spheres = 4;
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

	v3_init(&scn->spheres[3].center, 0.0f, -1001.0f, 0.0f);
	scn->spheres[3].radius = 1000.0f;
	scn->spheres[3].material = &(scn->materials[3]);

	scn->num_triangles = 1;
	scn->triangles = triangle_new(scn->num_triangles);
	v3_init(&scn->triangles[0].pt1, 4.0f, 0.0f, 0.10f);
	v3_init(&scn->triangles[0].pt2, 5.0f, 2.0f, 0.10f);
	v3_init(&scn->triangles[0].pt3, 6.0f, 0.0f, 0.10f);
	scn->triangles[0].material = &(scn->materials[1]);

	triangle_update(&scn->triangles[0]);


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

	if(scn->triangles)
	{
		triangle_del(scn->triangles);
		scn->triangles = 0;
		scn->num_triangles = 0;
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
