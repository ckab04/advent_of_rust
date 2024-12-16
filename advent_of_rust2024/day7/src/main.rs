fn main() {
    println!("Hello, world!");
}

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a>{

    // 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    //
    // 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
    //    returns a vector of references to those logs.


    fn new(logs: &'a Vec<String>) -> Self{
        Self { logs}
    }

    fn search(&self, keyword: &str){

        let value: Vec<&str> = self.logs.into_iter()
            .map(|y| y.as_str())
                .filter(|x| x.contains(keyword))
                .collect();


    }

}
