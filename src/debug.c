#include "includes.h"

#undef malloc
#undef free

struct _dbg_node {
  void *ptr;
  size_t size;
  int line;
  char *file;
  struct _dbg_node *next;
};

typedef struct _dbg_node dbg_node_t;

dbg_node_t *dbg_head = NULL;
dbg_node_t *dbg_tail = NULL;

void
debug_init()
{

}



void
dbg_node_add(dbg_node_t *node)
{
  if(dbg_tail == NULL) {
    dbg_head = dbg_tail = node;
  } else {
    dbg_tail->next = node;
    dbg_tail = node;
  }
}

void
dbg_node_del(void *ptr)
{

  dbg_node_t *prev = NULL;
  dbg_node_t *cur = dbg_head;

  while(cur && cur->ptr != ptr) {
    prev = cur;
    cur = cur->next;
  }

  if(cur) {
    if(prev)
      prev->next = cur->next;
    else
      dbg_head = cur->next;
    free(cur);
    cur = NULL;
  }
}

void*
malloc_debug(char* file, int line, size_t size)
{
  void *ptr = malloc(size);
  dbg_node_t *node = malloc(sizeof(dbg_node_t));
  node->ptr = ptr;
  node->file = file;
  node->line = line;
  node->size = size;
  dbg_node_add(node);
  return ptr;
}

void
free_debug(char* file, int line, void* loc)
{
  dbg_node_del(loc);
  //printf("bleh\n");
  free(loc);
  //printf("freed location %u\n", loc);
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
void
debug_end()
{
  dbg_node_t *cur = dbg_head;
  dbg_node_t *tmp;
  while(cur) {
    printf("%ub -> %s:%i\n", (unsigned int)cur->size, cur->file, cur->line);
    tmp = cur;
    cur = cur->next;
    free(tmp);
  }
}
