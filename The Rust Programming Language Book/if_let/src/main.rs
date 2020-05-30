fn main() {
   let some_u8 = Some(2u8);
   if let Some(3) = some_u8 {
       println!("Three");
   } else {
       println!("Other inputs");
   }
}
