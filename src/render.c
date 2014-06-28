#include <stdio.h>
#include <stdlib.h>
#include "vector3.h"
#include "primitives.h"
#include "job.h"
#include "render.h"

struct vector3 vdu;
struct vector3 vdv;

void init_delta_vectors(struct job_desc* job)
{
	v3_sub(&vdu,
		&job->scene->camera->right_top,
		&job->scene->camera->left_top);
	v3_sub(&vdv,
		&job->scene->camera->left_bottom,
		&job->scene->camera->left_top);

	v3_div_scalar(&vdu, (float) job->width);
	v3_div_scalar(&vdv, (float) job->height);

}

struct ray* getray(int x, int y, struct job_desc* job)
{
	struct ray* ray;
	struct vector3 u;
	struct vector3 v;

	ray = ray_new();
	v3_copy(&ray->origin, &job->scene->camera->eye);
	
	v3_copy(&u, &vdu);
	v3_copy(&v, &vdv);

	v3_mul_scalar(&u, (float) x);
	v3_mul_scalar(&v, (float) y);
	
	v3_copy(&ray->direction, &job->scene->camera->left_top);
	
	v3_add(&ray->direction, &ray->direction, &u);
	v3_add(&ray->direction, &ray->direction, &v);

	v3_sub(&ray->direction, &ray->direction, &job->scene->camera->eye);

	v3_normalize(&ray->direction);

	return ray;
}

//returns color
struct result traceray(struct ray* ray, struct scene* scene)
{
	struct result result;
	struct sphere* spheres;
	struct sphere* sphere;
	
	result.color.a = 1.0f;
	result.color.r = 0.0f;
	result.color.g = 0.0f;
	result.color.b = 0.0f;

	spheres = scene->spheres;
	for (int i = 0; i < scene->num_spheres; ++i)
	{
		sphere = &spheres[i];
		if (sphere_intersects(sphere, ray))
		{
			result.color.g = 1.0f;
		}
	}
	

	return result;
}


int color_to_argb(struct color color)
{
	int argb = 0;
	argb = (int) ((color.a<1.0f ? color.a : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color.r<1.0f ? color.r : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color.g<1.0f ? color.g : 1.0f) * 255);
	argb = argb << 8;
	argb |= (int) ((color.b<1.0f ? color.b : 1.0f) * 255);

	return argb;
}

int render(struct job_desc* job)
{
	int x = 0;
	int y = 0;
	int width = job->width;
	int height = job->height;
	int *buffer = job->buffer;

	init_delta_vectors(job);

	for (y = 0; y<height; ++y)
	{
		for (x = 0; x<width; ++x)
		{
			int p = y*height + x;
			struct result res;
			struct ray* ray;
			ray = getray(x, y, job);
			res = traceray(ray, job->scene);
			//ARGB
			buffer[p] = color_to_argb(res.color);
		}
	}
	return 0;
}