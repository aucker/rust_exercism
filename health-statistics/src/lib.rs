// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/*pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        unimplemented!()
    }

    pub fn name(&self) -> &str {
        unimplemented!()
    }

    pub fn age(&self) -> u32 {
        unimplemented!()
    }

    pub fn weight(&self) -> f32 {
        unimplemented!()
    }

    pub fn set_age(&mut self, new_age: u32) {
        unimplemented!()
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        unimplemented!()
    }
}*/

pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    pub fn new(name: String, age: u32, weight: f32) -> Self {
        /*User {
            name: String::from("Bob"),
            age: 32,
            weight: 155.2,
        }*/
        Self { name, age, weight }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn weight(&self) -> f32 {
        self.weight
    }
    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
        //println!("Updates {}'s age to {}; happy birthday {}!", self.name, new_age, self.name)
    }
    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
        //println!("Update {}'s weight to {}", self.name, new_weight)
    }
}
/*fn main() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    println!("a User with name {}, age {}, and weight {}", bob.name, bob.age, bob.name);
}*/