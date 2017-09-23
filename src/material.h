#pragma once
#include "includes.h"

typedef struct
{
	color_t color;
    float reflection;
    float refraction;
    float specular;
} material_t;

extern const material_t m_white;
extern const material_t m_blue;
extern const material_t m_red;
extern const material_t m_green;
extern const material_t m_black;
extern const material_t m_gray;
extern const material_t m_orange;
extern const material_t m_purple;
extern const material_t m_cyan;
extern const material_t m_yellow;

extern const material_t m_steel;

material_t
  material_new();

void
  material_destroy(material_t **material);
