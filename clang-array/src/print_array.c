#include <emscripten.h>

EM_JS(void, iprint, (int n), {
  console.log(n);
});

EM_JS(void, fprint, (float n), {
  console.log(n);
});

EM_JS(void, dprint, (double n), {
  console.log(n);
});

EMSCRIPTEN_KEEPALIVE
void print_iarray(int* in, int size) {
  for (int i = 0; i < size; i++) {
    iprint(in[i]);
  }
}

EMSCRIPTEN_KEEPALIVE
void print_farray(float* in, int size) {
  for (int i = 0; i < size; i++) {
    fprint(in[i]);
  }
}

EMSCRIPTEN_KEEPALIVE
void print_darray(double* in, int size) {
  for (int i = 0; i < size; i++) {
    dprint(in[i]);
  }
}