pub fn is_armstrong_number(num: u32) -> bool {
    let exp: u32 = (num as f32).log10().floor() as u32 + 1;
    let mut num_copy: u32 = num;
    let mut sum_: u32 = 0;

    while num_copy != 0 {
        sum_ += (num_copy%10).pow(exp);
        num_copy /= 10
    }
    sum_ == num
}
