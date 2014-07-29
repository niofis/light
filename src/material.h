#ifndef MATERIAL_H
#define MATERIAL_H

#include "color.h"

struct material
{
	struct color color;
};

struct material* material_new(int number);
void material_delete(struct material* material);

#endif
