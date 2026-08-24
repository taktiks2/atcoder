use proconio::input;

fn is_long(nums: Vec<i32>) -> bool {
    (nums[0] - nums[1]).abs() == 2 || (nums[0] - nums[1]).abs() == 3
}

fn main() {
    input! {
        s: String,
        t: String,
    };

    let s_num: Vec<i32> = s.chars().map(|c| (c as i32)).collect();
    let t_num: Vec<i32> = t.chars().map(|c| (c as i32)).collect();

    if is_long(s_num) == is_long(t_num) {
        println!("Yes")
    } else {
        println!("No")
    }
}
