fn main() {
   let mut total: u64 = 0; 
   const TARGET_PERFECT_SQUARES_COUNT: u64 = 266000;
   for i in 1..=TARGET_PERFECT_SQUARES_COUNT {
       if i % 2 == 1 {
           total += i.pow(2);
       }
   }

   println!("The sum of the odd squares of the first {TARGET_PERFECT_SQUARES_COUNT} perfect squares is {total}");
}
