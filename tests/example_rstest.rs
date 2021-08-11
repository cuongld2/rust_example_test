use rstest::*;
use std::sync::Once; 


static INIT: Once = Once::new();
#[fixture]
pub fn setup() -> () { 
    INIT.call_once(|| {
        println!("Hello, world!");
        // initialization code here
    });
}


#[fixture]
pub fn fixture() -> u32 { 42 }

#[rstest]
fn should_success(fixture: u32,setup: ()) {
    assert_eq!(fixture, 42);
}

#[rstest]
fn should_fail(fixture: u32,setup: ()) {
    assert_ne!(fixture, 42);
}