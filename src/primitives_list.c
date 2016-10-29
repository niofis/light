#include "includes.h"

prmlist_t*
prmlist_new()
{
  rmlist_t *lst = (prmlist_t*) malloc(sizeof(prmlist_t));
  return lst;
}

void
prmlist_destroy(prmlist_t **list)
{
  prmnode_t *node = (*list)->head;
  prmnode_t *tmp;
  while(node)
  {
    tmp = node->next;
    free(node);
    node = tmp;
  }

  (*list)->head = NULL;
  (*list)->tail = NULL;
  free(*list);
  *list = NULL;

}

prmnode_t*
prmlist_head(prmlist_t *list)
{
  return list->head;
}

void
prmlist_append(prmlist_t *list, primitive_t *prm)
{
  prmnode_t *nnode = (prmnode_t*) malloc(sizeof(prmnode_t));
  nnode->primitive = prm;
  nnode->next = NULL;
  
  list->tail->next = nnode;
  list->tail = nnode;

  if (!list->head)
    list->head = nnode;
}
