// We need to import public methods from our crate
use testing::my_public_method;

#[test]
fn test_init() {
  assert!(my_public_method());
}
