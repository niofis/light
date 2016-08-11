#include <stdlib.h>
#include "job.h"

job_t*
job_new()
{
	job_t *job;

	job = (job_t*) malloc(sizeof(job_t));

	job->width = 640; //1280;
	job->height = 360;// 720;
	job->buffer = (int*) malloc(sizeof(int) * job->width * job->height);
	//job->world = world_cornell();
    job->world = world_from_model("../models/bunny_low_res.ply");

	return job;
}

void
job_destroy(job_t **job)
{
	if (*job)
	{
      //TODO: destroy world
      //TODO: destroy buffer
		free(*job);
        *job = NULL;
	}
}
