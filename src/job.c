#include "includes.h"

job_t*
job_new(int width, int height)
{
	job_t *job;

	job = (job_t*) malloc(sizeof(job_t));

	job->width = width;//480;//640; //1280;
	job->height = height;//272;//360;// 720;
	job->buffer = (int*) malloc(sizeof(int) * job->width * job->height);
	//job->world = world_demo();
	job->world = world_cornell();
    //job->world = world_from_model("../models/bunny_low_res.ply");
    //job->world = world_new();

	return job;
}

void
job_destroy(job_t **job)
{
	if (*job)
	{
      job_t *j = *job;
      world_destroy(&(j->world));
      free(j->buffer);
      free(j);
      *job = NULL;
	}
}
