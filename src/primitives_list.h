#ifndef PRIMITIVES_LIST
#define PRIMITIVES_LIST

#include "primitive.h"

struct prm_list 
{
	int count;
	struct primitive* first;
	struct primitive* last;
};

struct prm_it
{
	struct primitive* current;
};

struct prm_list* prm_list_new();
//prm_list_del will dispose of individual elements
void prm_list_del(struct prm_list* list);

struct prm_it* prm_it_new(struct prm_list* list);
void prm_it_del(prm_it* it);
struct primitive* prm_it_next(struct prm_it* it);


#endif
