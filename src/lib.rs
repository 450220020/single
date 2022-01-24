extern crate dashmap;
extern crate once_cell;
pub mod com;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


#[test]
fn test_one(){

    let sin = single_init!();
    single_add!("a",Box::new("aaaa".to_string()));
    let straa = single_for!(sin,"a",String);
    println!("rustl:{:?}",straa);

}