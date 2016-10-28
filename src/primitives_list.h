#pragma once
#include "includes.h"

typedef struct
{
	int count;
	primitive_t* first;
	primitive_t* last;
} prmlist_t;

prmlist_t*
  prmlist_new();

//prm_list_del will dispose of individual elements
void
  prmlist_destroy(prmlist_t **list);
