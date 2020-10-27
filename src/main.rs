mod solutions;

use solutions::hard;
use solutions::middle;
use solutions::simple;
fn simple_test() {
    //    simple::two_sum::test();
    //    simple::sorted_squares::test();
    //    simple::is_palindrome::test();
    //    simple::is_power_of_four::test();
    //    simple::my_sqrt::test();
    //    simple::construct_rectangle::test();
}

fn middle_test() {
    //middle::length_of_longest_substring;
    //middle::three_sum::test();
    //middle::sort_colors::test();
    //middle::max_area_of_island::test();
}

fn hard_test() {
    //hard::num_dup_digits_at_most_n::test();
}

fn main() {
    simple_test();
    middle_test();
    hard_test();
}
