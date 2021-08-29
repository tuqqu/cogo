mod common;

use common::compare_stderr_output;

#[test]
fn test_for() {
    compare_stderr_output(
        r#"
package main

func main() {
    // for {}

    var x1 int;
    for {
        x1++
        if x1 == 10 {
            break;
        }
    }
    println(x1)

    var x2 int;
    for {
        x2++
        println(x2)
        if x2 == 2 {
            x2 += 10
            continue
        }

        if x2 > 10 {
            break
        }
    }
    println(x2)

    // for expr {}
    var b = true
    var x3 int
    for b == (true || false) {
        x3 ++
        if x3 == 10 {
            break;
        }
    }
    println(x3)

    var x4 int
    for b != (true || false) {
        x4 += 1
        if x4 == 10 {
            break;
        }
    }
    println(x4)

    var x5 int;
    for b == (true || false) {
        x5 = x5 + 1
        println(x5)
        if x5 == 2 {
            x5 = x5 + 10
            continue
        }

        if x5 > 10 {
            break
        }
    }
    println(x5)

    var b1 = true
    for b1 {
        b1 = false
    }
    println(b1)

    // for expr; expr; expr {}
    var x6 int
    for x6 = 1; x6 <= 10; x6 += 2 {
        if x6 >= 5 {
            x6 = x6 + 1
            break
        }
    }
    println(x6)

    var x7 uint
    for x7 = 1; x7 <= 40; x7 += 2 {
        x7 = x7 + 1
        println(x7)
        if x7 == 2 {
            x7 = x7 + 10
            continue
        }

        if x7 > 30 {
            break
        }
    }
    println(x7)

    // for ; expr; expr {}
    var x8 int64 = 1
    for ; x8 <= 10; x8 += 2 {
        if x8 >= 5 {
            x8 = x8 + 1
            break
        }
    }
    println(x8)

    // for expr; ; expr {}
    var x9 int8
    for x9 = 1; ; x9 += 2 {
        if x9 >= 5 {
            x9 = x9 + 3
            break
        }
    }
    println(x9)

    // for expr; expr; {}
    var x10 int
    for x10 = 1; x10 <= 10; {
        if x10 >= 5 {
            x10 = x10 + 1
            break
        }
        x10 = x10 + 2
    }
    println(x10)

    // for ; ; expr {}
    var x11 int
    x11 = 1
    for ; ; x11 = x11 + 2 {
        if x11 >= 5 {
            x11 = x11 + 1
            break
        }

        if x11 >= 10 {
            break
        }
    }
    println(x11)

    // for ; expr; {}
    var x12 int
    x12 = 1
    for ; x12 <= 10;  {
        if x12 >= 5 {
            x12 = x12 + 1
            break
        }

        x12 = x12 + 2
    }
    println(x12)

    // for ; ; {}
    var x13 int
    x13 = 1
    for ; ;  {
        if x13 >= 5 {
            x13++
            break
        }

        x13 = x13 + 2
    }
    println(x13)

    // nested
    println("nested")
    for x14 := 0; x14 < 10; x14++ {
        for x := 0; x < 10; x++ {
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

        println(x14)
    }
}
"#,
        r#"10
1
2
13
13
10
0
1
2
13
13
false
6
2
15
18
21
24
27
30
33
33
6
8
6
6
6
6
nested
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
9
"#,
    )
}
