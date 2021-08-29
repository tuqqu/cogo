mod common;

use common::compare_stderr_output;

#[test]
fn test_if() {
    compare_stderr_output(
        r#"
package main

func main() {
    if true {
        println(1)
    }

    if false {
        println(2)
    }

    if !false {
        println(3)
    }

    var x bool = true;
    if x {
        println(4)
    }

    if x == true {
        println(5)
    }

    if 5 - 2 == 2 + 1 {
        println(6)
    }

    if 5 > 2 * 6 {
        println(7)
    }

    var y int = 100
    if 5 * y > 2 * y / 1 {
        println(8)
    }

    z1 := "a"
    if true {
        z1 := "b"
        z1 = "c"
        println(z1)
    }
    println(z1)

    z2 := "a"
    if true {
        z2 = "c"
        z2 := "b"
        println(z2)
    }
    println(z2)

    var z3 string = "a"
    if false {
        z3 = "c"
        var z3 string = "b"
        println(z3)
    }
    println(z3)
}
"#,
        r#"1
3
4
5
6
8
c
a
b
c
a
"#,
    )
}

#[test]
fn test_if_else() {
    compare_stderr_output(
        r#"
package main

func main() {
    x := true
    if x {
       println(1)
    } else if !x {
        println(2)
    }

    if !x {
       println(3)
    } else if false {
        println(4)
    }

    if !x {
       println(5)
    } else if false {
        println(6)
    } else {
        println(7)
    }

    if !x {
       println(8)
    } else if false {
        println(9)
    } else {
        println(10)
    }

    if false {
       println(11)
    } else if false {
        println(12)
    } else if !true == !true {
        println(13)
    } else {
        println(14)
    }

    z := "h"
    if !x {
       z = "z"
       println(15)
    } else {
        z = "t"
        println(16)
    }
    println(z)
}
"#,
        r#"1
7
10
13
16
t
"#,
    )
}

#[test]
fn test_if_nested() {
    compare_stderr_output(
        r#"
package main

func main() {
    if true {
        if true {
            println(1)
        } else {
            println(2)
        }
    } else {
        if false {
            println(3)
        } else {
            println(4)
        }
    }

    var n1 string = "a"
    if false {
        var n1 string = "b"
        println(n1)
        if true {
            n1 = "dd"
            var n1 string = "c"
            println(n1)
        } else {
            n1 = "cc"
            var n1 string = "d"
            println(n1)
        }
    } else {
        if false {
            n1 = "bb"
            var n1 string = "f"
            println(n1)
        } else {
            n1 = "aa"
            var n1 string = "g"
            println(n1)
        }
        n1 = "kk"
        var n1 string = "e"
        println(n1)
    }
    println(n1)
}
"#,
        r#"1
g
e
kk
"#,
    )
}
