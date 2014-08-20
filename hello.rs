fn say_nine() -> int {
  9
}

#[test]
fn test_say_nine() {
  let x = say_nine();
  assert!(9 == x)
}
