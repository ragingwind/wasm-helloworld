#include <emscripten/emscripten.h>

EMSCRIPTEN_KEEPALIVE
const char* HELLO_WORLD = "Hello World!";

EMSCRIPTEN_KEEPALIVE
const char* str() {
  return HELLO_WORLD;
}

EMSCRIPTEN_KEEPALIVE
unsigned int len() {
  unsigned int len = 0;

  for (const char* p = HELLO_WORLD; *p != '\0'; p++) {
    len++;
  }

  return len;
}