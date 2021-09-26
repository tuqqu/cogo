mod common;

use common::compare_stderr_output;

#[test]
fn test_func_variadic_1() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 45
    var res_v1_1 = v1(1)
    println(res_v1_1)

    var res_v1_2 = v1(1, x)
    println(res_v1_2)

    var res_v1_3 = v1(1, 2, x, 4, 5)
    println(res_v1_3)

    var res_v1_4 = v1()
    println(res_v1_4)

    var s string = "good day"
    var res_v2_1 = v2("hi")
    println(res_v2_1)

    var res_v2_2 = v2(s)
    println(res_v2_2)

    var res_v2_3 = v2("hi", "hello", s)
    println(res_v2_3)

    var res_v2_4 = v2()
    println(res_v2_4)

    var ss1 []string = []string{"hi"}
    var ss2 []string = []string{"bye", "farewell"}
    v3([]string{"hi"})
    v3(ss2, []string{"hi"})
    v3([]string{"hi"}, []string{"hello"}, []string{"good day"}, ss1, ss2)
}

func v1(x ...int) int {
    println(x)
    return len(x) - 1
}

func v2(x ...string) bool {
    println(x)
    return len(x) == 0
}

func v3(x ...[]string) {
    println(x)
}
        "#,
        "<int>[1]
0
<int>[1 45]
1
<int>[1 2 45 4 5]
4
<int>[]
-1
<string>[hi]
false
<string>[good day]
false
<string>[hi hello good day]
false
<string>[]
true
<[]string>[<[]string>[hi]]
<[]string>[<[]string>[bye farewell] <[]string>[hi]]
<[]string>[<[]string>[hi] <[]string>[hello] <[]string>[good day] <[]string>[hi] <[]string>[bye farewell]]
",
    )
}

#[test]
fn test_func_variadic_with_other_params() {
    compare_stderr_output(
        r#"
package main

func main() {
    var x int = 45
    v1([]int{1,2}, 1)
    v1([]int{1,2}, 1, x)
    v1([]int{1,2}, 1, 2, x, 4, 5)

    var s string = "good day"
    v2("first", "hi")
    v2("first", s)
    v2("first", "hi", "hello", s)

    var ss1 []string = []string{"hi"}
    var ss2 []string = []string{"bye", "farewell"}
    var res1 = v3(true, []string{"hi"})
    println(res1)

    var res2 = v3(true, ss2, []string{"hi"})
    println(res2)

    var res3 = v3(false, []string{"hi"}, []string{"hello", "good day"}, ss1, ss2)
    println(res3)
}

func v1(f []int, x ...int) {
    println(f)
    println(x)
}

func v2(f string, x ...string) {
    println(f)
    println(x)
}

func v3(f bool, x ...[]string) int {
    println(f)
    println(x)

    return len(x)
}
        "#,
        "<[]int>[1 2]
<int>[1]
<[]int>[1 2]
<int>[1 45]
<[]int>[1 2]
<int>[1 2 45 4 5]
first
<string>[hi]
first
<string>[good day]
first
<string>[hi hello good day]
true
<[]string>[<[]string>[hi]]
1
true
<[]string>[<[]string>[bye farewell] <[]string>[hi]]
2
false
<[]string>[<[]string>[hi] <[]string>[hello good day] <[]string>[hi] <[]string>[bye farewell]]
4
",
    )
}
