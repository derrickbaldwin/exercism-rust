pub fn verse(n: i32) -> String {
    match n {
        0 => format!("{}", "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => format!("{}", "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
        
    }
}

pub fn sing(start: i32, end: i32) -> String {
    let mut song = verse(start);
    for i in (end..start).rev() {
        let new_phrase = "\n".to_string() + &verse(i);
        song.push_str(&new_phrase);
    }
    song
}
