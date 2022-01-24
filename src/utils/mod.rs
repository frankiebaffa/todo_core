pub fn get_printable_coords(nums: &Vec<usize>) -> String {
    nums.into_iter().map(|num| {
        num.to_string()
    }).collect::<Vec<String>>().join(", ").to_string()
}
