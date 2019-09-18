mod solutions;

use solutions::simple;
use solutions::middle;
fn simple_test() {
    simple::twoSum::test();
    simple::sorted_squares::test();
    simple::is_palindrome::test();
    simple::is_power_of_four::test();
    simple::my_sqrt::test();
}

fn middle_test() {
    //middle::length_of_longest_substring;
}

fn hard_test() {

}


fn main() {
    simple_test();
    middle_test();
    hard_test();
}
