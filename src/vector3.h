#pragma once

typedef struct 
{
	float x;
	float y;
	float z;
} v3_t;

v3_t*
  v3_new();

v3_t*
  v3_new_xyz(float x, float y, float z);

void
  v3_destroy(v3_t **v1);

void
  v3_set_xyz(v3_t *dest, float x, float y, float z);

void
  v3_copy(v3_t *dest, const v3_t *src);

void
  v3_add(v3_t *dest, const v3_t *v1, const v3_t *v2);

void
  v3_sub(v3_t *dest, const v3_t *v1, const v3_t *v2);

float
  v3_dot(const v3_t *v1, const v3_t *v2);

void
  v3_cross(v3_t *dest, const v3_t *v1, const v3_t *v2);

float
  v3_norm(const v3_t *v1);

void
  v3_normalize(v3_t *v1);

void
  v3_mul_scalar(v3_t *dest, const v3_t *v1, float f);

void
  v3_div_scalar(v3_t *dest, const v3_t *v1, float f);
