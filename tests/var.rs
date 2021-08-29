mod common;

use common::compare_stderr_output;

#[test]
fn test_var_default() {
    compare_stderr_output(
        r#"
package main

var x0 bool
var x1 int8
var x2 int16
var x3 int32
var x4 int64
var x5 int
var x6 uint8
var x7 uint16
var x8 uint32
var x9 uint64
var x10 uint
var x11 uintptr
var x12 float32
var x13 float64
var x14 complex64
var x15 complex128
var x16 string

func main() {
    var x17 bool
    var x18 int8
    var x19 int16
    var x20 int32
    var x21 int64
    var x22 int
    var x23 uint8
    var x24 uint16
    var x25 uint32
    var x26 uint64
    var x27 uint
    var x28 uintptr
    var x29 float32
    var x30 float64
    var x31 complex64
    var x32 complex128
    var x33 string

    println(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16)
    println(x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31, x32, x33)
}
"#,
        "false 0 0 0 0 0 0 0 0 0 0 0 0e0 0e0 (0e0+0e0i) (0e0+0e0i) \nfalse 0 0 0 0 0 0 0 0 0 0 0 0e0 0e0 (0e0+0e0i) (0e0+0e0i) \n",
    )
}

#[test]
fn test_var_with_init() {
    compare_stderr_output(
        r#"
package main

var x0 bool = true
var x1 int8 = 10
var x2 int16 = 56 - 3
var x3 int32 = -31 - 45
var x4 int64 = 56
var x5 int = 43
var x6 uint8 = 56
var x7 uint16 = 89
var x8 uint32 = 8 * 8
var x9 uint64 = 90 - 1
var x10 uint = 45 - 0 + 8 / 1
var x11 uintptr = 45
var x12 float32 = 9.9 + 3
var x13 float64 = 99.1 / 45
var x16 string = "strin" + "g"

func main() {
    var x17 bool = false
    var x18 int8 = 9 - 9
    var x19 int16 = -45
    var x20 int32 = 1
    var x21 int64 = 8 * 7
    var x22 int = -923824832
    var x23 uint8 = +33
    var x24 uint16 = 5
    var x25 uint32 = 45 / 5
    var x26 uint64 = 12
    var x27 uint = 6
    var x28 uintptr = 4
    var x29 float32 = 0.34 - 4
    var x30 float64 = 0.34 * 1
    var x33 string = "str" + "in" + "g"

    println(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x16)
    println(x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x33)
}
        "#,
        "true 10 53 -76 56 43 56 89 64 89 53 45 1.29e1 2.2022222222222223e0 string\nfalse 0 -45 1 56 -923824832 33 5 9 12 6 4 -3.66e0 3.4e-1 string\n",
    )
}

#[test]
fn test_var_short_decl() {
    compare_stderr_output(
        r#"
package main

func main() {
    x := 1
    println(x)
    {
        x := 2
        println(x)
    }
}
        "#,
        "1\n2\n",
    )
}

#[test]
fn test_var_local_shadowing() {
    compare_stderr_output(
        r#"
package main

var x0 bool
var x1 int8 = 45
var x2 int16 = 66
var x3 uint = 4

func main() {
    var x0 bool = true
    println(x0)
    println(x1)
    var x1 int8 = 54
    println(x1)
    println(x2)
    var x2 uint = 800 + x3
    println(x2)
    x3 := -9
    println(x3)
}
        "#,
        "true\n45\n54\n66\n804\n-9\n",
    )
}

#[test]
fn test_var_scope() {
    compare_stderr_output(
        r#"
package main

var b bool

func main() {
    var x int = 100
    y := 9;
    {
        println(b)
        b := true
        println(b)
        println(x)
        var x = 100
        x++
        println(x)
        println(y)
        y := "string"
        println(y)
    }

    println(x)
    println(y)
    println(b)
}
        "#,
        "false\ntrue\n100\n101\n9\nstring\n100\n9\nfalse\n",
    )
}

#[test]
fn test_var_scope_nested() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 100
    var y = 56;
    {
        {
            {}
        }

        {
            y++
            println(y)
            var y string = "hi";
            println(y)
            const s string = "heya"

            y = "hey"
            println(y)
            {
                println(s)
            }

        }

        {
            var z int = 999;
            {
                println(z)
                var z int;
                println(z);
                z += 1;
                {
                    z := 9
                    println(z)
                }
                println(z)
            }
            println(z)
        }
        var x int;
        println(x)
        println(y)
    }

    println(x)
    println(y)
}
        "#,
        "57\nhi\nhey\nheya\n999\n0\n9\n1\n999\n0\n57\n100\n57\n",
    )
}
