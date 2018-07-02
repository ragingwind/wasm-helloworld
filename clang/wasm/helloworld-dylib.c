#define HELLO_WORLD "Hello World!"

const char* str() {
  return HELLO_WORLD;
}

unsigned int len() {
  unsigned int len = 0;

  for (const char* p = HELLO_WORLD; *p != '\0'; p++) {
    len++;
  }

  return len;
}