extern crate eliza;

use eliza::Eliza;

#[test]
fn load_ok(){
    assert!(Eliza::new("scripts/rogerian_psychiatrist.json").is_ok());
}

#[test]
fn load_err(){
    assert!(Eliza::new("scripts/not_a_script.json").is_err());
}
