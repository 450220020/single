use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::{any::Any, collections::HashMap, slice::SliceIndex};

type LoaderType = (bool, String, Vec<String>, String);

#[warn(dead_code)]
static LOADER: OnceCell<DashMap<String, LoaderType>> = OnceCell::new();


#[warn(dead_code)]
pub fn loader_init<'a>() -> &'a DashMap<String, LoaderType> {
    return LOADER.get_or_init(|| DashMap::new());
}

#[macro_export]
macro_rules! get_loader {
    () => {
        $crate::com::single_loader::loader_init()
    };
}

#[derive(Debug, Clone)]
pub struct Assembly {}


///装载排序
pub fn get_assembly_sort() ->Option<Vec<LoaderType>>{
    let loading_max_size = get_loader!().len() + 1;
    let mut loding_idx = 0;
    let mut ruslt_loading_vec = vec![];
    let mut is_loding_true = HashMap::new();
    let mut is_loding_false = HashMap::new();
    loop {
        loding_idx += 1;
        if loding_idx >= loading_max_size {
            break;
        }
        is_loding_false = HashMap::new();
        is_loding_true = HashMap::new();
        for ent in get_loader!() {
            if ent.0 {
                continue;
            }
            let relation_vec = &ent.2;
            if relation_vec.len() > 0 {
                let mut is_loading_ok = true;
                for rel_str in relation_vec {
                    match get_loader!().get(rel_str) {
                        Some(r) => {
                            if !r.0 {
                                is_loading_ok = false;
                            }
                        }
                        None => {
                            is_loading_ok = false;
                        }
                    }
                }
                if !is_loading_ok {
                    is_loding_false.insert(ent.key(), ent.clone());
                    continue;
                }
            }
            is_loding_true.insert(ent.key(), ent.clone());
        }
        if is_loding_true.keys().len() < 1 {
            break;
        }
        for loding_str in is_loding_true {
            let mut moda = loding_str.1.clone();
            moda.0 = true;
            get_loader!().insert(loding_str.0.to_string(), moda.clone());
            ruslt_loading_vec.push(moda.clone());
        }
    }
    if is_loding_false.len() > 0 {
        return None;
    } else {
        return Some(ruslt_loading_vec);
    }
}