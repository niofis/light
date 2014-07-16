#include <stdlib.h>
#include <math.h>
#include "sphere.h"
#include "ray.h"
#include "render.h"

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
int sphere_intersects(struct sphere* sphere, struct ray* ray, struct trace_data* result)
{
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

	result->hit = 0;

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
			result->hit = 1;
			result->distance = t;

			//local_hit_point = ray.origin + t * ray.direction;
			v3_copy(&result->hit_point, &ray->direction);
			v3_mul_scalar(&result->hit_point, t);
			//for normal
			v3_copy(&result->normal, &result->hit_point);
			//
			v3_add(&result->hit_point, &result->hit_point, &ray->origin);

			//normal = (temp + t * ray.direction) / radius;
			v3_add(&result->normal, &result->normal, &temp);
			v3_div_scalar(&result->normal, sphere->radius);

			return 1;
		}

		t = (b + e) / denom; //larger root
		if (t > 0.001f)
		{
			//tmin = t;
			result->hit = 1;
			result->distance = t;

			//local_hit_point = ray.origin + t * ray.direction;
			v3_copy(&result->hit_point, &ray->direction);
			v3_mul_scalar(&result->hit_point, t);
			//for normal
			v3_copy(&result->normal, &result->hit_point);
			//
			v3_add(&result->hit_point, &result->hit_point, &ray->origin);

			//normal = (temp + t * ray.direction) / radius;
			v3_add(&result->normal, &result->normal, &temp);
			v3_div_scalar(&result->normal, sphere->radius);

			return 1;
		}

		return 0;
	}
}
