#![feature(const_type_id)]

#[macro_use]
extern crate infuse;

#[derive(Controller)]
#[controller(path = "/pet/:id")]
pub struct PetController {
    pet_service: PetService,
    pet_service1: PetService,
    pet_service2: PetService,
    pet_service3: PetService,
}

#[derive(Service)]
pub struct PetService {}

pub fn main() {
    println!("{:?}", PetController::dependencies)
}
