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

void triangle_list_update_all(struct triangle_list* list)
{

  	/*
    v3_sub(&triangle->edge1, &triangle->pt2, &triangle->pt1);
	v3_sub(&triangle->edge2, &triangle->pt3, &triangle->pt1);
	v3_cross(&triangle->normal, &triangle->edge1, &triangle->edge2);
	v3_normalize(&triangle->normal);
    */
    for(int i = 0; i < list->count; ++i)
    {
      float norm;

      //edge1
      list->v2_x[i] -= list->v1_x[i]
      list->v2_y[i] -= list->v1_y[i]
      list->v2_z[i] -= list->v1_z[i]

      //edge2
      list->v3_x[i] -= list->v1_x[i]
      list->v3_y[i] -= list->v1_y[i]
      list->v3_z[i] -= list->v1_z[i]

      //normal
      //cross product
      list->normal_x[i] = list->v2_y[i] * list->v3_y[i] - list->v2_z[i] * list->v3_y[i];
      list->normal_y[i] = list->v2_z[i] * list->v3_x[i] - list->v2_x[i] * list->v3_z[i];
      list->normal_z[i] = list->v2_x[1] * list->v3_y[i] - list->v2_y[i] * list->v3_x[i];
      
      //norm
      norm = sqrt(list->normal_x[i] * list->normal_x[i] +
                  list->normal_y[i] * list->normal_y[i] +
                  list->normal_z[i] * list->normal_z[i]);
      
      list->normal_x[i] /= norm;
      list->normal_y[i] /= norm;
      list->normal_z[i] /= norm;
    }
}
