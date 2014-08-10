#include <stdlib.h>
#include <math.h>
#include "vector3.h"

#ifdef DEBUG
#include "debug.h"
#endif

struct vector3* v3_new()
{
	struct vector3* v3 = (struct vector3*) malloc(sizeof(struct vector3));
	return v3;
}

void v3_del(struct vector3* v1)
{
	if (v1 != 0)
	{
		free(v1);
	}
}

void v3_init(struct vector3* v1, float x, float y, float z)
{
	v1->z = z;
	v1->y = y;
	v1->x = x;
}

//I like this dest, src arrangement because it's a bit like
//Intel's assembler: MOV eax,1
void v3_copy(struct vector3* dest, struct vector3* src)
{
	dest->x = src->x;
	dest->y = src->y;
	dest->z = src->z;
}

void v3_add(struct vector3* res, struct vector3* v1, struct vector3* v2)
{
	res->x = v1->x + v2->x;
	res->y = v1->y + v2->y;
	res->z = v1->z + v2->z;
}

void v3_sub(struct vector3* res, struct vector3* v1, struct vector3* v2)
{
	res->x = v1->x - v2->x;
	res->y = v1->y - v2->y;
	res->z = v1->z - v2->z;
}

float v3_dot(struct vector3* v1, struct vector3* v2)
{
	return	v1->x*v2->x + 
			v1->y*v2->y + 
			v1->z*v2->z;
}

void v3_cross(struct vector3* res, struct vector3* v1, struct vector3* v2)
{
	res->x = v1->y * v2->z - v1->z*v2->y;
	res->y = v1->z * v2->x - v1->x*v2->z;
	res->z = v1->x * v2->y - v1->y*v2->x;
}

float v3_norm(struct vector3* v1)
{
	return sqrtf(
			v1->x * v1->x +
			v1->y * v1->y +
			v1->z * v1->z
		);
}

void v3_normalize(struct vector3* v1)
{
	float norm = v3_norm(v1);
	v1->x /= norm;
	v1->y /= norm;
	v1->z /= norm;
}

void v3_mul_scalar(struct vector3* dest, struct vector3* v1, float f)
{
	dest->x = v1->x * f;
	dest->y = v1->y * f;
	dest->z = v1->z * f;
}

void v3_div_scalar(struct vector3* dest, struct vector3* v1, float f)
{
	dest->x = v1->x / f;
	dest->y = v1->y / f;
	dest->z = v1->z / f;
}

#ifdef DEBUG
void v3_dump(struct vector3* v3)
{
	printf("{x:%f, y:%f, z: %f}\n",
		v3->x,
		v3->y,
		v3->z);
}
#endif
