#include "includes.h"

sphere_t*
sphere_new()
{
	sphere_t* spheres;
	spheres = (sphere_t*) malloc(sizeof(sphere_t));
	
	return spheres;
}

void
sphere_destroy(sphere_t **sphere)
{
	if (*sphere) {
		free(*sphere);
        *sphere = NULL;
	}
}

//from the book "Ray Tracing from the ground up"
int sphere_intersects(sphere_t *sphere, ray_t *ray, intersection_t *result)
{
	float t;
	v3_t temp;
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

	if (disc < 0.0) {
		return 0;
	}
	else {
		e = sqrtf(disc);
		denom = 2.0f * a;
		t = (b - e) / denom; //smaller root

		if (t > 0.007f) {
			//tmin = t;
			result->hit = 1;
			result->distance = t;
/*
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
*/
			return 1;
		}

		t = (b + e) / denom; //larger root
		if (t > 0.007f) {
			//tmin = t;
			result->hit = 1;
			result->distance = t;
/*
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
*/
			return 1;
		}

		return 0;
	}
}
