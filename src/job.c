#include <stdlib.h>
#include "job.h"

job_t*
job_new()
{
	job_t *job;

	job = (job_t*) malloc(sizeof(job_t));

	job->width = 1280;
	job->height = 720;
	job->buffer = (int*) malloc(sizeof(int) * job->width * job->height);
	job->world = world_cornell();

	return job;
}

void
job_destroy(job_t **job)
{
	if (*job)
	{
		free(*job);
        *job = NULL;
	}
}
