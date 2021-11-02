mkdir -p pkg
emcc -O2 -s -pthread -s PTHREAD_POOL_SIZE=4 ./src/test.c -o ./pkg/test.js
python3 -m http.server