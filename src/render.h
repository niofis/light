#ifndef RENDER_H
#define RENDER_H




typedef struct result
{
	struct color color;
	struct vector3 point;
	int hit;
};




int render(struct job_desc* job);




#endif