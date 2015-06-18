#include "vector3ps.h"
#include "memory.h"


struct v3ps* vector3ps_new(int count)
{
  struct v3ps* vec;

  vec = (struct v3ps*) aligned_malloc(16, sizeof(struct v3ps));

  vec->x = arr_ps_new(count);
  vec->y = arr_ps_new(count);
  vec->z = arr_ps_new(count);

  return vec;
}

void v3ps_del(struct v3ps* vec)
{
  arr_ps_del(vec->x);
  arr_ps_del(vec->y);
  arr_ps_del(vec->z);
  aligned_free(vec);
}

void v3ps_add(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2)
{
  arr_ps_add(dest->x, v1->x, v2->x);
  arr_ps_add(dest->y, v1->y, v2->y);
  arr_ps_add(dest->z, v1->z, v2->z);
}

void v3ps_sub(struct v3ps* dest, struct v3ps* v1, struct v3ps* v2)
{
  arr_ps_sub(dest->x, v1->x, v2->x);
  arr_ps_sub(dest->y, v1->y, v2->y);
  arr_ps_sub(dest->z, v1->z, v2->z);
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
