#include "includes.h"

const material_t m_white = {.color = c_white};
const material_t m_blue = {.color = c_blue};
const material_t m_red = {.color = c_red};
const material_t m_green = {.color = c_green};
const material_t m_black = {.color = c_black};
const material_t m_gray = {.color = c_gray};
const material_t m_orange = {.color = c_orange};
const material_t m_purple = {.color = c_purple};
const material_t m_cyan = {.color = c_cyan};
const material_t m_yellow = {.color = c_yellow};

const material_t m_steel = {.color = c_gray, .reflection = 1.0f};

material_t
material_new()
{
	material_t material = {{1.0f, 1.0f, 1.0f, 1.0f}};
	//material = (material_t*) malloc(sizeof(material_t));
  	//color_set_argb(&material.color, 1.0f, 1.0f, 1.0f, 1.0f);
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
