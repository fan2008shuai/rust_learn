
fn main() {
    let u1 = foo();
    let u2 = 56u8;

    println!("u8 add is : {}", (u1 + u2));
}

fn foo() -> u8 {
    200u8
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max = nums[0];
    let mut sum = 0;
    for num in &nums {
        if sum < 0 {
            sum = *num;
        } else {
            sum += num;
        }

        if sum > max {
            max = sum;
        }
    }
    max
}