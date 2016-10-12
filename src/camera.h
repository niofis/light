#pragma once

#include "includes.h"

typedef struct
{
	v3_t left_top;
	v3_t left_bottom;
	v3_t right_top;
	v3_t eye;
} camera_t;

camera_t*
  camera_new();

void
  camera_destroy(camera_t **camera);
