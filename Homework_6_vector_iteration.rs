fn running_sum(nums: Vec<i32>) -> Vec<i32> {  
    let mut v = nums;
    for i in 0..v.len() - 1 {
       v[i+1] += v[i];
    }
    v
}  
  
fn main() {  
    println!("{:?}", running_sum(vec![1, 1, 1, 1, 1]));
}a