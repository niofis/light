#include <stdlib.h>
#include "material.h"

material_t*
material_new(int number)
{
	material_t* materials;
	materials = (material_t*) malloc(sizeof(material_t) * number);
	for(int i = 0; i < number; ++i) {
		color_set_argb(&materials[i].color, 1.0f, 1.0f, 1.0f, 1.0f);
	}
	return materials;
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
