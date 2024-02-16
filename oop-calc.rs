struct CalcNumber {
  value: u32,
}

impl CalcNumber {
  fn new(value: u32) -> CalcNumber {
    return CalcNumber { value }
  }

  fn sum (&self, value_to_sum: u32) -> u32 {
    return self.value + value_to_sum
  }
  
  fn subtract (&self, value_to_subtract: u32) -> u32 {
    return self.value - value_to_subtract
  }
  
  fn multiply (&self, value_to_multiply: u32) -> u32 {
    return self.value * value_to_multiply
  }
  
  fn divide (&self, value_to_divide: u32) -> u32 {
    return self.value / value_to_divide
  }
}

fn main() {
  println!("OOP Calculator\n");
  
  let numbers_to_calculate = [10, 20, 30, 40, 50, 60];
  
  for number in numbers_to_calculate.iter() {
    let calculable_number = CalcNumber::new(*number);

    let algo = 2;

    println!("{}\n", calculable_number.value);

    println!("{} + {} = {}", calculable_number.value, algo, calculable_number.sum(algo));
    println!("{} - {} = {}", calculable_number.value, algo, calculable_number.subtract(algo));
    println!("{} * {} = {}", calculable_number.value, algo, calculable_number.multiply(algo));
    println!("{} / {} = {}", calculable_number.value, algo, calculable_number.divide(algo));
    
    println!("\n");
  }
}
