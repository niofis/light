#include "includes.h"

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
  v3_set_xyz(&triangle->v0, -8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, -8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->v2, -8.0f, 9.0f, 5.0f);
  triangle->material = red;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v2, -8.0f, 0.0f, 5.0f);
  triangle->material = red;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //right wall

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v2, 8.0f, 9.0f, 0.0f);
  triangle->material = green;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->v2, 8.0f, 9.0f, 5.0f);
  triangle->material = green;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //back wall

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->v2, -8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v2, 8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //ceiling

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v1, -8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->v2, 8.0f, 9.0f, 0.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, 8.0f, 9.0f, 0.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 9.0f, 5.0f);
  v3_set_xyz(&triangle->v2, -8.0f, 9.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  //floor

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->v1, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v2, -8.0f, 0.0f, 0.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  triangle = triangle_new();
  v3_set_xyz(&triangle->v0, 8.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, -8.0f, 0.0f, 5.0f);
  v3_set_xyz(&triangle->v2, 8.0f, 0.0f, 5.0f);
  triangle->material = white;
  triangle_update(triangle);
  list_append(world->triangles, triangle);	

  //Lights
  
  world->lights = list_new();
  point_light_t *light = point_light_new();
  v3_set_xyz(&light->position, 0.0f, 6.0f, 0.0f);
  list_append(world->lights, light);

  //printf("Number of triangles: %zu\n", world->triangles->length);

  world->bvh = bvh_new(world->triangles);

  return world;
}

world_t*
world_from_model(const char *file)
{
  world_t *world;
  triangle_t *triangle;

  world = (world_t*) malloc(sizeof(world_t));

  world->camera = camera_new();
  world->camera->eye.y += 4.0f;
  world->camera->eye.z = 75.0f;
  world->camera->left_top.y += 4.0f;
  world->camera->left_top.z = 50.0f;
  world->camera->right_top.y += 4.0f;
  world->camera->right_top.z = 50.0f;
  world->camera->left_bottom.y += 4.0f;
  world->camera->left_bottom.z = 50.0f;

  world->materials = list_new();

  material_t *white = material_new();
  color_set_argb(&white->color, 1.0f, 1.0f, 1.0f, 1.0f);
  list_append(world->materials, white);

  world->lights = list_new();
  point_light_t *light = point_light_new();
  v3_set_xyz(&light->position, 0.0f, 0.0f, 100.0f);
  list_append(world->lights, light);

  world->triangles = list_new();


  const struct aiScene *scene = aiImportFile(file, 0);

  if(scene) {
    /*
    printf("has meshes: %i\n", scene->mNumMeshes);
    struct aiMesh *mesh = scene->mMeshes[0];
    printf("has faces: %i\n", mesh->mNumFaces);
    struct aiFace face = mesh->mFaces[0];
    printf("has indices: %i, %i, %i\n", face.mIndices[0], face.mIndices[1], face.mIndices[2]);
    struct aiVector3D vertex = mesh->mVertices[face.mIndices[0]];
    printf("first vertex: (%f, %f, %f)\n", vertex.x, vertex.y, vertex.z);
    */
    for(int mesh_idx = 0; mesh_idx < scene->mNumMeshes; mesh_idx++) {
      struct aiMesh *mesh = scene->mMeshes[mesh_idx];
      for(int face_idx = 0; face_idx < mesh->mNumFaces; face_idx++) {
        struct aiFace face = mesh->mFaces[face_idx];
        struct aiVector3D v1 = mesh->mVertices[face.mIndices[0]];
        struct aiVector3D v2 = mesh->mVertices[face.mIndices[1]];
        struct aiVector3D v3 = mesh->mVertices[face.mIndices[2]];
        triangle = triangle_new();
        v3_set_xyz(&triangle->v0, v1.x, v1.y, v1.z);
        v3_set_xyz(&triangle->v1, v2.x, v2.y, v2.z);
        v3_set_xyz(&triangle->v2, v3.x, v3.y, v3.z);
        triangle->material = white;
        triangle_scale_uni(triangle, 100.0f);
        triangle_update(triangle);
        list_append(world->triangles, triangle);
      }
    }

    printf("Total triangles: %zu\n", world->triangles->length);
    aiReleaseImport(scene);
  }
  else
    printf("Error loading model!\n");

  world->bvh = bvh_new(world->triangles);

  return world;
}

world_t*
world_new()
{
  world_t *world;

  world = (world_t*) malloc(sizeof(world_t));

  world->camera = camera_new();

  world->materials = list_new();

  material_t *red = material_new();
  color_set_argb(&red->color, 1.0f, 1.0f, 0.0f, 0.0f);
  list_append(world->materials, red);

  world->triangles = list_new();

  triangle_t *triangle = triangle_new();
  v3_set_xyz(&triangle->v0, -6.0f, 0.0f, 0.0f);
  v3_set_xyz(&triangle->v1, 0.0f, 8.0f, 0.0f);
  v3_set_xyz(&triangle->v2, 6.0f, 0.0f, 0.0f);
  triangle->material = red;
  triangle_update(triangle);
  list_append(world->triangles, triangle);

  world->lights = list_new();
  point_light_t *light = point_light_new();
  v3_set_xyz(&light->position, 0.0f, 8.0f, -10.0f);
  list_append(world->lights, light);

  world->bvh = bvh_new(world->triangles);

  return world;
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
    list_destroy(&(scn->triangles));
  }

  if (scn->lights) {
    node = list_head(scn->lights);
    while(node) {
      point_light_destroy((point_light_t**)&node->item);
      node = list_next(node);
    }
    list_destroy(&(scn->lights));
  }

  if(scn->materials) {
    node = list_head(scn->materials);
    while(node) {
      material_destroy((material_t**)&node->item);
      node = list_next(node);
    }
    list_destroy(&(scn->materials));
  }

  if(scn->bvh) {
    bvh_destroy(&(scn->bvh));
  }

  free(*world);
  *world = NULL;
}
