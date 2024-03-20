package main

import (
	"fmt"
	"unsafe"
)

func main() {
	hello := make([]byte, 0, 15)
	hello = append(hello, []byte("hello,world")...)

	h1 := hello[1:3]
	h2 := hello[6:]

	//sh1 := (*reflect.SliceHeader)(unsafe.Pointer(&h1))

	fmt.Printf("%-2d %-2d %p %v\n", len(hello), cap(hello), unsafe.SliceData(hello), hello) // 11 15 0x14000110020 [104 101 108 108 111 44 119 111 114 108 100]
	fmt.Printf("%-2d %-2d %p %v\n", len(h1), cap(h1), unsafe.SliceData(h1), h1)             // 2  14 0x14000110021 [101 108]
	fmt.Printf("%-2d %-2d %p %v\n", len(h2), cap(h2), unsafe.SliceData(h2), h2)             // 5  9  0x14000110026 [119 111 114 108 100]
}
