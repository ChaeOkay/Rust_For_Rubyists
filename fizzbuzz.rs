fn div_by_three(num: int) -> bool {
  num % 3 == 0
}

fn div_by_five(num: int) -> bool {
  num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
  num % 15 == 0
}

fn main() {
  for num in range(1i, 101) {
    println!("{:s}",
            if div_by_fifteen(num) { "FizzBuzz".to_string() }
            else if div_by_three(num) { "Fizz".to_string() }
            else if div_by_five(num) { "Buzz".to_string() }
            else { num.to_string() }
        );
  }
}


#[test]
fn test_div_by_five_with_fifteen() {
  if !div_by_fifteen(15) {
    fail!("Fifteen should be here")
  }
}

#[test]
fn test_div_by_fifteen() {
  if div_by_fifteen(1) {
    fail!("One is not fifteen!")
  }
}

#[test]
fn test_div_by_five_with_five() {
  if !div_by_five(5) {
    fail!("Five should be there")
  }
}

#[test]
fn test_div_by_five() {
  if div_by_five(1) {
    fail!("One is not three!")
  }
}

#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    fail!("One is not three!")
  }
}


#[test]
fn test_div_three_with_three() {
  if !div_by_three(3) {
    fail!("Three should be there")
  }
}
