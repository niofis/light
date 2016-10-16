#pragma once
#include "includes.h"
#undef malloc
#undef free
#define malloc(a) malloc_debug(__FILE__, __LINE__, a)
#define free(a) free_debug(__FILE__, __LINE__, a)

void
  debug_init();
void
  debug_end();
void
  debug_nl();
void
  debug_pause();
//void v3_dump(struct vector3* v3);
//void ray_dump(struct ray* ray);
