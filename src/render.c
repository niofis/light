#include "includes.h"
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

  intersection_t its;

  its.hit = 0;

  if(its.hit == 0) {
    node_t *node = list_head(world->primitives);
    while(node) {
      //tr = (triangle_t*) node->item;
      //triangle_intersects(tr, ray, &its);
      prm_intersect(node->item, ray, &its);
      if(its.hit && its.distance < max_distance && its.distance > 0.001f)
        break;
      its.hit = 0;
      node = list_next(node);
    }
  }

  //memcpy(result, &its, sizeof(intersection_t));

  result->hit = its.hit;

  return its.hit;
}

int
bvh_find_any_heap(ray_t* ray, bvh_heap_t *heap, size_t idx, float max_distance, intersection_t* result)
{
  result->hit = 0;
  primitive_t *primitive = heap->primitives[idx];
  if(primitive != NULL) {
    //Only leaves have primitives
    intersection_t its;
    its.hit = 0;
    prm_intersect(primitive, ray, &its);
    if(its.hit && its.distance > 0.001f && its.distance < max_distance) {
      result->hit = 1;
      return 1;
    }
  }
  else {
    //Not a leaf
    float dxi = 1.0f / ray->direction.x;
    float dyi = 1.0f / ray->direction.y;
    float dzi = 1.0f / ray->direction.z;
    float min[3] = {heap->min_x[idx], heap->min_y[idx], heap->min_z[idx]};
    float max[3] = {heap->max_x[idx], heap->max_y[idx], heap->max_z[idx]};
    int sign[3] = {dxi<0,dyi<0,dzi<0};
    float *params[2] = {min, max};

    float tmin = (params[sign[0]][0] - ray->origin.x) * dxi;
    float tmax = (params[1 - sign[0]][0] - ray->origin.x) * dxi;
    float tymin = (params[sign[1]][1] - ray->origin.y) * dyi;
    float tymax = (params[1- sign[1]][1] - ray->origin.y) * dyi;
    if(tmin > tymax || tymin > tmax)
      return 0;
    if(tymin > tmin)
      tmin = tymin;
    if(tymax < tmax)
      tmax = tymax;
    float tzmin = (params[sign[2]][2] - ray->origin.z) * dzi;
    float tzmax = (params[1 - sign[2]][2] - ray->origin.z) * dzi;
    if(tmin > tzmax || tzmin > tmax)
      return 0;


    int id_l = (idx) * 2 + 1;
    int id_r = (idx) * 2 + 2;
    if(id_l < heap->length)
      bvh_find_any_heap(ray, heap, id_l, max_distance, result);
    if(id_r < heap->length && result->hit == 0)
      bvh_find_any_heap(ray, heap, id_r, max_distance, result);
  }

  return 0;
}

int
bvh_find_any(ray_t *ray, bvhnode_t *node, float max_distance, intersection_t *result)
{
  result->hit = 0;

  primitive_t *primitive = node->primitive;
  if(primitive != NULL) {
    //Only leaves have primitives
    intersection_t its;
    its.hit = 0;
    prm_intersect(primitive, ray, &its);
    if(its.hit && its.distance > 0.001f && its.distance < max_distance) {
      result->hit = 1;
      return 1;
    }
    return 0;
  }
  else {
    //Not a leaf
    if(!aabb_intersect(&node->bb, ray))
      return 0;

    if(node->left)
      bvh_find_any(ray, node->left, max_distance, result);
    if(node->right && result->hit == 0)
      bvh_find_any(ray, node->right, max_distance, result);
  }

  return 0;
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

  color_set_argb(&light, 1.0f, 0.0f, 0.0f, 0.0f);

  v3_copy(&light_ray.origin, &trace->hit_point);
  node_t *light_node = list_head(world->lights);
  while(light_node) {
    point_light = (point_light_t*) light_node->item;
    v3_sub(&light_ray.direction, &point_light->position, &light_ray.origin);
    light_distance = v3_norm(&light_ray.direction);
    v3_normalize(&light_ray.direction);

    result.hit = 0;
    //find_any(&light_ray, world, light_distance, &result);
    //bvh_find_any(&light_ray, world->bvh->root, light_distance, &result);
    bvh_find_any_heap(&light_ray, world->bvh->heap, 0, light_distance, &result);

    if (result.hit == 0) {
      float s = v3_dot(&trace->normal, &light_ray.direction);
      if (s < 0.0f)
        s = 0.0f;
      color_mul_scalar(&light_temp, &point_light->color, s);
      color_add(&light, &light, &light_temp);
    }
    light_node = list_next(light_node);
  }
  color_mul(color, &(trace->material.color), &light);
}

