
pub fn is_capped(s: &str) -> bool {
    s == s.to_ascii_uppercase()
}

pub fn is_question(s: &str) -> bool {
    s.ends_with("?")
}

pub fn is_a_char(s: &str) -> bool {
    for x in s.chars() {
        if x.is_alphabetic() {
            return true;
        }
    }
    false
}

pub fn reply(message: &str) -> &str {
    let s = message.trim();
    match message.trim() {
        "" => "Fine. Be that way!",
        _  => match (is_capped(s), is_question(s), is_a_char(s))  {
                (true, true, true)   => "Calm down, I know what I'm doing!",
                (true, false, true)  => "Whoa, chill out!",
                (true, true, false)  => "Sure.",
                (false, true, _)     => "Sure.",
                _                    => "Whatever."
            }
    }
}
