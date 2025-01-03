// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than or equal to 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / ((10 * 1) + (2 * 2)) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty
    //

    if good_deeds == 0 && bad_deeds == 0{
        return false;
    }
    let good= (good_deeds as f32 * GOOD_WEIGHT);
    let bad = bad_deeds as f32 * BAD_WEIGHT;

    //let ratio = / ((good_deeds as f32 * GOOD_WEIGHT) + ( (2 * bad_deeds) as f32 * BAD_WEIGHT));
    let ratio = good / (good + bad);
    println!("Ratio value = {}", ratio);

    if ratio > 0.75{
        return true;
    }
    false
}



fn main() {
    println!("Hello, world!");
    let val = is_nice(10, 2);
    println!("Ratio = {}", val);
}
