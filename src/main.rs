pub mod date;
pub mod dump;

fn main() {
    println!("{}", date::get_todays_np_date());
    println!("{}", date::convert_ad_to_bs(2002, 12, 31));
    println!("{}", date::convert_utc_to_bs("2023-10-20T11:27:16.537Z"));
}