void
bvh_traverse_heap(ray_t* ray, bvh_heap_t *heap, size_t idx, intersection_t* closest)
{

  primitive_t *primitive = heap->primitives[idx];
  if(primitive != NULL) {
    //Only leaves have primitives
    intersection_t its;
    its.hit = 0;
    prm_intersect(primitive, ray, &its);
    if(its.hit && its.distance > 0.001f && its.distance < closest->distance) {
      v3_mul_scalar(&its.hit_point, &ray->direction, its.distance);
      v3_add(&its.hit_point, &its.hit_point, &ray->origin);

      if(primitive->type == TRIANGLE) {
        triangle_t *triangle = primitive->obj;
        v3_copy(&its.normal, &triangle->normal);
        its.material = triangle->material;
      }
      else if(primitive->type == SPHERE) {
        sphere_t *sphere = primitive->obj;
        v3_sub(&its.normal, &its.hit_point, &sphere->center);
        v3_normalize(&its.normal);
        its.material = sphere->material;
      }
      //memcpy(closest, &its, sizeof(intersection_t));

      v3_copy(&closest->hit_point, &its.hit_point);
      v3_copy(&closest->normal, &its.normal);
      closest->material = its.material;
      closest->distance = its.distance;
      closest->hit = its.hit;

    }
  }
  else {
    //Not a leaf
    float dxi = 1.0f / ray->direction.x;
    float dyi = 1.0f / ray->direction.y;
    float dzi = 1.0f / ray->direction.z;
    float min[3] = {heap->min_x[idx], heap->min_y[idx], heap->min_z[idx]};
    float max[3] = {heap->max_x[idx], heap->max_y[idx], heap->max_z[idx]};
    int sign[3] = {dxi<0,dyi<0,dzi<0};
    float *params[2] = {min, max};

    float tmin = (params[sign[0]][0] - ray->origin.x) * dxi;
    float tmax = (params[1 - sign[0]][0] - ray->origin.x) * dxi;
    float tymin = (params[sign[1]][1] - ray->origin.y) * dyi;
    float tymax = (params[1- sign[1]][1] - ray->origin.y) * dyi;
    if(tmin > tymax || tymin > tmax)
      return;
    if(tymin > tmin)
      tmin = tymin;
    if(tymax < tmax)
      tmax = tymax;
    float tzmin = (params[sign[2]][2] - ray->origin.z) * dzi;
    float tzmax = (params[1 - sign[2]][2] - ray->origin.z) * dzi;
    if(tmin > tzmax || tzmin > tmax)
      return;

    int id_l = (idx) * 2 + 1;
    int id_r = (idx) * 2 + 2;
    if(id_l < heap->length)
      bvh_traverse_heap(ray, heap, id_l, closest);
    if(id_r < heap->length)
      bvh_traverse_heap(ray, heap, id_r, closest);
  }
}

intersection_t
bvh_traverse(ray_t *ray, bvhnode_t *node)
{
  intersection_t closest = {.hit = 0};
  if(!aabb_intersect(&node->bb, ray))
    return closest;

  primitive_t *primitive = node->primitive;
  if(primitive != NULL) {
    //Only leaves have primitives
    intersection_t its;
    its.hit = 0;
    prm_intersect(primitive, ray, &its);
    if(its.hit && its.distance > 0.001f && its.distance < closest.distance) {
      v3_mul_scalar(&its.hit_point, &ray->direction, its.distance);
      v3_add(&its.hit_point, &its.hit_point, &ray->origin);

      if(primitive->type == TRIANGLE) {
        triangle_t *triangle = primitive->obj;
        v3_copy(&its.normal, &triangle->normal);
        its.material = triangle->material;
      }
      else if(primitive->type == SPHERE) {
        sphere_t *sphere = primitive->obj;
        v3_sub(&its.normal, &its.hit_point, &sphere->center);
        v3_normalize(&its.normal);
        its.material = sphere->material;
      }
      //memcpy(closest, &its, sizeof(intersection_t));

      v3_copy(&closest.hit_point, &its.hit_point);
      v3_copy(&closest.normal, &its.normal);
      closest.material = its.material;
      closest.distance = its.distance;
      closest.hit = its.hit;

    }
  }
  else {
    //Not a leaf
    if(node->left)
      return bvh_traverse(ray, node->left);
    if(node->right)
      return bvh_traverse(ray, node->right);
  }

  return closest;
}

