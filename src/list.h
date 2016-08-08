#pragma once

typedef struct _node {
  void *item;
  struct _node *next;
} node_t;

typedef struct {
  size_t length;
  node_t *head;
  node_t *tail;
} list_t;

list_t*
  list_new();

void
  list_destroy(list_t **list);

node_t*
  list_head(const list_t *list);

node_t*
  list_next(const node_t *node);

node_t*
  list_tail(const list_t *list);

void
  list_append(list_t *list, void *item);
