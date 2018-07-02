build:
	GOARCH=wasm GOOS=js ${GOROOT}/bin/go build -o ./wasm/main.wasm ./src/main.go

build-server:
	go build -o ./server ./src/server.go

run:
	./server ${PORT}