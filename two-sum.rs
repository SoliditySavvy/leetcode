impl Solution 
{
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
    {
        // [1] prepare numbers for binary search by 
        //     saving original indices and sorting
        let mut nums: Vec<(usize, i32)> = 
            nums.into_iter()
                .enumerate()
                .collect::<Vec<(usize, i32)>>();

        nums.sort_unstable_by_key(|&(_, n)| n);
        
        // [2] we perform linear scan for the first number
        //     and do binary search for the second number
        for (k, (i, ni)) in nums.iter().enumerate()
        {
            // [3] to prevent duplication, slices start from k+1
            match nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj)
            {
                Ok(jj) => return vec![*i as i32, nums[jj+k+1].0 as i32],
                Err(_) => {}
            }
        }

        unreachable!("Error: this place should be unreachable");
        return vec![0,0];
    }
}
