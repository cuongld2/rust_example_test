use std::panic;
use rstest::*;

fn setup(){

    println!("setup 111");
}

fn teardown(){
    println!("teardown 2222");
}

fn run_test<T>(test: T) -> ()
    where T: FnOnce() -> () + panic::UnwindSafe
{
    setup();    
    let result = panic::catch_unwind(|| {
        test()
    });    
    teardown();    
    assert!(result.is_ok())
}

fn function_under_test(){

    assert_eq!(1,3);
}


#[rstest]
fn example_setup_teardown() {
    run_test(|| {
        let ret_value = function_under_test();
        assert!(false);
    })
}