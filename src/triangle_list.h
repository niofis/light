#ifndef TRIANGLE_LIST
#define TRIANGLE_LIST

struct triangle_list {
  int count;
  float* v1_x;
  float* v1_y;
  float* v1_z;
  float* v2_x;
  float* v2_y;
  float* v2_z;
  float* v3_x;
  float* v3_y;
  float* v3_z;
  float* normal_x;
  float* normal_y;
  float* normal_z;
  int* material_idx;
  int* group_idx;
};

struct triangle_list* triangle_list_new(int count);
void triangle_list_del(struct triangle_list* list);


#endif
