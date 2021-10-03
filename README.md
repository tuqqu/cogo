# Cogo

A bytecode compiler and a virtual machine for a subset of Go, written in Rust


```go
// an example of a fully compiling program

package main

var (
    x uint8 = 255
    vec     = []int{45, 23, -7, int(x), 0, -102}
)

func main() {
    insertionSort(vec) // [-102 -7 0 23 45 255]
    println("Vector is sorted.")
}

func insertionSort(vec []int) {
    for i, n := 1, len(vec); i < n; i++ {
        j := i
        for j > 0 {
            if vec[j-1] > vec[j] {
                vec[j], vec[j-1] = vec[j-1], vec[j]
            }
            j--
        }
    }
}
```

### Implemented Features

- [x] primitive types and type conversions
- [x] comparison, math, logic, bitwise operations
- [x] variables and constants
  - [x] global and scoped
  - [x] group declarations
  - [x] multiple declarations and assignments
- [x] control flow
  - [x] `if` statements
  - [x] `for` statements (with `breaks` and `continue`)
  - [x] `switch` statements (with `fallthrough`)
- [x] functions
  - [x] recursive functions
  - [x] variadic functions
  - [x] multiple return values
- [x] partial support of `builtin.go`
- [x] arrays
- [x] slices (partially)
- [ ] closures
- [ ] `range` and `for range` loops
