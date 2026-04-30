# Nepali Date Converter

A Rust crate for converting dates from the Gregorian calendar to the Nepali
calendar (Bikram Sambat).

## Features

- `NepaliDate` — strongly-typed BS date with `year`, `month`, `day`, and
  `weekday` accessors plus a `Display` impl.
- Convert in both directions: AD → BS from `(y, m, d)`, a
  `chrono::NaiveDate`, or an RFC 3339 timestamp string; BS → AD via
  `NepaliDate::from_bs_ymd` and `NepaliDate::to_gregorian`.
- Errors are returned via `Result<_, NepaliDateError>` rather than panicking
  or returning sentinel strings.
- Just two transitive dependencies: `chrono` and `num-traits`.

## Usage

```toml
[dependencies]
adtobs = "0.2"
```

## Examples

### Today's Nepali date

```rust
use adtobs::NepaliDate;

let today = NepaliDate::today();
println!("Today is {today}");
println!("year={}, month={}, day={}", today.year(), today.month(), today.day());
```

### Convert a Gregorian (year, month, day)

```rust
use adtobs::{Month, NepaliDate};

let bs = NepaliDate::from_gregorian_ymd(2023, 11, 29).unwrap();
assert_eq!(bs.year(), 2080);
assert_eq!(bs.month(), Month::Mangsir);
assert_eq!(bs.day(), 13);
assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
```

### Convert from a `chrono::NaiveDate`

```rust
use adtobs::NepaliDate;
use chrono::NaiveDate;

let nd = NaiveDate::from_ymd_opt(2023, 11, 29).unwrap();
let bs = NepaliDate::try_from(nd).unwrap();
println!("{bs}");
```

### Parse an RFC 3339 / ISO 8601 timestamp

The instant is interpreted in Nepal Time (UTC+05:45) before conversion.

```rust
use adtobs::NepaliDate;

let bs: NepaliDate = "2023-11-29T12:00:00Z".parse().unwrap();
assert_eq!(bs.to_string(), "2080 Mangsir 13, Wednesday");
```

### Convert a Nepali date back to Gregorian

```rust
use adtobs::NepaliDate;
use chrono::NaiveDate;

let bs = NepaliDate::from_bs_ymd(2080, 8, 13).unwrap();
let ad: NaiveDate = bs.into(); // or bs.to_gregorian()
assert_eq!(ad, NaiveDate::from_ymd_opt(2023, 11, 29).unwrap());
```

The supported BS year range is 2000..=2090.

## Backwards compatibility

The old free functions still work but are deprecated and will be removed in
a future major release:

| Deprecated                            | Replacement                                   |
| ------------------------------------- | --------------------------------------------- |
| `get_todays_np_date()`                | `NepaliDate::today().to_string()`             |
| `convert_ad_to_bs(y, m, d)`           | `NepaliDate::from_gregorian_ymd(y, m, d)`     |
| `convert_utc_to_bs(s)`                | `NepaliDate::from_rfc3339(s)` / `s.parse()`   |

## License

This project is licensed under the GNU General Public License v3.0.
