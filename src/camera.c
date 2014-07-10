#include <stdlib.h>
#include "camera.h"


struct camera* camera_new()
{
	struct camera* cmr;

	cmr = (struct camera*) malloc(sizeof(struct camera));

	cmr->left_top.x = -8.0f;
	cmr->left_top.y = 9.0f;
	cmr->left_top.z = -50.0f;

	cmr->right_top.x = 8.0f;
	cmr->right_top.y = 9.0f;
	cmr->right_top.z = -50.0f;

	cmr->left_bottom.x = -8.0f;
	cmr->left_bottom.y = 0.0f;
	cmr->left_bottom.z = -50.0f;

	cmr->eye.x = 0.0f;
	cmr->eye.y = 4.5f;
	cmr->eye.z = -75.0f;

	return cmr;
}

void camera_del(struct camera* cmr)
{
	if (cmr)
	{
		free(cmr);
	}
}

