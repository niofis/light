#pragma once
//Vector3 packed single precision
#ifndef VECTOR3PS
#define VECTOR3PS
#include "arrow.h"

struct v3ps
{
  float* x;
  float* y;
  float* z;
  int count;
};


struct v3ps* vector3ps_new(int count);
void v3ps_del(struct v3ps* vec);
void v3ps_add(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2);
void v3ps_sub(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2);
void v3ps_norm(float* n, struct v3ps* v1);
void v3ps_normalize(struct v3ps* v1);
void v3ps_dot(float* d, struct v3ps* v1);
void v3ps_cross(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2);

#endif