void
simple_traverse(ray_t *ray, list_t *primitives, intersection_t *closest)
{
  primitive_t *primitive;
  intersection_t its;
  node_t * node = list_head(primitives);
  while(node) {
    primitive = (primitive_t*) node->item;
    prm_intersect(primitive, ray, &its);
    if(its.hit && its.distance > 0.001f) {
      if (closest->hit == 0 || its.distance < closest->distance) {
        v3_mul_scalar(&its.hit_point, &ray->direction, its.distance);
        v3_add(&its.hit_point, &its.hit_point, &ray->origin);

        if(primitive->type == TRIANGLE) {
          triangle_t *triangle = primitive->obj;
          v3_copy(&its.normal, &triangle->normal);
          its.material = triangle->material;
        }
        else if(primitive->type == SPHERE) {
          sphere_t *sphere = primitive->obj;
          v3_sub(&its.normal, &its.hit_point, &sphere->center);
          v3_normalize(&its.normal);
          its.material = sphere->material;
        }

        v3_copy(&closest->hit_point, &its.hit_point);
        v3_copy(&closest->normal, &its.normal);
        closest->material = its.material;
        closest->distance = its.distance;
        closest->hit = its.hit;
      }			
    }
    node = list_next(node);
  }
}

intersection_t
find_closest(ray_t* ray, world_t* world, float max_distance)
{
  intersection_t closest;

  closest.hit = 0;
  closest.distance = 1e16f;

  //int hits = 0;
  //simple_traverse(ray, world->primitives, &closest);
  //bvh_traverse(ray, world->bvh->root, &closest);
  bvh_traverse_heap(ray, world->bvh->heap, 0, &closest);
  //if(hits)
  //printf("hits: %i\n", hits);

  //memcpy(result, &closest, sizeof(intersection_t));

  return closest;
}

void
traceray(ray_t *ray, world_t *world, color_t *color)
{
  //intersection_t result;
  float max_distance = 1000.0f;

  intersection_t result = find_closest(ray, world, max_distance);
  if (result.hit) {
    //color_init(color, 1.0f, 1.0f, 1.0f, 0.0f);
    if(result.material.reflection > 0) {
      ray_t rf_ray;
      v3_t vtemp;
      float t = v3_dot(&ray->direction, &result.normal) * 2;
      v3_mul_scalar(&vtemp, &result.normal, t);
      v3_sub(&rf_ray.direction, &ray->direction, &vtemp);
      v3_normalize(&rf_ray.direction);
      rf_ray.origin = result.hit_point;
      traceray(&rf_ray, world, color);
    }
    else {
      shading(world, &result, color);
    }
  }
  else
  {
    color_set_argb(color, 1.0f, 0.0f, 0.0f, 0.0f);
  }
}

float randf()
{
  return (float)rand() / (float)RAND_MAX ;
}

v3_t
rnd_dome(const v3_t* normal)
{
  v3_t p;
  float d;
  do
  {
    p.x = 2.0f * randf() - 1.0f;
    p.y = 2.0f * randf() - 1.0f;
    p.z = 2.0f * randf() - 1.0f;

    v3_normalize(&p);

    d = v3_dot(&p, normal);
  } while(d <= 0);

  return p;
}

/*
color_t
pathtrace(ray_t *ray, world_t *world,int depth)
{
  intersection_t result;
  float max_distance = 1000.0f;
  color_t color;
  color_set_argb(&color, 1.0f, 0.0f, 0.0f, 0.0f);

  find_closest(ray, world, max_distance, &result);
  if (result.hit) {
      color_copy(&color, &(result.material->color));
  }
  if (result.hit && depth < 10) {
    if (!result.material->is_light) {
      ray_t nray;
      nray.origin = result.hit_point;
      nray.direction = rnd_dome(&(result.normal));
      color_t ncolor = pathtrace(&nray, world, depth + 1);
      float at = v3_dot(&nray.direction, &(result.normal));
      color_mul_scalar(&ncolor, &ncolor, at);
      color_mul(&color, &color, &ncolor);
    }
    else
    {
      color_copy(&color, &(result.material->color));
    }
  }
  return color;
}*/

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
    for (x = 0; x < width; ++x)
    {
      int p = y*width + x;
      ray_t ry;
      color_t color;
      getray(&ry, x, y, job);
      traceray(&ry, job->world, &color);
      //color = pathtrace(&ry, job->world, 0);
      //ARGB
      buffer[p] = color_to_argb(&color);
    }
  }
  return 0;
}
