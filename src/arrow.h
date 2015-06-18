#pragma once
#ifndef ARROW
#define ARROW

#include <stdint.h>
#include <math.h>
#include "almalloc.h"

struct arr_desc
{
  uint32_t length;
  uint32_t pad[3];
};

struct arr_params
{
  void* dest;
  void* arr1;
  void* arr2;
  uint32_t length;
  uint32_t offset;
};
  

float* arr_ps_new(uint32_t length)
{
  uint8_t* arr;
  struct arr_desc* desc;

  arr = aligned_malloc(16, sizeof(struct arr_desc) + sizeof(float)*length);

  desc = (struct arr_desc*)arr;
  desc->length = length;
  
  arr += sizeof(struct arr_desc);

  return (float*)arr;
}

void arr_ps_del(void* arr)
{
  void* ptr = (struct arr_desc*)arr - 1;

  aligned_free(ptr);
}

struct arr_desc* arr_desc_get(void* arr)
{
  return (struct arr_desc*)arr - 1;
}

/***Arithmetic operations***/
void arr_ps_add(float* dest, float* arr1, float* arr2)
{
	const struct arr_desc* desc = arr_desc_get(arr1);
	const uint32_t size = desc->length;
	int i = 0;

	//#pragma omp parallel for private(i) //num_threads(8)
	for (i = 0; i < size; ++i)
	{
		dest[i] = arr1[i] - arr2[i];
	};
}

void arr_ps_sub(float* dest, float* arr1, float* arr2)
{
	const struct arr_desc* desc = arr_desc_get(arr1);
	const uint32_t size = desc->length;
  int i = 0;

	#pragma omp parallel for private(i)
  for(i = 0; i < size; ++i)
  {
    dest[i] = arr1[i] - arr2[i];
  } 
}

void arr_ps_mul(float* dest, float* arr1, float* arr2)
{
  const struct arr_desc* desc = arr_desc_get(arr1);
  const uint32_t size = desc->length;
  int i = 0;

  #pragma omp parallel for private(i)
  for(i = 0; i < size; ++i)
  {
    dest[i] = arr1[i] * arr2[i];
  }
}

void arr_ps_div(float* dest, float* arr1, float* arr2)
{
  const struct arr_desc* desc = arr_desc_get(arr1);
  const uint32_t size = desc->length;
  int i = 0;

  #pragma omp parallel for private(i)
  for(i = 0; i < size; ++i)
  {
    dest[i] = arr1[i] / arr2[i];
  }
}

void arr_ps_sqrt(float* dest, float* arr1)
{
  const struct arr_desc* desc = arr_desc_get(arr1);
  const uint32_t size = desc->length;
  int i = 0;

  #pragma omp parallel for private(i)
  for(i = 0; i < size; ++i)
  {
    dest[i] = sqrtf(arr1[i]);
  }
}

#endif
