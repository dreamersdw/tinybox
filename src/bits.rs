pub fn bits(nums: &Vec<u64>) {
    for n in nums {
        for i in 0..63 {
            let b = n >> i & 1 as u64;
            print!("{}", b);
            if (i + 1) % 8 == 0 {
                print!(" ");
            }
        }
        println!(" {}", n);
    }
}
