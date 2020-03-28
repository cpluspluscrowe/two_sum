fn main() {
    let input_array = vec![2, 7, 11, 15];
    let target = 9;
    let answer = two_sum(input_array, target);
    println!("{:?}", answer);
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    for index1 in 0..nums.len(){
        for index2 in index1..nums.len(){
            let to_add1 = nums[index1];
            let to_add2 = nums[index2];
            if to_add1 + to_add2 == target {
                return vec![index1, index2]
            }
        }
    }
    // empty vector to communicate that no two pairs exist that equal the sum
    return vec![];
}

