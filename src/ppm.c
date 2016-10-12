#include "includes.h"

int ppm_create(char* filename, int width, int height, short max_color,int format, int* data)
{
	int x = 0;
	int y = 0;
	FILE *file;

	char* ppm_magic = "";
	char* pixel_format = "";

	if (format == PPM_P6) {
		ppm_magic = "P6";
		pixel_format = "%c%c%c";
	}
	else {
		ppm_magic = "P3";
		pixel_format = " %i %i %i ";
	}


	file = fopen(filename, "w+");
	if (file)
	{
		fprintf(file, "%s %i %i %i\n", ppm_magic, width, height, max_color);

		for (y = 0; y<height; ++y)
		{
			for (x = 0; x<width; ++x)
			{
				int p = y*width + x;
				//gets the address for the pixel data
				int pixel = data[p];
				//byte* pixel=(byte*)(&px);
				/*
				fputc(pixel[1],file);
				fputc(pixel[2],file);
				fputc(pixel[3],file);
				*/

				
				//fputc((pixel & 0x00FF0000) >> 16, file);
				//fputc((pixel & 0x0000FF00) >> 8, file);
				//fputc((pixel & 0x000000FF), file);
				
				//fprintf(file, " %i %i %i ", 
				//		(pixel & 0x00FF0000) >> 16, 
				//		(pixel & 0x0000FF00) >> 8, 
				//		(pixel & 0x000000FF));
				//if (format == PPM_P6)
				//{
				//	fprintf(file, "%c%c%c",
				//		(pixel & 0x00FF0000) >> 16,
				//		(pixel & 0x0000FF00) >> 8,
				//		(pixel & 0x000000FF));
				//}
				//else
				//{
				//	fprintf(file, " %i %i %i ", 
				//			(pixel & 0x00FF0000) >> 16, 
				//			(pixel & 0x0000FF00) >> 8, 
				//			(pixel & 0x000000FF));
				//}
				//printf("(%i,%i,%i) %X \n",pixel[1],pixel[2],pixel[3],px);
				fprintf(file, pixel_format,
					(pixel & 0x00FF0000) >> 16,
					(pixel & 0x0000FF00) >> 8,
					(pixel & 0x000000FF));
			}
			if (format == PPM_P3)
				fprintf(file, "\n");
		}

		fclose(file);
	}
	else
	{
		return 1;
	}
	return 0;
}
