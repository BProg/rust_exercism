pub fn verse(n: i32) -> String {
  format!("{} {} of beer on the wall, {} {} of beer.
{}, {} {} of beer on the wall.
", count_capitalized(n), bottle(n),
count(n), bottle(n),
take_beer(n),
count(n - 1), bottle(n - 1))
}

pub fn sing(start: i32, end: i32) -> String {
  (end..start+1)
    .into_iter()
    .map(verse)
    .rev()
    .collect::<Vec<String>>()
    .join("\n")
}

fn count(n: i32) -> String {
  if n == 0 {
    return String::from("no more");
  } else if n == -1 {
    return 99.to_string();
  }
  n.to_string()
}

fn count_capitalized(n: i32) -> String {
  if n == 0 {
    return String::from("No more");
  }
  count(n)
}

fn bottle(n: i32) -> String {
  if n == 1 {
    return String::from("bottle");
  }
  String::from("bottles")
}

fn take_beer(n: i32) -> String {
  if n == 0 {
    return String::from("Go to the store and buy some more")
  }
  format!("Take {} down and pass it around", if n == 1 { "it" } else { "one" })
}
