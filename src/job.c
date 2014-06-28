#include <stdlib.h>
#include "job.h"
#include "primitives.h"

struct job_desc* job_new()
{
	struct job_desc* job;

	job = (struct job_desc*) malloc(sizeof(struct job_desc));

	job->width = 1280;
	job->height = 720;
	job->buffer = (int*) malloc(sizeof(int) * 3 * job->width * job->height);
	job->scene = scene_new();

	return job;
}

void job_del(struct job_desc* job)
{

}