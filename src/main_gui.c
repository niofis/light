#include "includes.h"

#if !defined(byte)
  #define byte unsigned char
#endif

#define BPP 4

int gui_init(int width, int height, bool fullscreen)
{
  int running = 1;
  SDL_Window *window;
  SDL_Renderer *renderer;
  SDL_Texture *texture;
  SDL_Event event;
  job_t *job;
  struct timer timer;

  job = job_new(width, height);
  
  if (SDL_Init(SDL_INIT_VIDEO) != 0) {
    printf("SDL_Init failed: %s\n", SDL_GetError());
	SDL_Quit();
    return 1;
  }

  window = SDL_CreateWindow("Light", 0, 0, job->width, job->height, SDL_WINDOW_SHOWN);
  if (window == NULL) {
	  printf("SDL_CreateWindow failed: %s\n", SDL_GetError());
	  SDL_Quit();
	  return 1;
  }

  if (fullscreen && SDL_SetWindowFullscreen(window, SDL_WINDOW_FULLSCREEN) != 0) {
    printf("SDL_SetWindowFullscreen failed: %s\n", SDL_GetError());
    return 1;
  }

  renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
  if (renderer == NULL) {
	  SDL_DestroyWindow(window);
	  printf("SLD_CreateRenderer failed: %s\n", SDL_GetError());
	  SDL_Quit();
	  return 1;
  }

  texture = SDL_CreateTexture(renderer, SDL_PIXELFORMAT_ARGB8888, SDL_TEXTUREACCESS_STREAMING, job->width, job->height);
  if (texture == NULL) {
	  SDL_DestroyRenderer(renderer);
	  SDL_DestroyWindow(window);
	  printf("SDL_CreateTexture failed: %s\n", SDL_GetError());
	  return 1;
  }
  
  char buffer[256];
  
  while (running) {
    if (SDL_PollEvent(&event)) {
      switch (event.type) {
        case SDL_KEYDOWN:

          if (event.key.keysym.sym == SDLK_ESCAPE) {
            running = 0;
          }
          
          break;
      }
    }

    timer_start(&timer);
    render(job);
    timer_stop(&timer);

    //printf("Render time = %fs\n", timer.elapsed);

    SDL_UpdateTexture(texture, NULL, job->buffer, job->width * sizeof(Uint32));
    //SDL_RenderClear(renderer);
    SDL_RenderCopy(renderer, texture, NULL, NULL);

    sprintf(buffer, "Render time = %fs", timer.elapsed);
    stringRGBA(renderer, 0, 0, buffer, 255,255,255,255);

    SDL_RenderPresent(renderer);

    camera_rotate_xyz(job->world->camera, 0, 0.1f, 0);

    usleep(10);
  }

  SDL_DestroyTexture(texture);
  SDL_DestroyRenderer(renderer);
  SDL_DestroyWindow(window);
  
  SDL_Quit();
  
  job_destroy(&job);

  return 0;
}

int main(int argc, char **argv)
{
  int width = 480;
  int height = 272;
  bool fullscreen = false;

  for(int i = 0; i < argc; i++) {
    if(strcmp("-w", argv[i]) == 0) {
      width = atoi(argv[++i]);
    }
    else if(strcmp("-h", argv[i]) == 0) {
      height = atoi(argv[++i]);
    }
    else if(strcmp("-f", argv[i]) == 0) {
      fullscreen = true;
    }
  }
  
  gui_init(width, height, fullscreen);

#ifdef DEBUG
  debug_end();
#endif

  return 0;
}
