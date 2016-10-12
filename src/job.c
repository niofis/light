#include "includes.h"

job_t*
job_new()
{
	job_t *job;

    printf("aki1!\n");
	job = (job_t*) malloc(sizeof(job_t));
    printf("aki2!\n");

	job->width = 480;//640; //1280;
    printf("aki3!\n");
	job->height = 272;//360;// 720;
	job->buffer = (int*) malloc(sizeof(int) * job->width * job->height);
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
      //TODO: destroy world
      //TODO: destroy buffer
		free(*job);
        *job = NULL;
	}
}
