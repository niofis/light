#include <stdio.h>
#include <stdlib.h>
#include "vector3.h"
#include "job.h"
#include "render.h"
#include "ray.h"
#include "scene.h"
#include "camera.h"
#include "sphere.h"

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

void getray(struct ray* ray, int x, int y, struct job_desc* job)
{
	struct vector3 u;
	struct vector3 v;

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

}

void shading(struct trace_data* data, struct scene* scene)
{
	float s = v3_dot(&data->normal, &data->ray.direction);
	if (s < 0.0f)
	{
		s *= -1.0f;
	}
	color_mul_scalar(&data->color,s);
}

//returns color
void traceray(struct trace_data* data, struct scene* scene)
{
	struct sphere* spheres;
	struct sphere* sphere;
	
	data->color.a = 1.0f;
	data->color.r = 1.0f;
	data->color.g = 1.0f;
	data->color.b = 0.0f;

	spheres = scene->spheres;
	for (int i = 0; i < scene->num_spheres; ++i)
	{
		sphere = &(spheres[i]);
		sphere_intersects(sphere, &data->ray, data);
		if (data->hit)
		{
			shading(data, scene);
		}
		else
		{
			color_mul_scalar(&data->color,0.0f);
		}
	}
	

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
			struct trace_data data;
			getray(&data.ray, x, y, job);
			traceray(&data, job->scene);
			//ARGB
			buffer[p] = color_to_argb(&data.color);
		}
	}
	return 0;
}
