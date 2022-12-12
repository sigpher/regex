use std::error::Error;

use regex::Regex;
use regex::RegexSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let text = client
        .get("https://movie.douban.com/subject/1428175/")
        .send()
        .await?
        .text()
        .await?;
    // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})")?;
    // for cap in re.captures_iter(&text) {
    //     println!("Year: {}, Month: {}, Day: {}", &cap[1], &cap[2], &cap[3]);
    // }

    // let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
    // let after = re.replace_all(&text, "$m/$d/$y");
    // println!("{}", after);

    // If you wish to match against whitespace in this mode,
    // you can still use \s, \n, \t, etc. For escaping a single
    // space character, you can escape it directly with \ , use
    // its hex character code \x20 or temporarily disable
    // the x flag, e.g., (?-x: ).
    // let re = Regex::new(
    //     r"(?x)
    // (?P<y>\d{4}) #the year
    // -
    // (?P<m>\d{2}) #the month
    // -
    // (?P<d>\d{2}) #the day
    // ",
    // )
    // .unwrap();
    // let after = re.replace_all(&text, "$m/$d/$y");
    // println!("{}", after);

    let set = RegexSet::new(&[
        r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
    ])
    .unwrap();
    let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // You can also test whether a particular regex matched:
    let matches = set.matches("foobar");
    assert!(!matches.matched(5));
    assert!(matches.matched(6));
    Ok(())
}
