package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

func wasmHandler(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/wasm")
	http.ServeFile(w, r, "./wasm/main.wasm")
}

func main() {
	var port string
	if len(os.Args) > 1 {
		port = os.Args[1]
	} else {
		port = "8080"
	}

	mux := http.NewServeMux()
	mux.Handle("/", http.FileServer(http.Dir(".")))
	mux.HandleFunc("/main.wasm", wasmHandler)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%s", port), mux))
}
