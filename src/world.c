#include <stdlib.h>
#include <stdio.h>
#include <assimp/cimport.h>
#include <assimp/scene.h>
#include "world.h"
#include "material.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"
#include "triangle.h"
#include "triangle_list.h"

world_t*
world_cornell()
{
  world_t *world;
  triangle_t *triangle;

  world = (world_t*) malloc(sizeof(world_t));

  world->camera = camera_new();

  //Camera
  world->camera->left_bottom.z = 0.0f;
  world->camera->left_top.z = 0.0f;
  world->camera->right_top.z = 0.0f;
  world->camera->eye.z = -5.0f;


  //Materials

  world->materials = list_new();

  material_t *red = material_new();
  color_set_argb(&red->color, 1.0f, 1.0f, 0.0f, 0.0f);
  list_append(world->materials, red);

  material_t *green = material_new();
  color_set_argb(&green->color, 1.0f, 0.0f, 1.0f, 0.0f);
  list_append(world->materials, green);

  material_t *white = material_new();
  color_set_argb(&white->color, 1.0f, 1.0f, 1.0f, 1.0f);
  list_append(world->materials, white);


  //Triangles
  world->triangles = list_new();

  //left wall

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, -8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->pt3, -8.0f, 9.0f, 5.0f);
  triangle->material = red;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, -8.0f, 0.0f, 5.0f);
  triangle->material = red;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //right wall

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, 8.0f, 9.0f, 0.0f);
  triangle->material = green;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, 8.0f, 9.0f, 5.0f);
  triangle->material = green;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //back wall

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, -8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, 8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //ceiling

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt2, -8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->pt3, 8.0f, 9.0f, 0.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, 8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, -8.0f, 9.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //floor

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, -8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->pt2, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt3, -8.0f, 0.0f, 0.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->pt2, -8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->pt3, 8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);	

  //Lights
  
  world->lights = list_new();
  point_light_t *light = point_light_new();
  v3_set_xyz(&light->position, 0.0f, 6.0f, 0.0f);
  list_append(world->lights, light);

  const struct aiScene *scene = aiImportFile("../models/bunny.ply",0);

  if(scene) {
    aiReleaseImport(scene);
  }
  else
    printf("Error loading model!");

  return world;
}

world_t*
world_new()
{
  world_t *scn;

  scn = (world_t*) malloc(sizeof(world_t));

  scn->camera = camera_new();

  scn->materials = list_new();

  material_t *green = material_new();
  color_set_argb(&green->color, 1.0f, 0.0f, 1.0f, 0.0f);
  list_append(scn->materials, green);

  scn->triangles = list_new();

  triangle_t *triangle = triangle_new();
  v3_set_xyz(&triangle->pt1, 4.0f, 0.0f, 0.10f);
  v3_set_xyz(&triangle->pt2, 5.0f, 2.0f, 0.10f);
  v3_set_xyz(&triangle->pt3, 6.0f, 0.0f, 0.10f);
  triangle->material = green;
  triangle_update(triangle);
  list_append(scn->triangles, triangle);


  scn->lights = list_new();
  point_light_t *light = point_light_new();
  v3_set_xyz(&light->position, 0.0f, 8.0f, 0.0f);
  list_append(scn->lights, light);

  return scn;
}

void
world_destroy(world_t **world)
{
  world_t *scn = *world;

  node_t *node;

  if (scn->camera) {
    camera_destroy(&scn->camera);
    scn->camera = NULL;
  }

  if(scn->triangles) {
    node = list_head(scn->triangles);
    while(node) {
      triangle_destroy((triangle_t**)&node->item);
      node = list_next(node);
    }
    scn->triangles = NULL;
  }

  if (scn->lights) {
    node = list_head(scn->lights);
    while(node) {
      point_light_destroy((point_light_t**)&node->item);
      node = list_next(node);
    }
    scn->lights = NULL;
  }

  if(scn->materials) {
    node = list_head(scn->materials);
    while(node) {
      material_destroy((material_t**)&node->item);
      node = list_next(node);
    }
    scn->materials = NULL;
  }

  *world = NULL;
}
