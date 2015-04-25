#include "triangle_list.h"

struct triangle_list* triangle_list_new(int count)
{
  struct triangle_list* list;

  list = (struct triangle_list*) malloc(sizeof(struct triangle_list)); 

//have to init each array to the count size e¡specified

  return list;
}

void triangle_list_del(struct triangle_list* list)
{

}


