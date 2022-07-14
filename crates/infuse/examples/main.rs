#![feature(const_type_id)]

#[macro_use]
extern crate infuse;

#[derive(Controller)]
#[controller(path = "/pet/:id", dependencies(PetService, OwnerService))]
pub struct PetController {
    pet_service: PetService,
    pet_service1: PetService,
    pet_service2: PetService,
    pet_service3: PetService,
    owner_service: OwnerService,
}

/* #[derive(Controller)]
#[controller(path = "/owner/")]
pub struct OwnerController {
    owner_service: OwnerService,
} */
/*
impl PetController {
    #[get("/")]
    pub async fn get(id: u32) -> u32 {
        id
    }
} */

#[derive(Service)]
pub struct PetService {}

impl PetService {
    pub fn hey(&self) -> String {
        format!("pet")
    }
}

#[derive(Service)]
pub struct OwnerService {
    pet_serivce: PetService,
}
/*
impl OwnerService {
    pub fn hey(&self) -> String {
        format!("hey {}", self.pet_serivce.hey())
    }
} */

pub fn main() {
    /* let s = OwnerService {
        pet_serivce: PetService {  }
    }; */

    /* println!(
        "Total dependencies: {}",
        PetController::
    );
    for dependency in PetController::dependencies {
        println!(
            "{:?}",
            format!("{:?}", dependency) == format!("{:?}", PetController::dependencies[0])
        );
    } */
}
