#ifndef VECTOR3_H
#define VECTOR3_H

struct vector3
{
	float x;
	float y;
	float z;
};

struct vector3* v3_new();
void v3_del(struct vector3* v1);
void v3_init(struct vector3*, float x, float y, float z);
void v3_copy(struct vector3* dest, struct vector3* src);
void v3_add(struct vector3* res, struct vector3* v1, struct vector3* v2);
void v3_sub(struct vector3* res, struct vector3* v1, struct vector3* v2);
float v3_dot(struct vector3* v1, struct vector3* v2);
void v3_cross(struct vector3* res, struct vector3* v1, struct vector3* v2);
float v3_norm(struct vector3* v1);
void v3_normalize(struct vector3* v1);
void v3_mul_scalar(struct vector3* v1, float f);
void v3_div_scalar(struct vector3* v1, float f);

#endif
