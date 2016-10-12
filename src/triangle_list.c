#include "includes.h"

triangle_list_t*
triangle_list_new()
{
  triangle_list_t *list = (triangle_list_t*) malloc(sizeof(triangle_list_t));
  return list;
}

void
triangle_list_destroy(triangle_list_t **list)
{
  tr_node_t *node = (*list)->head;
  while(node) {
    triangle_destroy(&node->triangle);
    node = node->next;
  }
  (*list)->head = NULL;
  (*list)->tail = NULL;
  free(*list);
  *list = NULL;
}

const tr_node_t*
triangle_list_head(const triangle_list_t *list)
{
  return list->head;
}

const tr_node_t*
triangle_list_next(const tr_node_t *node)
{
  return node->next;
}

const tr_node_t*
triangle_list_tail(const triangle_list_t *list)
{
  return list->tail;
}

void
triangle_list_append(triangle_list_t *list, triangle_t *triangle)
{
  tr_node_t *nnode = (tr_node_t*) malloc(sizeof(tr_node_t));
  nnode->triangle = triangle;
  nnode->next = NULL;

  list->tail->next = nnode;
  list->tail = nnode;
}
