#include <stdlib.h>
#include "triangle_list.h"

triangle_list*
triangle_list_new(int count)
{
  triangle_list *list = (triangle_list*) malloc(sizeof(triangle_list));
  return list;
}

void
triangle_list_destroy(triangle_list **list)
{
  tr_node *node = *list->head;
  while(node) {
    triangle_destroy(*node->triangle);
    node = node->next;
  }
  *list->head = NULL;
  *list->tail = NULL;
  free(*list);
  *list = NULL;
}

const tr_node*
triangle_list_iterator(triangle_list *list)
{

}
