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

//I'm tired of fiddling around with so many text files
//let's try something different.

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

/*
########  ########  ##     ##
##     ## ##     ## ###   ###
##     ## ##     ## #### ####
########  ########  ## ### ##
##        ##        ##     ##
##        ##        ##     ##
##        ##        ##     ##
*/

char* ppm_magic = "P6";

int ppm_create(char* filename, int width, int height, short max_color, int* data)
{
	int x=0;
	int y=0;
	FILE *file;
	file = fopen(filename,"w+");
	if(file)
	{
		fprintf(file,"%s %i %i %i ",ppm_magic,width,height,max_color);

		for(y=0;y<height;++y)
		{
			for(x=0;x<width;++x)
			{
				int p = y*height + x;
				//gets the address for the pixel data
				int pixel = data[p];
				//byte* pixel=(byte*)(&px);
				/*
				fputc(pixel[1],file);
				fputc(pixel[2],file);
				fputc(pixel[3],file);
				*/

				fputc((pixel & 0x00FF0000) >> 16, file);
				fputc((pixel & 0x0000FF00) >> 8, file);
				fputc((pixel & 0x000000FF), file);

				//printf("(%i,%i,%i) %X \n",pixel[1],pixel[2],pixel[3],px);
			}
		}

		fclose(file);
	}
	else
	{
		return 1;
	}
	return 0;
}

/*
########  ######## ##    ## ########  ######## ########
##     ## ##       ###   ## ##     ## ##       ##     ##
##     ## ##       ####  ## ##     ## ##       ##     ##
########  ######   ## ## ## ##     ## ######   ########
##   ##   ##       ##  #### ##     ## ##       ##   ##
##    ##  ##       ##   ### ##     ## ##       ##    ##
##     ## ######## ##    ## ########  ######## ##     ##
*/

typedef struct
{
	float a;
	float r;
	float g;
	float b;
} st_color;

typedef struct
{
	float x;
	float y;
	float z;
} st_vector3;

typedef struct
{
	st_vector3 left_top;
	st_vector3 lef_bottom;
	st_vector3 right_top;
	st_vector3 eye;
} st_camera;

typedef struct
{
	st_vector3 pt1;
	st_vector3 pt2;
	st_vector3 pt3;
} st_triangle;

typedef struct
{
	st_vector3 center;
	float radius;
} st_sphere;

typedef struct
{
	st_camera *camera;
	st_triangle *objects;
} st_scene;

typedef struct
{
	st_vector3 direction;
	st_vector3 origin;
	float refraction_index;
} st_ray;

typedef struct
{
	int hit;
	st_color color;
	st_vector3 point;
	st_triangle object;
} st_result;

st_ray rnd_getray(int x, int y)
{
	st_ray ray;
	//ray=(st_ray *)malloc(sizeof(st_ray));
	ray.direction.x=0;
	return ray;
}

typedef struct
{
	float width;
	float height;
	int *buffer;
	st_scene scene;
} job_desc;

job_desc job_demo()
{
	job_desc job;

	job.width=80; //1280;
	job.height=45; //720;

	job.buffer = (int *)malloc(sizeof(int)*job.width*job.height);

	return job;
}

//returns color
st_result rnd_traceray(st_ray ray)
{
	st_result result;
	result.color.a=1.0f;
	result.color.r=0.0f;
	result.color.g=0.0f;
	result.color.b=1.0f;
	return result;
}

int rnd_color_to_argb(st_color color)
{
	int argb=0;
	argb = (int)((color.a<1.0f?color.a:1.0f)*255);
	argb = argb<<8;
	argb |= (int)((color.r<1.0f?color.r:1.0f)*255);
	argb = argb<<8;
	argb |= (int)((color.g<1.0f?color.g:1.0f)*255);
	argb = argb<<8;
	argb |= (int)((color.b<1.0f?color.b:1.0f)*255);
	//printf("(%f,%f,%f,%f) %X \n",color.a,color.r,color.g,color.b,argb);
	return argb;
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

int render(job_desc job)
{
	int x=0;
	int y=0;
	int width=job.width;
	int height=job.height;
	int *buffer=job.buffer;


	for(y=0;y<height;++y)
	{
		for(x=0;x<width;++x)
		{
			int p=y*height + x;
			st_result res;
			st_ray ray;
			ray=rnd_getray(x,y);
			res=rnd_traceray(ray);
			//ARGB
			buffer[p]=rnd_color_to_argb(res.color);
		}
	}
	return 0;
}

int main(int argc, char** argv)
{
	job_desc job = job_demo();

	render(job);

	ppm_create("image.ppm",job.width,job.height,255,job.buffer);

	free(job.buffer);

	return 0;
}

