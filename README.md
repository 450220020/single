# single
rust single 



#[macro_use]
extern crate single;

#[test]
fn test_one(){

    let sin = single_init!();
    single_add!("a",Box::new("aaaa".to_string()));
    let straa = single_for!(sin,&"a".to_string(),String);
    println!("rustl:{:?}",straa);

}