use chrono::Utc;

mod day_1;
mod day_2;


fn main() 
{
    let now = Utc::now();
    day_1::puzzle_solution();
    day_2::puzzle_solution();
    println!("Finished run. {}ms passed", (Utc::now() - now).num_milliseconds());
}


