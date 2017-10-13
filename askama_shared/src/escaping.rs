use std::fmt::{self, Display, Formatter};


#[derive(Debug, PartialEq)]
pub enum MarkupDisplay<T> where T: Display {
    Safe(T),
    Unsafe(T),
}

impl<T> MarkupDisplay<T> where T: Display {
    pub fn mark_safe(self) -> MarkupDisplay<T> {
        match self {
            MarkupDisplay::Unsafe(t) => MarkupDisplay::Safe(t),
            _ => { self },
        }
    }
    pub fn unsafe_string(&self) -> String {
        match *self {
            MarkupDisplay::Safe(ref t) | MarkupDisplay::Unsafe(ref t) => format!("{}", t)
        }
    }
}

impl<T> From<T> for MarkupDisplay<T> where T: Display {
    fn from(t: T) -> MarkupDisplay<T> {
        MarkupDisplay::Unsafe(t)
    }
}

impl<T> Display for MarkupDisplay<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            MarkupDisplay::Unsafe(_) => {
                write!(f, "{}", escape(self.unsafe_string()))
            },
            MarkupDisplay::Safe(ref t) => {
                t.fmt(f)
            },
        }
    }
}

pub fn escape(input: String) -> String {
    let mut output = String::with_capacity(input.len() * 2);
    for c in input.chars() {
        match c {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            '`' => output.push_str("&#96;"),
            _ => output.push(c)
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_escape() {
        assert_eq!(escape("".to_string()), "");
        assert_eq!(escape("<&>".to_string()), "&lt;&amp;&gt;");
        assert_eq!(escape("bla&".to_string()), "bla&amp;");
        assert_eq!(escape("<foo".to_string()), "&lt;foo");
    }
}
