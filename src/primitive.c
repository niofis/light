#include <stdlib.h>
#include "primitive.h"

int prm_intersect(struct primitive* prm, struct ray* ray, struct intersection* result);

void prm_del(struct primitive* prm)
{
    if(prm != 0)
    {
        if(prm->type == SPHERE)
        {
            sphere_del(prm->obj);
        }
        else if(prm->type == TRIANGLE)
        {
            triangle_del(prm->obj);
        }
        else
        {
            free(prm->obj);
        }

        free(prm);
    }
}

