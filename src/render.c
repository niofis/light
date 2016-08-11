#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "material.h"
#include "vector3.h"
#include "job.h"
#include "render.h"
#include "ray.h"
#include "world.h"
#include "camera.h"
#include "triangle.h"
#include "point_light.h"
#include "intersection.h"

//not thread safe :(
v3_t vdu;
v3_t vdv;

void
init_delta_vectors(job_t *job)
{
	v3_sub(&vdu,
		&job->world->camera->right_top,
		&job->world->camera->left_top);
	v3_sub(&vdv,
		&job->world->camera->left_bottom,
		&job->world->camera->left_top);

	v3_div_scalar(&vdu, &vdu, (float) job->width);
	v3_div_scalar(&vdv, &vdv, (float) job->height);

}

void
getray(ray_t *ray, int x, int y, job_t *job)
{
	v3_t u;
	v3_t v;

	v3_copy(&ray->origin, &job->world->camera->eye);
	
	v3_copy(&u, &vdu);
	v3_copy(&v, &vdv);

	v3_mul_scalar(&u, &u, (float) x);
	v3_mul_scalar(&v, &v, (float) y);
	
	v3_copy(&ray->direction, &job->world->camera->left_top);
	
	v3_add(&ray->direction, &ray->direction, &u);
	v3_add(&ray->direction, &ray->direction, &v);

	v3_sub(&ray->direction, &ray->direction, &job->world->camera->eye);

	v3_normalize(&ray->direction);

}

int
find_any(ray_t *ray, world_t *world, float max_distance, intersection_t *result)
{

	triangle_t *tr;
	intersection_t its;

	its.hit = 0;

	if(its.hit == 0) {
        node_t *node = list_head(world->triangles);
        while(node) {
			tr = (triangle_t*) node->item;
			triangle_intersects(tr, ray, &its);
			if(its.hit && its.distance < max_distance && its.distance > 0.001f)
				break;
			its.hit = 0;
            node = list_next(node);
		}
	}

	memcpy(result, &its, sizeof(intersection_t));

	return its.hit;
}

void
shading(world_t *world, intersection_t *trace, color_t *color)
{
	ray_t light_ray;
	point_light_t *point_light;
	intersection_t result;
	color_t light;
	color_t light_temp;
	float light_distance = 0.0f;

	result.hit = 0;
	color_set_argb(&light, 1.0f, 0.0f, 0.0f, 0.0f);

	v3_copy(&light_ray.origin, &trace->hit_point);
    node_t *light_node = list_head(world->lights);
	while(light_node) {
		point_light = (point_light_t*) light_node->item;
		v3_sub(&light_ray.direction, &point_light->position, &light_ray.origin);
		light_distance = v3_norm(&light_ray.direction);
		v3_normalize(&light_ray.direction);
	
		find_any(&light_ray, world, light_distance, &result);

		if (result.hit == 0) {
			float s = v3_dot(&trace->normal, &light_ray.direction);
			if (s < 0.0f)
				s = 0.0f;
			color_mul_scalar(&light_temp, &point_light->color, s);
			color_add(&light, &light, &light_temp);
		}
        light_node = list_next(light_node);
	}
	color_mul(color, &(trace->material->color), &light);
}


int
find_closest(ray_t* ray, world_t* world, float max_distance, intersection_t* result)
{
	triangle_t *triangle;
	intersection_t its;
	intersection_t closest;

	closest.hit = 0;

    node_t * node = list_head(world->triangles);
	while(node) {
		triangle = (triangle_t*) node->item;
		triangle_intersects(triangle, ray, &its);
		if(its.hit && its.distance > 0.001f) {
			if (closest.hit == 0 || its.distance < closest.distance) {
				v3_copy(&its.normal, &triangle->normal);

				v3_mul_scalar(&its.hit_point, &ray->direction, its.distance);
				v3_add(&its.hit_point, &its.hit_point, &ray->origin);

				memcpy(&closest, &its, sizeof(intersection_t));
				closest.material = triangle->material;
			}			
		}
        node = list_next(node);
	}

	memcpy(result, &closest, sizeof(intersection_t));
	
	return closest.hit;
}

void
traceray(ray_t *ray, world_t *world, color_t *color)
{
	intersection_t result;
	float max_distance = 1000.0f;


	find_closest(ray, world, max_distance, &result);
	if (result.hit) {
		//color_init(color, 1.0f, 1.0f, 1.0f, 0.0f);
		shading(world, &result, color);
	}
	else
	{
		color_set_argb(color, 1.0f, 0.0f, 0.0f, 0.0f);
	}
}

int
render(job_t *job)
{
	int x = 0;
	int y = 0;
	int width = job->width;
	int height = job->height;
	int *buffer = job->buffer;

	init_delta_vectors(job);

	for (y = 0; y < height; ++y)
	{
		for (x = 2; x < width; ++x)
		{
			int p = y*width + x;
			ray_t ry;
			color_t color;
			getray(&ry, x, y, job);
			traceray(&ry, job->world, &color);
			//ARGB
			buffer[p] = color_to_argb(&color);
		}
	}
	return 0;
}
