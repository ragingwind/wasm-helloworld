#include <emscripten/emscripten.h>

const char* HELLO_WORLD = "Hello World!";

EMSCRIPTEN_KEEPALIVE
const char* getString() {
  return HELLO_WORLD;
}

unsigned int getLength() {
  unsigned int len = 0;

  for (const char* p = HELLO_WORLD; *p != '\0'; p++) {
    len++;
  }

  return len;
}