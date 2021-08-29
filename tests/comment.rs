mod common;

use common::compare_stderr_output;

#[test]
fn test_comment() {
    compare_stderr_output(
        r#"
package main

// Doc comment
func main() {
    var x = 10 //inline comment
    var y /* inline comment */ = 20
    /*
    comment
    */
    var z = 30
    println(x, y, z)
}
        "#,
        "10 20 30\n",
    )
}
