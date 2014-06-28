#include "ppm.h"
#include <stdio.h>

char* ppm_magic = "P6";

int ppm_create(char* filename, int width, int height, short max_color, int* data)
{
	int x = 0;
	int y = 0;
	FILE *file;
	file = fopen(filename, "w+");
	if (file)
	{
		fprintf(file, "%s %i %i %i ", ppm_magic, width, height, max_color);

		for (y = 0; y<height; ++y)
		{
			for (x = 0; x<width; ++x)
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