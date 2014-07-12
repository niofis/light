//For ASCII banners:
//http://www.network-science.de/ascii/
//font: banner3

/*
##       ####  ######   ##     ## ########
##        ##  ##    ##  ##     ##    ##
##        ##  ##        ##     ##    ##
##        ##  ##   #### #########    ##
##        ##  ##    ##  ##     ##    ##
##        ##  ##    ##  ##     ##    ##
######## ####  ######   ##     ##    ##
*/

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <omp.h>


/*
 ######   ##        #######  ########     ###    ##          ########  ######## ########  ######
##    ##  ##       ##     ## ##     ##   ## ##   ##          ##     ## ##       ##       ##    ##
##        ##       ##     ## ##     ##  ##   ##  ##          ##     ## ##       ##       ##
##   #### ##       ##     ## ########  ##     ## ##          ##     ## ######   ######    ######
##    ##  ##       ##     ## ##     ## ######### ##          ##     ## ##       ##             ##
##    ##  ##       ##     ## ##     ## ##     ## ##          ##     ## ##       ##       ##    ##
 ######   ########  #######  ########  ##     ## ########    ########  ######## ##        ######
 */
#if !defined(byte)
	#define byte unsigned char
#endif

#ifdef int32_t
	#define int int32_t
#endif


#define BPP 4

#include "ppm.h"
#include "job.h"
#include "render.h"


struct job_desc* job_demo()
{
	struct job_desc* job;

	job = job_new();


	return job;
}


/*
##     ##    ###    #### ##    ##
###   ###   ## ##    ##  ###   ##
#### ####  ##   ##   ##  ####  ##
## ### ## ##     ##  ##  ## ## ##
##     ## #########  ##  ##  ####
##     ## ##     ##  ##  ##   ###
##     ## ##     ## #### ##    ##
*/



int main(int argc, char** argv)
{
	struct job_desc* job = job_demo();
	double start;
	double finish;

	start = omp_get_wtime();
	render(job);
	finish = omp_get_wtime();

	printf("Render time = %fs", finish - start);

	ppm_create("image.ppm",job->width,job->height,255,job->buffer);

	job_del(job);

	return 0;
}

