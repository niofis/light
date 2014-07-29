#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "vector3.h"
#include "job.h"
#include "render.h"
#include "ray.h"
#include "scene.h"
#include "camera.h"
#include "sphere.h"
#include "point_light.h"

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


int find_any(struct ray* ray, struct scene* scene, float max_distance, struct intersection* result)
{

	struct sphere* spheres;
	struct sphere* sphere;
	struct intersection its;

	its.hit = 0;

	spheres = scene->spheres;
	for (int i = 0; i < scene->num_spheres; ++i)
	{
		sphere = &(spheres[i]);
		sphere_intersects(sphere, ray, &its);
		if (its.hit && its.distance < max_distance) {
			break;
		}
	}

	memcpy(result, &its, sizeof(struct intersection));

	return its.hit;
}

void shading(struct scene* scene, struct intersection* trace, struct color* color)
{
	struct ray light_ray;
	struct point_light* point_light;
	struct intersection result;
	struct color light;
	struct color light_temp;
	float light_distance = 0.0f;

	color_init(&light, 1.0f, 0.0f, 0.0f, 0.0f);

	v3_copy(&light_ray.origin, &trace->hit_point);
	for (int i = 0; i < scene->num_point_lights; ++i)
	{
		point_light = &(scene->point_lights[i]);
		v3_sub(&light_ray.direction, &point_light->position, &light_ray.origin);
		light_distance = v3_norm(&light_ray.direction);
		v3_normalize(&light_ray.direction);
	
		find_any(&light_ray, scene, light_distance, &result);

		if (result.hit == 0)
		{
			float s = v3_dot(&trace->normal, &light_ray.direction);
			if (s < 0.0f)
			{
				s *= -1.0f;
			}
			color_mul_scalar(&light_temp, &point_light->color, s);
			color_add(&light, &light, &light_temp);
		}
	}

	color_mul(color, color, &light);
}


int find_closer(struct ray* ray, struct scene* scene, float max_distance, struct intersection* result)
{
	struct sphere* spheres;
	struct sphere* sphere;
	struct intersection its;
	struct intersection closer;

	closer.hit = 0;
	spheres = scene->spheres;
	for (int i = 0; i < scene->num_spheres; ++i)
	{
		sphere = &(spheres[i]);
		sphere_intersects(sphere, ray, &its);
		if (its.hit) {
			if (closer.hit == 0 || its.distance < closer.distance) {
				memcpy(&closer, &its, sizeof(struct intersection));
			}
		}
	}

	memcpy(result, &closer, sizeof(struct intersection));
	
	return closer.hit;
}

//returns color
void traceray(struct ray* ray, struct scene* scene, struct color* color)
{
	struct intersection result;
	float max_distance = 1000.0f;


	find_closer(ray, scene, max_distance, &result);
	if (result.hit) {
		color_init(color, 1.0f, 1.0f, 1.0f, 0.0f);
		shading(scene, &result, color);
	}
	else
	{
		color_init(color, 1.0f, 0.0f, 0.0f, 0.0f);
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
			struct ray ray;
			struct color color;
			getray(&ray, x, y, job);
			traceray(&ray, job->scene, &color);
			//ARGB
			buffer[p] = color_to_argb(&color);
		}
	}
	return 0;
}
