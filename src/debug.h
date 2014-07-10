#ifdef DEBUG
#ifndef DEBUG_H
#include <stdio.h>
#include "vector3.h"
#include "primitives.h"

void debug_nl();
void debug_pause();
void v3_dump(struct vector3* v3);
void ray_dump(struct ray* ray);
#endif
#endif
