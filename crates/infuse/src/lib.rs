#![feature(const_type_id)]
#![feature(const_trait_impl)]

pub mod service;

use std::any::TypeId;

pub use infuse_macros::*;


const fn count_unique_type_ids<const LEN: usize>(numbers: &[TypeId; LEN]) -> usize {
    let mut unique_values: [Option<TypeId>; LEN] = [None; LEN]; 
    let mut unique_count = 0;
    let mut count = 0;
    'nominate: loop {
        if count < LEN {
            let mut inner_count = 0;
            'detect: loop {
                if inner_count < LEN {
                    if let Some(value)  = unique_values[inner_count] {
                        if value.eq(&numbers[count]) {
                            break 'detect;
                        }
                    }
                    inner_count+=1;
                } else {
                    unique_values[unique_count] = Some(numbers[count]); 
                    unique_count+=1;
                    break 'detect;
                }
            }
            count+=1;
        } else {
            break 'nominate;
        }
    }
    unique_count
}

const fn unique_type_ids<const LEN: usize, const FINAL_LEN: usize>(numbers: &[TypeId; LEN]) -> [TypeId; FINAL_LEN] {
    let mut unique_values = [None; LEN]; 
    let mut unique_count = 0;
    let mut count = 0;
    'nominate: loop {
        if count < LEN {
            let mut inner_count = 0;
            'detect: loop {
                if inner_count < LEN {
                    if let Some(value)  = unique_values[inner_count] {
                        if value == numbers[count] {
                            break 'detect;
                        }
                    }
                    inner_count+=1;
                } else {
                    unique_values[unique_count] = Some(numbers[count]); 
                    unique_count+=1;
                    break 'detect;
                }
            }
            count+=1;
        } else {
            break 'nominate;
        }
    }
    let mut data = [TypeId::of::<u8>(); FINAL_LEN];

    let mut count = 0;
    loop {
        if count < FINAL_LEN {
            if let Some(value) = unique_values[count] {
                data[count] = value;
            }
            count+=1;
        } else {
            break;
        }
    }

    data
}