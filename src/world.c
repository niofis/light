#include <stdlib.h>
#include "world.h"
#include "material.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "triangle.h"

world_t*
world_cornell()
{
	world_t* world;
	
	world = (world_t*) malloc(sizeof(world_t));

	world->camera = camera_new();

	world->num_materials = 7;
	world->materials = material_new(world->num_materials);
	color_set_argb(&(world->materials[0].color), 1.0f, 1.0f, 0.0f, 0.0f); //Red
	color_set_argb(&(world->materials[1].color), 1.0f, 0.0f, 1.0f, 0.0f); //Green
	color_set_argb(&(world->materials[2].color), 1.0f, 0.0f, 0.0f, 1.0f); //Blue
	color_set_argb(&(world->materials[3].color), 1.0f, 1.0f, 1.0f, 1.0f); //White
	color_set_argb(&(world->materials[4].color), 1.0f, 1.0f, 1.0f, 0.0f); //Yellow
	color_set_argb(&(world->materials[5].color), 1.0f, 0.0f, 0.0f, 0.0f); //Black
	color_set_argb(&(world->materials[6].color), 1.0f, 1.0f, 0.5f, 0.0f); //Orange 
	
	world->num_triangles = 10;
	world->triangles = triangle_new(world->num_triangles);

	//left wall

	v3_set_xyz(&world->triangles[0].pt1, -8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[0].pt2, -8.0f, 9.0f, 0.0f);
	v3_set_xyz(&world->triangles[0].pt3, -8.0f, 9.0f, 5.0f);
	world->triangles[0].material = &(world->materials[0]);

	triangle_update(&world->triangles[0]);

	v3_set_xyz(&world->triangles[1].pt1, -8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[1].pt2, -8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[1].pt3, -8.0f, 0.0f, 5.0f);
	world->triangles[1].material = &(world->materials[0]);

	triangle_update(&world->triangles[1]);

	//right wall

	v3_set_xyz(&world->triangles[2].pt1, 8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[2].pt2, 8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[2].pt3, 8.0f, 9.0f, 0.0f);
	world->triangles[2].material = &(world->materials[1]);

	triangle_update(&world->triangles[2]);

	v3_set_xyz(&world->triangles[3].pt1, 8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[3].pt2, 8.0f, 0.0f, 5.0f);
	v3_set_xyz(&world->triangles[3].pt3, 8.0f, 9.0f, 5.0f);
	world->triangles[3].material = &(world->materials[1]);

	triangle_update(&world->triangles[3]);

	//back wall

	v3_set_xyz(&world->triangles[4].pt1, -8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[4].pt2, 8.0f, 0.0f, 5.0f);
	v3_set_xyz(&world->triangles[4].pt3, -8.0f, 0.0f, 5.0f);
	world->triangles[4].material = &(world->materials[3]);

	triangle_update(&world->triangles[4]);

	v3_set_xyz(&world->triangles[5].pt1, -8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[5].pt2, 8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[5].pt3, 8.0f, 0.0f, 5.0f);
	world->triangles[5].material = &(world->materials[3]);

	triangle_update(&world->triangles[5]);

	//ceiling

	v3_set_xyz(&world->triangles[6].pt1, -8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[6].pt2, -8.0f, 9.0f, 0.0f);
	v3_set_xyz(&world->triangles[6].pt3, 8.0f, 9.0f, 0.0f);
	world->triangles[6].material = &(world->materials[3]);

	triangle_update(&world->triangles[6]);

	v3_set_xyz(&world->triangles[7].pt1, 8.0f, 9.0f, 0.0f);
	v3_set_xyz(&world->triangles[7].pt2, 8.0f, 9.0f, 5.0f);
	v3_set_xyz(&world->triangles[7].pt3, -8.0f, 9.0f, 5.0f);
	world->triangles[7].material = &(world->materials[3]);

	triangle_update(&world->triangles[7]);

	//floor

	v3_set_xyz(&world->triangles[8].pt1, -8.0f, 0.0f, 5.0f);
	v3_set_xyz(&world->triangles[8].pt2, 8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[8].pt3, -8.0f, 0.0f, 0.0f);
	world->triangles[8].material = &(world->materials[3]);

	triangle_update(&world->triangles[8]);

	v3_set_xyz(&world->triangles[9].pt1, 8.0f, 0.0f, 0.0f);
	v3_set_xyz(&world->triangles[9].pt2, -8.0f, 0.0f, 5.0f);
	v3_set_xyz(&world->triangles[9].pt3, 8.0f, 0.0f, 5.0f);
	world->triangles[9].material = &(world->materials[3]);

	triangle_update(&world->triangles[9]);
	

	//Lights
	world->num_point_lights = 1; //2;
	world->point_lights = point_light_new(world->num_point_lights);
	v3_set_xyz(&(world->point_lights[0].position), 0.0f, 8.0f, 0.0f);



	//Camera
	world->camera->left_bottom.z = 0.0f;
	world->camera->left_top.z = 0.0f;
	world->camera->right_top.z = 0.0f;
	world->camera->eye.z = -5.0f;

	return world;
}

world_t*
world_new()
{
	world_t *scn;

	scn = (world_t*) malloc(sizeof(world_t));

	scn->camera = camera_new();

	scn->num_materials =  7;
	scn->materials = material_new(scn->num_materials);
	color_set_argb(&(scn->materials[0].color), 1.0f, 1.0f, 0.0f, 0.0f); //Red
	color_set_argb(&(scn->materials[1].color), 1.0f, 0.0f, 1.0f, 0.0f); //Green
	color_set_argb(&(scn->materials[2].color), 1.0f, 0.0f, 0.0f, 1.0f); //Blue
	color_set_argb(&(scn->materials[3].color), 1.0f, 1.0f, 1.0f, 1.0f); //White
	color_set_argb(&(scn->materials[4].color), 1.0f, 1.0f, 1.0f, 0.0f); //Yellow
	color_set_argb(&(scn->materials[5].color), 1.0f, 0.0f, 0.0f, 0.0f); //Black
	color_set_argb(&(scn->materials[6].color), 1.0f, 1.0f, 0.5f, 0.0f); //Orange 

	
	scn->num_triangles = 1;
	scn->triangles = triangle_new(scn->num_triangles);
	v3_set_xyz(&scn->triangles[0].pt1, 4.0f, 0.0f, 0.10f);
	v3_set_xyz(&scn->triangles[0].pt2, 5.0f, 2.0f, 0.10f);
	v3_set_xyz(&scn->triangles[0].pt3, 6.0f, 0.0f, 0.10f);
	scn->triangles[0].material = &(scn->materials[1]);

	triangle_update(&scn->triangles[0]);


	scn->num_point_lights = 1; //2;
	scn->point_lights = point_light_new(scn->num_point_lights);
	
	//scn->point_lights[1].position.x = -5.0f;


	//scn->triangles = (struct triangle*) malloc(sizeof(struct triangle) * triangles_count);

	return scn;
}

void
world_destroy(world_t **world)
{
  world_t *scn = *world;

  if (scn->camera) {
    camera_destroy(&scn->camera);
    scn->camera = 0;
  }

  if(scn->triangles) {
    triangle_destroy(&scn->triangles);
    scn->triangles = 0;
    scn->num_triangles = 0;
  }

  if (scn->point_lights) {
    point_light_destroy(&scn->point_lights);
    scn->point_lights = 0;
    scn->num_point_lights = 0;
  }

  if(scn->materials) {
    material_destroy(&scn->materials);
    scn->materials = 0;
  }

  *world = NULL;
}
