use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field

    records: HashMap<String, bool>,


}

impl SantaList {
    // 2. Implement the new method
    fn new() -> Self{
        Self{
             records: HashMap::new(),
            }
    }
     // 3. Implement the add method

    fn add(&mut self, name: &str, behaviour: bool){
        self.records.insert(name.into(), behaviour);
    }

    // 4. Implement the remove method

    // 5. Implement the get method

    fn get(&self, name: &str)-> Option<bool>{
        self.records.get(name.into()).cloned()
    }
    // 6. Implement the count method

    fn count(&self)-> (u32, u32){
        let (nice, naughty) = self.records.iter()
                .fold((0,0), |(nice, naughty), (_, &val)|{

                    if val{
                        (nice + 1, naughty)
                    }else{
                        (nice, naughty + 1)
                    }

                }
                );
        (nice, naughty)
    }


    // 7. Implement the list_by_behavior method
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
