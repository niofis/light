#ifndef JOB_H
#define JOB_H

#include "scene.h"

struct job_desc
{
	int width;
	int height;
	int *buffer;
	struct scene* scene;
};


struct job_desc* job_new();
void job_del(struct job_desc*);

#endif
