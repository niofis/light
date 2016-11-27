#include "includes.h"

camera_t*
camera_new()
{
	camera_t* camera;

	camera = (camera_t*) malloc(sizeof(camera_t));

	camera->left_top.x = -8.0f;
	camera->left_top.y = 9.0f;
	camera->left_top.z = -50.0f;

	camera->right_top.x = 8.0f;
	camera->right_top.y = 9.0f;
	camera->right_top.z = -50.0f;

	camera->left_bottom.x = -8.0f;
	camera->left_bottom.y = 0.0f;
	camera->left_bottom.z = -50.0f;

	camera->eye.x = 0.0f;
	camera->eye.y = 4.5f;
	camera->eye.z = -75.0f;

	return camera;
}

void
camera_rotate_xyz(camera_t *camera, float rx, float ry, float rz)
{
  v3_rotate_xyz(&camera->left_top, rx, ry, rz);
  v3_rotate_xyz(&camera->right_top, rx, ry, rz);
  v3_rotate_xyz(&camera->left_bottom, rx, ry, rz);
  v3_rotate_xyz(&camera->eye, rx, ry, rz);
}

void
camera_destroy(camera_t **camera)
{
	if (*camera)
	{
		free(*camera);
        *camera = NULL;
	}
}

void
camera_rotate(float rx, float ry, float rz)
{
  return;
}
