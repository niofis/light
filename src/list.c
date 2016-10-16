#include "includes.h"

list_t*
list_new()
{
  list_t *list = (list_t*) malloc(sizeof(list_t));
  list->head = NULL;
  list->tail = NULL;
  list->length = 0;
  return list;
}

void
list_destroy(list_t **list)
{
  list_t *l = *list;
  node_t *node = l->head;
  node_t *tmp;
  while(node) {
    tmp = node->next;
    free(node);
    node = tmp;
  }

  l->head = NULL;
  l->tail = NULL;
  free(l);
  l = NULL;
}

node_t*
list_head(const list_t *list)
{
  return list->head;
}

node_t*
list_next(const node_t *node)
{
  return node->next;
}

node_t*
list_tail(const list_t *list)
{
  return list->tail;
}

void
list_append(list_t *list, void *item)
{
  node_t *nnode = (node_t*) malloc(sizeof(node_t));
  nnode->item = item;
  nnode->next = NULL;

  if(list->tail != NULL)
    list->tail->next = nnode;

  if(list->head == NULL)
    list->head = nnode;
  
  list->tail = nnode;
  list->length += 1;
}
