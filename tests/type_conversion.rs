mod common;

use common::compare_stderr_output;

#[test]
fn test_const_decl() {
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
    var integer int = int(x1) + 10 + int(x2) + int(x3) + int(int8(x5))
    println(integer)

    var unsigned_integer uint64 = uint64(float64(x12) + float64(x1) + float64(float32(float64(int(x1)))))
    println(unsigned_integer)
}
        "#,
        "40\n32\n",
    )
}
