#ifndef PRIMITIVES_H
#define PRIMITIVES_H

#include "vector3.h"

typedef struct color
{
	float a;
	float r;
	float g;
	float b;
};

typedef struct ray
{
	struct vector3 direction;
	struct vector3 origin;
};

struct ray* ray_new();
void ray_delete();

#ifdef DEBUG
void ray_dump(struct ray* ray);
#endif

typedef struct camera
{
	struct vector3 left_top;
	struct vector3 left_bottom;
	struct vector3 right_top;
	struct vector3 eye;
};

typedef struct triangle
{
	struct vector3 pt1;
	struct vector3 pt2;
	struct vector3 pt3;
};

typedef struct sphere
{
	struct vector3 center;
	float radius;
};

typedef struct scene
{
	struct camera *camera;
	struct triangle *triangles;
	struct sphere *spheres;
	int num_triangles;
	int num_spheres;
};

struct scene* scene_new();
void scene_del(struct scene*);

struct camera* camera_new();
void camera_del(struct camera*);

struct sphere* sphere_new(int count);
void sphere_del(struct sphere* spheres);
//just for test purposes, intersection should return true or false
int sphere_intersects(struct sphere* sphere, struct ray* ray);
#endif