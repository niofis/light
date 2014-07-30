#include "timer.h"

void timer_start(struct timer* timer)
{
	timer->start = clock();
}
void timer_stop(struct timer* timer)
{
	timer->stop = clock();
	timer->elapsed = (float)(timer->stop - timer->start) 
		/ (float)CLOCKS_PER_SEC;
}

