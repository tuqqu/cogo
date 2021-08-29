mod common;

use common::compare_stderr_output;

#[test]
fn test_const_decl() {
    compare_stderr_output(
        r#"
package main

const x0 bool = true
const x1 int8 = 10
const x2 int16 = 56 - 3
const x3 int32 = -31 - 45
const x4 int64 = 56
const x5 int = 43
const x6 uint8 = 56
const x7 uint16 = 89
const x8 uint32 = 8 * 8
const x9 uint64 = 90 - 1
const x10 uint = 45 - 0 + 8 / 1
const x11 uintptr = 45
const x12 float32 = 9.9 + 3
const x13 float64 = 99.1 / 45
const x16 string = "strin" + "g"

func main() {
    const x17 bool = false
    const x18 int8 = 9 - 9
    const x19 int16 = -45
    const x20 int32 = 1
    const x21 int64 = 8 * 7
    const x22 int = -923824832
    const x23 uint8 = +33
    const x24 uint16 = 5
    const x25 uint32 = 45 / 5
    const x26 uint64 = 12
    const x27 uint = 6
    const x28 uintptr = 4
    const x29 float32 = 0.34 - 4
    const x30 float64 = 0.34 * 1
    const x33 string = "str" + "in" + "g"

    println(x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x16)
    println(x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x33)
}
        "#,
        "true 10 53 -76 56 43 56 89 64 89 53 45 1.29e1 2.2022222222222223e0 string\nfalse 0 -45 1 56 -923824832 33 5 9 12 6 4 -3.66e0 3.4e-1 string\n",
    )
}

#[test]
fn test_const_local_shadowing() {
    compare_stderr_output(
        r#"
package main

const x0 bool = false
const x1 int8 = 45
const x2 int16 = 66
const x3 uint = 4

func main() {
    const x0 bool = true
    println(x0)
    println(x1)
    const x1 int8 = 54
    println(x1)
    println(x2)
    const x2 uint = 800 + x3
    println(x2)
    x3 := -9
    println(x3)
}
        "#,
        "true\n45\n54\n66\n804\n-9\n",
    )
}

#[test]
fn test_const_scope() {
    compare_stderr_output(
        r#"
package main

const b bool = false

func main() {
    const x int = 100
    const y = 9;
    {
        println(b)
        b := true
        println(b)
        println(x)
        const x = 101
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
