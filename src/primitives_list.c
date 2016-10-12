#include "includes.h"

struct prm_list* prm_list_new()
{
    struct prm_list* lst = (struct prm_list*) malloc(sizeof(struct prm_list));
    return lst;
}

//prm_list_del will dispose of individual elements
void prm_list_del(struct prm_list* list)
{
    struct prm_it* it;
    struct primitive* prm;

    it = prm_it_new(list);

    prm = prm_it_next(it);

    while(prm != 0)
    {
        prm_del(prm);
        prm = prm_it_next(it);
    }

    prm_it_del(it);
        
}

struct prm_it* prm_it_new(struct prm_list* list)
{
    struct prm_it* it;
    
    it = (struct prm_it*) malloc(sizeof(struct prm_it));

    it->current = list->first;

    return it;

}

void prm_it_del(struct  prm_it* it)
{
    if(it != 0)
    {
        free(it);
    }
}


struct primitive* prm_it_next(struct prm_it* it)
{
    struct primitive* prm = 0;
    if(it != 0)
    {
        prm = it->current;
        if(prm != 0)
        {
            it->current = prm->next;
        }
    }
    return prm;
}
