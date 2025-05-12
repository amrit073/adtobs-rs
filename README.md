# Nepali Date Converter

A Rust crate for converting dates from the Gregorian calendar to the Nepali calendar (Bikram Sambat).

## Features

- `get_todays_np_date`: Get today's date in the Nepali calendar.
- `convert_ad_to_bs`: Convert a Gregorian (English) date to a Nepali date.
- `convert_utc_to_bs`: Convert a UTC date string to a Nepali date.

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
adtobs = "0.1.3"
```

## Examples
### Get Today's Nepali Date
```rust
use adtobs::get_todays_np_date;

let nepali_date = get_todays_np_date();
println!("Today's Nepali Date: {}", nepali_date);
```

### Convert Gregorian Date to Nepali Date
```rust
use adtobs::convert_ad_to_bs;

let nepali_date = convert_ad_to_bs(2023, 11, 29);
println!("Nepali Date: {}", nepali_date);
```

### Convert UTC String to Nepali Date
```rust
use adtobs::convert_utc_to_bs;

let utc_date = "2023-11-29T12:00:00Z";
let nepali_date = convert_utc_to_bs(utc_date);
println!("Nepali Date: {}", nepali_date);
```

### License
This project is licensed under the GNU General Public License v3.0
