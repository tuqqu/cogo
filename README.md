# Cogo

A bytecode compiler and a virtual machine for a subset of Go, written in Rust


```go
// an example of a fully compiling program

package main

var (
    vec = [5]int{1, 2, 3, 4, 5}
    q = 5
)

func main() {    
    if contains(vec, q) {
        println("Value 5 is in the vector")
    }
}

func contains(vec [5]int, val int) bool {
    for i := 0; i < len(vec); i++ {
        if vec[i] == val {
            return true
        }
    }
    
    return false
}
```

### Implemented Features

- [x] primitive types and type conversions
- [x] comparison, math, logic operations
- [x] global and scoped variables
- [x] global and scoped constants
- [x] group declarations
- [x] `if` statements
- [x] `for` statements (with `breaks` and `continue`)
- [x] `switch` statements (with `fallthrough`)
- [x] functions (+ recursive calls)
- [x] partial support of `builtin.go`
- [x] arrays
- [ ] closures
- [ ] `range` and `for range` loops
