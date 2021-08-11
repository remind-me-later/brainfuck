#[test]
fn simple_hello() {
    // Super simple hello world program should show any glaring errors
    let program = "
[
  A simple \"Hello, World\" program that prints a newline at the end,
  only the first cell is manipulated to obtain the desired ASCII values.

  A loop at the beginning of a program will never be executed as the value
  of the first cell is 0, so you can write a comment using any character you
  like as long as the '[' and ']' are balanced.
]

++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.   Add 72 which is ASCII for 'H' to Cell #0 and print it
+++++++++++++++++++++++++++++.                                              Add 30 to get to the value 101 for 'e'
+++++++.                                                                    Add 7 for 'l'
.                                                                           Print for another 'l'
+++.                                                                        Add 3 for 'o'
-------------------------------------------------------------------.        Subtract until 44 for comma
------------.                                                               The same to get to 32 for Space
+++++++++++++++++++++++++++++++++++++++++++++++++++++++.                    Get to 87 for 'W'
++++++++++++++++++++++++.                                                   111 for 'o'
+++.                                                                        114 for 'r'
------.                                                                     108 for 'l'
--------.                                                                   100 for 'd'
-------------------------------------------------------------------.        10  for '!'";

    let mut output_file = Vec::with_capacity(13);

    program
        .parse::<Instructions>()
        .unwrap()
        .execute(&mut output_file, &mut std::io::empty());

    let mut out = Vec::with_capacity(13);
    output_file.as_slice().read_to_end(&mut out).unwrap();

    assert_eq!(String::from_utf8(out).unwrap(), "Hello, World!");
}
