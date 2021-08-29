mod common;

use common::compare_stderr_output;

#[test]
fn test_func_void() {
    compare_stderr_output(
        r#"
package main

func main() {
    void()
    void2(1, "hi")

    var x int = 3
    var y string = "hello"

    void2(x, y)
    void3(y, x)

    println(x)
    println(y)
}

func void() {
}

func void2(v int, s string) {
}

func void3(s string, x int) {
    println(x)
    println(s)
    x = 99
    println(x)
    s = s + "hi"
    println(s)
}
        "#,
        "3
hello
99
hellohi
3
hello
",
    )
}

#[test]
fn test_func_return() {
    compare_stderr_output(
        r#"
package main

func main() {
    var t = 1
    f1(t)

    println(t)

    println(f2(true))
    println(f2(!true))

    var str = f3("hi!")
    println(str)

    println(f4())

    var y3 = 9;
    println(y3)
}

func f1(x int) {
    var y = x
    y = y + 1
    println(y)
}

func f2(b bool) int {
    if b {
        return 10
    } else {
        return -10
    }
}

func f3(s string) string {
    return s + s
}

func f4() int {
    var x14 int
    for x14 = 0; x14 < 10; x14 = x14 + 1 {
        var x int
        for x = 0; x < 10; x = x + 1 {
            if x == 4 {
                continue
            }
            if x == 5 {
                break
            }
            println(x)
        }

        if x14 == 8 {
            continue
        }

        if x14 == 9 {
            return x14 + 100
        }

        println(x14)
    }
    return 89
}
        "#,
        "2
1
10
-10
hi!hi!
0
1
2
3
0
0
1
2
3
1
0
1
2
3
2
0
1
2
3
3
0
1
2
3
4
0
1
2
3
5
0
1
2
3
6
0
1
2
3
7
0
1
2
3
0
1
2
3
109
9
",
    )
}
