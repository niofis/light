#include "includes.h"

material_t*
material_new()
{
	material_t* material;
	material = (material_t*) malloc(sizeof(material_t));
  	color_set_argb(&material->color, 1.0f, 1.0f, 1.0f, 1.0f);
	return material;
}
void
material_destroy(material_t **material)
{
	if(*material)
	{
		free(*material);
        *material = NULL;
	}
}
