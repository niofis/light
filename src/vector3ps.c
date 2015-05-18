#include "vector3ps.h"
#include "memory.h"


struct v3ps* vector3ps_new(int count)
{
  struct v3ps* vec;

  vec = (struct v3ps*) aligned_malloc(16, sizeof(struct v3ps));

  vec->x = (float *) aligned_malloc(16, sizeof(float) * count);
  vec->y = (float *) aligned_malloc(16, sizeof(float) * count);
  vec->z = (float *) alogned_malloc(16, sizeof(float) * count);

  return vec;
}

void v3ps_del(struct v3ps* vec)
{
  aligned_free(vec->x);
  aligned_free(vec->y);
  aligned_free(vec->z);
  aligned_free(vec);
}

void v3ps_add(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2)
{
  int count = v1->count;
  
  for(int i = 0; i < count; ++i)
  {
    dest->x[i] = v1->x[i] + v2->x[i];
    dest->y[i] = v1->y[i] + v2->y[i];
    dest->z[i] = v1->z[i] + v2->z[i];
  }
}

void v3ps_sub(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2)
{
  int count = v1->count;

  for(int i = 0; i < count; ++i)
  {
    dest->x[i] = v1->x[i] - v2->x[i];
    dest->y[i] = v1->y[i] - v2->y[i];
    dest->z[i] = v1->z[i] - v2->z[i];
  }
}

void v3ps_norm(float* n, struct v3ps* v1)
{
  int count = v1->count;

  for(int i = 0; i < count; ++i)
  {
    n[i] = sqrt(v1->x[i] * v1->x[i] +
                v1->y[i] * v1->y[i] +
                v1->z[i] * v1->z[i]);
  }
}

void v3ps_normalize(struct v3ps* dest, struct v3ps* v1)
{
  int count = v1->count;
  float* n = (float *) aligned_malloc(16, sizeof(float) * count);

  v3ps_norm(n, v1);

  for(int i = 0; i < count; ++i)
  {
    dest->x[i] = v1->x[i] / n[i];
    dest->y[i] = v1->y[i] / n[i];
    dest->z[i] = v1->z[i] / n[i];
  }
    
}

void v3ps_dot(float* d, struct v3ps* v1)
{
  int count = v1->count;

  for(int i = 0; i < count; ++i)
  {
    d[i] =  v1->x[i] * v1->x[i] +
            v1->y[i] * v1->y[i] +
            v1->z[i] * v1->z[i];
  }
}

void v3ps_cross(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2)
{
  int count = v1->count;

  for(int i = 0; i < count; ++i)
  {
    
  }
}
