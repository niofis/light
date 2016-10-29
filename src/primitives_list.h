#pragma once
#include "includes.h"

typedef struct _prmnode {
  primitive_t *primitive;
  struct _prmnode *next;
} prmnode_t;

typedef struct
{
  size_t length;
  prmnode_t *head;
  prmnode_t *tail;
} prmlist_t;

prmlist_t*
  prmlist_new();

void
  prmlist_destroy(prmlist_t **list);

prmnode_t*
  prmlist_head(prmlist_t *list);

void
  prmlist_append(prmlist_t *list, primitive_t *prm);
