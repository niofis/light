#pragma once

#include "includes.h"

typedef struct _trnode {
  triangle_t *triangle;
  struct _trnode *next;
} tr_node_t;

typedef struct {
  size_t length;
  tr_node_t *head;
  tr_node_t *tail;
} triangle_list_t;

triangle_list_t*
  triangle_list_new();

void
  triangle_list_destroy(triangle_list_t **list);

const tr_node_t*
  triangle_list_head(const triangle_list_t *list);

const tr_node_t*
  triangle_list_next(const tr_node_t *node);

const tr_node_t*
  triangle_list_tail(const triangle_list_t *list);

void
  triangle_list_append(triangle_list_t *list, triangle_t *triangle);
