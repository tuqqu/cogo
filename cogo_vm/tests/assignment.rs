mod common;

use common::compare_stderr_output;

#[test]
fn test_assignment() {
    compare_stderr_output(
        r#"
package main

var (
    a [10]int
    b [2]string = [...]string{"string1", "string2"}
    c [2][2]uint8
    d int
    e string = "hi"
)

func main() {
    var (
        f string
        g [1][1]int64
        h byte
        i []int
   )

   i = append(i, 10)

   a[1], b, c[d][inc(d)], d, e, f, g[0], h, i[0] = 9, [...]string{"another1", "another2"}, 9 + 9, inc(11), "bye", "hel" + "lo", [...]int64{int64(d + 100)}, 1, 2
   println(a,b,c,d,e,f,g,h,i)
}

func inc(x int) int {
    return x + 1
}
        "#,
        "<[10]int>[0 9 0 0 0 0 0 0 0 0] <[2]string>[another1 another2] <[2][2]uint8>[<[2]uint8>[0 18] <[2]uint8>[0 0]] 12 bye hello <[1][1]int64>[<[1]int64>[100]] 1 <[]int>[2]\n",
    )
}
