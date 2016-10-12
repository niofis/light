#include "includes.h"

#undef malloc
#undef free

void*
malloc_debug(char* file, int line, size_t size)
{
  printf("allocated %u bytes in line %i of file %s\n", size, line, file);
  void *ptr = malloc(size);
  return ptr;
}

void
free_debug(void* loc)
{
  //printf("freed location %u\n", loc);
  free(loc);
}

void
debug_nl()
{
	printf("\n");
}

void
debug_pause()
{
	//system("pause");
}

/*
void ray_dump(ray_t* ray)
{
	printf("{\norigin: \n{\n");
	v3_dump(&ray->origin);
	printf("},\ndirection: \n{\n");
	v3_dump(&ray->direction);
	printf("}\n}\n");
}
*/
