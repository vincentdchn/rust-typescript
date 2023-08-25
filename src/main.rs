fn multiply(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}

fn main() {}
