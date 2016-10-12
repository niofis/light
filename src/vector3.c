#include "includes.h"

v3_t*
v3_new()
{
	v3_t *v3 = (v3_t*) malloc(sizeof(v3_t));
	return v3;
}

v3_t*
v3_new_xyz(float x, float y, float z)
{
  v3_t *v3 = v3_new();
  v3->x = x;
  v3->y = y;
  v3->z = z;
  return v3;
}

void
v3_destroy(v3_t **v1)
{
	if (*v1 != NULL)
	{
		free(*v1);
        *v1 = NULL;
	}
}

void
v3_set_xyz(v3_t* v1, float x, float y, float z)
{
	v1->z = z;
	v1->y = y;
	v1->x = x;
}

//I like this dest, src arrangement because it's a bit like
//Intel's assembler: MOV eax,1
void
v3_copy(v3_t *dest, const v3_t *src)
{
	dest->x = src->x;
	dest->y = src->y;
	dest->z = src->z;
}

void
v3_add(v3_t *res, const v3_t *v1, const v3_t *v2)
{
	res->x = v1->x + v2->x;
	res->y = v1->y + v2->y;
	res->z = v1->z + v2->z;
}

void
v3_sub(v3_t *res, const v3_t *v1, const v3_t *v2)
{
	res->x = v1->x - v2->x;
	res->y = v1->y - v2->y;
	res->z = v1->z - v2->z;
}

float
v3_dot(const v3_t *v1, const v3_t *v2)
{
	return	v1->x*v2->x + 
			v1->y*v2->y + 
			v1->z*v2->z;
}

void
v3_cross(v3_t *res, const v3_t *v1, const v3_t *v2)
{
	float x = v1->y * v2->z - v1->z * v2->y;
	float y = v1->z * v2->x - v1->x * v2->z;
	float z = v1->x * v2->y - v1->y * v2->x;

    res->x = x;
    res->y = y;
    res->z = z;
}

float
v3_norm(const v3_t *v1)
{
	return sqrtf(
			v1->x * v1->x +
			v1->y * v1->y +
			v1->z * v1->z
		);
}

void
v3_normalize(v3_t *v1)
{
	float norm = v3_norm(v1);
	v1->x /= norm;
	v1->y /= norm;
	v1->z /= norm;
}

void
v3_mul_scalar(v3_t *dest, const v3_t *v1, float f)
{
	dest->x = v1->x * f;
	dest->y = v1->y * f;
	dest->z = v1->z * f;
}

void
v3_div_scalar(v3_t *dest, const v3_t *v1, float f)
{
	dest->x = v1->x / f;
	dest->y = v1->y / f;
	dest->z = v1->z / f;
}

#ifdef DEBUG
void
v3_dump(v3_t *v3)
{
	printf("{x:%f, y:%f, z: %f}\n",
		v3->x,
		v3->y,
		v3->z);
}
#endif
