#ifndef PPM_H
#define PPM_H

/*
########  ########  ##     ##
##     ## ##     ## ###   ###
##     ## ##     ## #### ####
########  ########  ## ### ##
##        ##        ##     ##
##        ##        ##     ##
##        ##        ##     ##
*/

extern char* ppm_magic;

int ppm_create(char* filename, int width, int height, short max_color, int* data);

#endif