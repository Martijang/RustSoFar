
pub struct WareHouse{
    items: Vec<&'static str>,
    capacity: i32
}

pub struct Cargo{
    list: Vec<&'static str>,
    capacity: i32
}
pub trait Manage{
    fn new_items(&mut self, name: &'static str);

    fn delete_from_back(&mut self) -> Option<&'static str>;
}

impl Manage for WareHouse{
    fn new_items(&mut self, name: &'static str) {
        self.items.push(name);
        self.capacity+=1;
    }

    fn delete_from_back(&mut self) -> Option<&'static str>{
        return self.items.pop();
    }
}

impl Manage for Cargo {
    fn new_items(&mut self, name: &'static str) {
        self.list.push(name);
        self.capacity += 1;
    }
    fn delete_from_back(&mut self) -> Option<&'static str> {
        return self.list.pop();
    }
}

fn main(){
    let mut ware_house = WareHouse{ items: Vec::new(), capacity: 0};
    let mut cargo = Cargo{ list: Vec::new(), capacity: 0};

    ware_house.new_items("Box of Tomatoes");
    cargo.new_items("idk");

    println!("cargo: {:?}, capacity: {}", cargo.delete_from_back().or(None), cargo.capacity);
    println!("Ware house: {:?}, capacity: {}", ware_house.delete_from_back().or(None), ware_house.capacity);
    println!("{:?}", ware_house.delete_from_back().or(None));
}