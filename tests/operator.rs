mod common;

use common::compare_stderr_output;

#[test]
fn test_operator_math() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 600
    var y int = 999;
    const R = 9

    x++
    y--
    x = x + y
    x = x + 4
    x = x - 998
    x = x / 2
    x = x % 3
    x = x * 6
    x = x * R

    println(x, y)

    a := "a"
    b := "b"
    s := a + b + "c"
    println(s)
}
        "#,
        "108 998\nabc\n",
    )
}

#[test]
fn test_operator_compound() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 600
    var y int = 999;
    const R = 9

    x++
    y--
    x += y
    x += 4
    x -= 998
    x /= 2
    x %= 3
    x *= 6
    x *= R

    println(x, y)

    a := "a"
    b := "b"
    a += b + "c"
    println(a)
}
        "#,
        "108 998\nabc\n",
    )
}

#[test]
fn test_operator_logic() {
    compare_stderr_output(
        r#"
package main

func main() {
    println(true || false)
    println(false || false)

    println(true && false)
    println(true && false == false)

    var b bool = true

    println((b || 5 > 6) && true && b)
    println(5 == 5 && 3 <= 100 * 5 || false || (5 == 45 / (7 - 5)) || !b)
    println(false || 45 != 45 || b == b)
    println(false && 45 != 45 && b == b)
}
        "#,
        "true\nfalse\nfalse\ntrue\ntrue\ntrue\ntrue\nfalse\n",
    )
}

#[test]
fn test_operator_comparison() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 100;
    var y int = 45;
    var b bool = true
    var c bool = false

    println(true == true)
    println(true == false)
    println(true != false)
    println(!true)
    println(!false)

    println(b == true)
    println(b == false)
    println(b != false)
    println(b != b)
    println(!b)
    println(!b == true)
    println(!b == false)
    println(!b != false)
    println(!b != !b)

    println(b == c)
    println(b == c)
    println(b != c)
    println(b != b == c == true)
    println(b != !b == !c == !true)

    println(5 > 1)
    println(5 < 1)
    println(5 >= 6)
    println(5 <= 6)

    println(x > 1)
    println(x < 1)
    println(x >= 6)
    println(6 >= x)
    println(x >= x)
    println(x <= 6)
    println(6 <= x)
    println(x <= x)

    println(x > y)
    println(x < y)
    println(x >= y)
    println(x > y - 1)
    println(x < y - 5)
    println(x <= y)

    println(56 - 6 >= 40)
    println(56 * 6 > -100)
    println(45 - 5 == 40 + 0 + 0 * 100)
    println(45 + 5 * 1 != 40 + 0 + 0 * 100)
}
        "#,
        "true
false
true
false
true
true
false
true
false
false
false
true
false
false
false
false
true
true
false
true
false
false
true
true
false
true
false
true
false
true
true
true
false
true
true
false
false
true
true
true
true
",
    )
}
