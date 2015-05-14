#include "triangle_list.h"

struct triangle_list* triangle_list_new(int count)
{
  struct triangle_list* list;

  list = (struct triangle_list*) malloc(sizeof(struct triangle_list)); 

  list->count = count;

//have to init each array to the count size e¡specified

  list->v1_x = (float*)malloc(sizeof(float) * count);
  list->v1_y = (float*)malloc(sizeof(float) * count);
  list->v1_z = (float*)malloc(sizeof(float) * count);
  list->v2_x = (float*)malloc(sizeof(float) * count);
  list->v2_y = (float*)malloc(sizeof(float) * count);
  list->v2_z = (float*)malloc(sizeof(float) * count);
  list->v3_x = (float*)malloc(sizeof(float) * count);
  list->v3_y = (float*)malloc(sizeof(float) * count);
  list->v3_z = (float*)malloc(sizeof(float) * count);
  list->normal_x = (float*)malloc(sizeof(float) * count);
  list->normal_y = (float*)malloc(sizeof(float) * count);
  list->normal_z = (float*)malloc(sizeof(float) * count);
  list->material_idx = (int*)malloc(sizeof(int) * count);
  list->group_idx = (int*)malloc(sizeof(int) * count);


  return list;
}

void triangle_list_del(struct triangle_list* list)
{
	free(list->v1_x);
	free(list->v1_y);
	free(list->v1_z);
	free(list->v2_x);
	free(list->v2_y);
	free(list->v2_z);
	free(list->v3_x);
	free(list->v3_y);
	free(list->v3_z);
	free(list->normal_x);
	free(list->normal_y);
	free(list->normal_z);
	free(list->material_idx);
	free(list->group_idx);

	free(list);
}


