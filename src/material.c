#include <stdlib.h>
#include "material.h"

struct material* material_new(int number)
{
	struct material* materials;
	materials = (struct material*) malloc(sizeof(struct material) * number);
	return materials;
}
void material_delete(struct material* material)
{
	if(material)
	{
		free(material);
	}
}
