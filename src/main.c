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

#include "timer.h"
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
	struct timer timer;

	timer_start(&timer);
	render(job);
	timer_stop(&timer);

	printf("Render time = %fs\n", timer.elapsed);
	/*
	printf("Writing image.ppm file...\n");
	ppm_create("image.ppm",job->width,job->height,255,PPM_P6,job->buffer);
	*/
	printf("Done!\n");

	job_del(job);

	return 0;
}

