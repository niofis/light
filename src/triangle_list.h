#pragma once

#include "triangle.h"

typedef struct {
  triangle *triangle;
  tr_node *next;
} tr_node;

typedef struct {
  size_t length;
  tr_node *head;
  tr_node *tail;
} triangle_list;

triangle_list*
  triangle_list_new(size_t count);

void
  triangle_list_destroy(triangle_list **list);

const tr_node*
  triangle_list_iterator(triangle_list *list);

