#pragma once
#ifndef MEMORY
#define MEMORY

#include <stdlib.h>

#define aligned_malloc(alignment_bytes, size) malloc(size);
#define aligned_free(ptr) free(ptr);

#endif
