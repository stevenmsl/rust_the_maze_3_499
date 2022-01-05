use rust_the_maze_3_499::Solution;
fn main() {
    println!(
        "{:?}",
        Solution::shortest_distance(&(4, 3), &(0, 1), &Solution::test_fixture_1())
    );
}
