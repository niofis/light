#ifndef TIMER_H
#define TIMER_H

#include <time.h>

struct timer
{
	long start;
	long stop;
	float elapsed;
};

void timer_start(struct timer* timer);
void timer_stop(struct timer* timer);

#endif
