pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1

        // 🎁 Your code here! 🎁
        let val = csv_row.split(",").collect::<Vec<&str>>();
        if val.len() != 3{
            return Err("Invalid csv line");
        }
        let name = val.get(0).unwrap_or(&" ");
        let name = String::from(*name);
        let good_deeds = val[1].parse::<u32>().map_err(|_| "Invalid value to parse")?;
        let bad_deeds = val[2].parse::<u32>().map_err(|_| "Invalid value to parse")?;

        Ok(Self::new(name, good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
           if good_deeds == 0 && bad_deeds == 0 {
               return false;
           }

           let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
           let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

           let ratio = good_deeds / (good_deeds + bad_deeds);

           ratio >= 0.75
       }
}
