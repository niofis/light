#include <stdlib.h>
#include "material.h"

struct material* material_new(int number)
{
	struct material* materials;
	materials = (struct material*) malloc(sizeof(struct material) * number);
	for(int i = 0; i < number; ++i)
	{
		color_init(&materials[i].color, 1.0f, 1.0f, 1.0f, 1.0f);
	}
	return materials;
}
void material_del(struct material* material)
{
	if(material)
	{
		free(material);
	}
}
