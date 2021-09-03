mod common;

use common::compare_stderr_output;

#[test]
fn test_array_default() {
    compare_stderr_output(
        r#"
package main

var q [2]int
var y [3][2]int8

func main() {

    println(q, q[1], q[0])

    var w [3]bool
    println(w, w[1], w[0])

    var r [4]int8
    println(r)

    println(y, y[0], y[0][1])

    var x [1][1][2]float64
    println(x, x[0], x[0][0], x[0][0][0], x[0][0][1])
}
"#,
        r#"<[2]int>[0 0] 0 0
<[3]bool>[false false false] false false
<[4]int8>[0 0 0 0]
<[3][2]int8>[<[2]int8>[0 0] <[2]int8>[0 0] <[2]int8>[0 0]] <[2]int8>[0 0] 0
<[1][1][2]float64>[<[1][2]float64>[<[2]float64>[0e0 0e0]]] <[1][2]float64>[<[2]float64>[0e0 0e0]] <[2]float64>[0e0 0e0] 0e0 0e0
"#,
    )
}

#[test]
fn test_array_var() {
    compare_stderr_output(
        r#"
package main

var q [2]int = [2]int{1, 2}
var w [3]bool = [3]bool{true, false, !false}

func main() {

    println(q, q[1], q[0])
    println(w, w[1], w[0])

    var e = [2]string{"string", "another " + "string"}
    println(e, e[0], e[1])

    rr := 1
    r := [4]int8{1, int8(rr) + 2, 3 - 4, int8(4 * rr)}
    println(r)

    var y [3][2]int = [3][2]int{
        [2]int{1, 2},
        [2]int{3, rr},
        q,
    }
    println(y, y[0], y[0][1])

    var x [1][1][2]string = [1][1][2]string{
        [1][2]string{e},
    }
    println(x, x[0], x[0][0], x[0][0][0], x[0][0][1])
}
"#,
        r#"<[2]int>[1 2] 2 1
<[3]bool>[true false true] false true
<[2]string>[string another string] string another string
<[4]int8>[1 3 -1 4]
<[3][2]int>[<[2]int>[1 2] <[2]int>[3 1] <[2]int>[1 2]] <[2]int>[1 2] 2
<[1][1][2]string>[<[1][2]string>[<[2]string>[string another string]]] <[1][2]string>[<[2]string>[string another string]] <[2]string>[string another string] string another string
"#,
    )
}

#[test]
fn test_array_mutate() {
    compare_stderr_output(
        r#"
package main

var q [2]int = [2]int{1, 2}
var w [3]bool = [3]bool{true, false, !false}

func main() {
    q[0] = 5
    q[0] = q[0] - 1
    q[0]--
    println(q, q[1], q[0])

    w[0] = !true
    w[2] = w[0]
    println(w, w[1], w[0])

    var e = [2]string{"string", "another " + "string"}
    e[0] += "!!"
    println(e, e[0], e[1])

    var rr int64 = 1
    r := [4]int8{1, int8(rr) + 2, 3 - 4, 4 * int8(rr)}
    r[0] += r[0]
    r[1] += r[2]
    println(r)

    var y [3][2]int = [3][2]int{
        [2]int{1, 2},
        [2]int{3, int(rr)},
        q,
    }
    var h = y
    var j = y[0]
    h[0][1] = 56
    j[1] = 77
    y[0][1] = 88
    println(y, y[0], y[0][1])
    println(h, h[0])
    println(j)
}
"#,
        r#"<[2]int>[3 2] 2 3
<[3]bool>[false false false] false false
<[2]string>[string!! another string] string!! another string
<[4]int8>[2 2 -1 4]
<[3][2]int>[<[2]int>[1 88] <[2]int>[3 1] <[2]int>[3 2]] <[2]int>[1 88] 88
<[3][2]int>[<[2]int>[1 88] <[2]int>[3 1] <[2]int>[3 2]] <[2]int>[1 88]
<[2]int>[1 88]
"#,
    )
}

#[test]
fn test_array_param() {
    compare_stderr_output(
        r#"
package main

var q [4]int = [4]int{1, 2, 3, 4}

func main() {
    var has_4 = in(q, 4)
    println(has_4)

    var has_6 = in(q, 6)
    println(has_6)
}

func in(array [4]int, x int) bool {
    for i := 0; i < 4; i++ {
        if array[i] == x {
            return true
        }
    }

    return false
}
"#,
        r#"true
false
"#,
    )
}

#[test]
fn test_array_return() {
    compare_stderr_output(
        r#"
package main

func main() {
    initial := [4]int{2, 3, 4, 5}

    var by_3 = multiply(initial, 3)
    println(by_3)

    var by_m10 = multiply(initial, -10)
    println(by_m10)
}

func multiply(array [4]int, by int) [4]int {
    var multiplied [4] int
    for i := 0; i < 4; i++ {
        multiplied[i] = array[i] * by
    }

    return multiplied
}
"#,
        r#"<[4]int>[6 9 12 15]
<[4]int>[-20 -30 -40 -50]
"#,
    )
}
