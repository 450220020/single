use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::any::Any;

#[warn(dead_code)]
type SingleType = (dyn Any + Send + Sync);

#[warn(dead_code)]
static INSTANCE: OnceCell<DashMap<String, OnceCell<Box<SingleType>>>> = OnceCell::new();

#[warn(dead_code)]
pub fn single_init<'a>()-> &'a DashMap<String, once_cell::sync::OnceCell<Box<(dyn Any + Sync + std::marker::Send + 'a)>>>{
    return INSTANCE.get_or_init(|| DashMap::new());
}
#[warn(dead_code)]
pub fn new_once<T>()->OnceCell<T>{
    OnceCell::new()
}


#[macro_export]
macro_rules! single_init {
    () => {
        $crate::com::single::single_init()
    };
}


#[macro_export]
macro_rules! single_for{
    ($a:expr,$e:expr,$b:ty) => {
        $a.get($e)
        .take()
        .unwrap_or_else(|| panic!("dashMap the key is null"))
                .get()
                .take()
                .unwrap_or_else(|| panic!("once the  loading null"))
                .downcast_ref::<$b>()
                .unwrap_or_else(|| panic!("type cast error")).clone()
    };
}


#[macro_export]
macro_rules! single_add{
    ($key:expr,$val:expr) => {
        $crate::com::single::single_init().entry($key).or_insert($crate::com::single::new_once()).get_or_init(||{
            $val
        });
    };
}



#[test]
fn test_single(){

    let sin = single_init!();
    single_add!("a".to_string(),Box::new("aaaa".to_string()));
    let straa = single_for!(sin,&"a".to_string(),String);
    println!("rustl:{:?}",straa);

}