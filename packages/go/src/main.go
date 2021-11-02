package main

import (
	"syscall/js"
)

func main() {
	output := js.Global().Get("document").Call("querySelector", "#output")
	output.Set("textContent", Add(10, 90))
}

func Add(a, b int) int {
	return a + b
}
