#include <stdlib.h>
#include <math.h>
#include "primitives.h"

#ifdef DEBUG
#include "debug.h"

void ray_dump(struct ray* ray)
{
	printf("{\norigin: \n{\n");
	v3_dump(&ray->origin);
	printf("},\ndirection: \n{\n");
	v3_dump(&ray->direction);
	printf("}\n}\n");
}
#endif

struct ray* ray_new()
{
	struct ray* ray;
	ray = (struct ray*) malloc(sizeof(struct ray));
	v3_init(&ray->origin, 0.0f, 0.0f, 0.0f);
	v3_init(&ray->direction, 0.0f, 0.0f, 0.0f);
	return ray;
}

void ray_delete(struct ray* ray)
{
	if (ray)
	{
		free(ray);
	}
}

struct scene* scene_new()
{
	struct scene *scn;

	scn = (struct scene*) malloc(sizeof(struct scene));

	scn->camera = camera_new();
	scn->num_spheres = 1;
	scn->spheres = sphere_new(scn->num_spheres);
	v3_init(&scn->spheres[0].center, 1.0f, 0.0f, 0.0f);
	scn->spheres[0].radius = 1.0f;

	//scn->triangles = (struct triangle*) malloc(sizeof(struct triangle) * triangles_count);

	return scn;
}

void scene_del(struct scene* scn)
{
	if (scn->camera)
	{
		camera_del(scn->camera);
		scn->camera = 0;
	}

	if (scn->spheres)
	{
		sphere_del(scn->spheres);
		scn->spheres = 0;
		scn->num_spheres = 0;
	}
}

struct camera* camera_new()
{
	struct camera* cmr;

	cmr = (struct camera*) malloc(sizeof(struct camera));

	cmr->left_top.x = -8.0f;
	cmr->left_top.y = 9.0f;
	cmr->left_top.z = -50.0f;

	cmr->right_top.x = 8.0f;
	cmr->right_top.y = 9.0f;
	cmr->right_top.z = -50.0f;

	cmr->left_bottom.x = -8.0f;
	cmr->left_bottom.y = 0.0f;
	cmr->left_bottom.z = -50.0f;

	cmr->eye.x = 0.0f;
	cmr->eye.y = 4.5f;
	cmr->eye.z = -75.0f;

	return cmr;
}

void camera_del(struct camera* cmr)
{
	if (cmr) 
	{
		free(cmr);
	}
}

struct sphere* sphere_new(int count)
{
	struct sphere* spheres;
	spheres = (struct sphere*) malloc(count * sizeof(struct sphere));
	return spheres;
}

void sphere_del(struct sphere* spheres)
{
	if (spheres)
	{
		free(spheres);
	}
}

//from the book "Ray Tracing from the ground up"
int sphere_intersects(struct sphere* sphere, struct ray* ray)
{
	/*
	struct vector3 edge;
	float B;
	float B2;
	float C;
	float I;
	float t;
	float t0;

	v3_sub(&edge, &(ray->origin), &(sphere->center));
	B = -2.0f * v3_dot(&edge, &(ray->direction));
	B2 = B * B;
	C = v3_dot(&edge, &edge) - (sphere->radius * sphere->radius);
	I = B2 - 4.0f * C;

	if (I < 0.0f)
	{
		return 0;
	}

	t0 = sqrtf(I);
	t = (B - t0) / 2.0f;

	if (t < 0.01f)
	{
		t = (B + t0) / 2.0f;
	}

	if (t < 0.01f)
	{
		return 0;
	}

	return 1;
	*/
	
	float t;
	struct vector3 temp;
	float a;
	float b;
	float c;
	float disc;
	float e;
	float denom;

	v3_sub(&temp, &ray->origin, &sphere->center);
	a = v3_dot(&ray->direction, &ray->direction);
	b = -2.0f * v3_dot(&temp, &ray->direction);
	c = v3_dot(&temp, &temp) - (sphere->radius * sphere->radius);
	disc = (b * b) - (4.0f * c);


	if (disc < 0.0)
	{
		return 0;
	}
	else
	{
		e = sqrtf(disc);
		denom = 2.0f * a;
		t = (b - e) / denom; //smaller root

		if (t > 0.001f)
		{
			//tmin = t;
			//normal = (temp + t * ray.direction) / radius;
			//local_hit_point = ray.origin + t * ray.direction;
			return 1;
		}

		t = (b + e) / denom; //larger root
		if (t > 0.001f)
		{
			//tmin = t;
			//normal = (temp + t * ray.direction) / radius;
			//local_hit_point = ray.origin + t * ray.direction;
			return 1;
		}

		return 0;
	}
	

}