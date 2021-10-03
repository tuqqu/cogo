mod common;

use common::compare_stderr_output;

#[test]
fn test_switch() {
    compare_stderr_output(
        r#"
package main

func main() {
    // switch expr {}

    switch {
    }
    println(1)

    switch true {
    case false:
        println(2)
    case true:
        println(3)
    }

    switch {
    case false:
        println(4)
    case !true:
        println(5)
    case true:
        println(6)
    }

    switch true {
    case true:
        println(7)
    case false:
        println(8)
    case !true:
        println(9)
    }

    switch "hello" {
    case "hi":
        println(10)
    case "he" + "llo" + "hi":
        println(11)
    case "he" + "llo":
        println(12)
        break
        println(13)
    }

    var b = 900
    switch b - 100 {
    case 800:
        println(14)
        break
        println(15)
    case 700:
        println(16)
    case 800 + 1:
        println(17)
    default:
        println(18)
    }

    switch true {
    default:
        println(19)
    case true:
        println(20)
    case false:
        println(21)
    case !true:
        println(22)
    }

    switch 11 {
    case 12:
        println(20)
    case 13:
        println(21)
    default:
        println(22)
    case 14:
        println(23)
    }

    switch true {
    case true:
        println(24)
    case false:
        println(25)
    default:
        switch true {
        case true:
            println(26)
        case false:
            println(27)
        default:
            println(28)
        case !true:
            println(29)
        }
        println(30)
    case !true:
        println(31)
    }

    switch true {
    case false:
        println(31)
    case false:
        println(32)
    default:
        switch true {
        case true:
            println("inner")
            var x int
            for x = 0; x <= 8; x = x + 1 {
                if x == 7 {
                    break
                }

                if x == 5 {
                    continue
                }
                println(x)
            }
            fallthrough
        case false:
            println(34)
            break
            println("after")
        default:
            println(35)
        case !true:
            println(36)
        }
        println(37)
    case !true:
        println(38)
    }
}
"#,
        r#"1
3
6
7
12
14
20
22
24
inner
0
1
2
3
4
6
34
37
"#,
    )
}
