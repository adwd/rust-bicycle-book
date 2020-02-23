use std::io;
use std::io::Write;
mod circle;

fn main() {
    let mut year = String::new();
    print!("Please input a year to check if it is a leap year: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut year).unwrap();
    let year = year.trim().parse::<u32>().unwrap();

    if is_leap_year(year) {
        println!("{} is a leap year!", year);
    } else {
        println!("{} is not a leap year.", year);
    }

    println!("{:?}", circle::Circle::small_circle());

    {
        let n = 42;
        let face = match n {
            1 | 2 | 3 => ":)".to_string(),
            10..=20 => ":(".to_string(),
            x if x % 100 == 0 => format!("{}", x),
            _ => ":innocent:".to_string(),
        };
    }
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
}

mod test {

    #[test]
    fn test_leap_year() {
        struct TestCase(u32, bool);
        use super::is_leap_year;

        let test_cases = vec![
            TestCase(2000, true),
            TestCase(1999, false),
            TestCase(2100, false),
        ];

        test_cases
            .iter()
            .for_each(|t| assert_eq!(is_leap_year(t.0), t.1))
    }
}
