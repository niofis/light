#ifndef PRIMITIVE_H
#define PRIMITIVE_H

#define SPHERE
#define TRIANGLE

struct primitive
{

	int type;
};

int prm_intersect(struct primitive* prm, struct ray* ray, struct intersection* result);

#endif
