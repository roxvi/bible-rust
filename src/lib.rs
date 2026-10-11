//! The complete King James Bible as static Rust data.
#[derive(Debug)]
pub struct Verse {
    pub number: u16,
    pub text: &'static str,
}

#[derive(Debug)]
pub struct Chapter {
    pub number: u16,
    pub verses: &'static [Verse],
}

#[derive(Debug)]
pub struct Book {
    pub name: &'static str,
    pub chapters: &'static [Chapter],
}

mod bible;
pub use bible::BIBLE;

pub fn book(name: &str) -> Option<&'static Book> {
    BIBLE
        .iter()
        .find(|book| book.name.eq_ignore_ascii_case(name))
}

pub fn verse(name: &str, chapter: u16, number: u16) -> Option<&'static Verse> {
    book(name)?
        .chapters
        .iter()
        .find(|c| c.number == chapter)?
        .verses
        .iter()
        .find(|v| v.number == number)
}

/// Case-insensitive substring search, in canonical book order.
pub fn search(
    query: &str,
) -> impl Iterator<Item = (&'static Book, &'static Chapter, &'static Verse)> {
    let query = query.to_lowercase();
    BIBLE
        .iter()
        .flat_map(|b| {
            b.chapters
                .iter()
                .flat_map(move |c| c.verses.iter().map(move |v| (b, c, v)))
        })
        .filter(move |(_, _, v)| v.text.to_lowercase().contains(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_and_sequential() {
        assert_eq!(BIBLE.len(), 66);
        assert_eq!(BIBLE.iter().map(|b| b.chapters.len()).sum::<usize>(), 1189);
        assert_eq!(
            BIBLE
                .iter()
                .flat_map(|b| b.chapters)
                .map(|c| c.verses.len())
                .sum::<usize>(),
            31102
        );
        for b in BIBLE {
            for (i, c) in b.chapters.iter().enumerate() {
                assert_eq!(c.number as usize, i + 1);
                for (j, v) in c.verses.iter().enumerate() {
                    assert_eq!(v.number as usize, j + 1);
                    assert!(!v.text.trim().is_empty());
                }
            }
        }
    }

    #[test]
    fn lookup_and_search() {
        assert_eq!(
            verse("genesis", 1, 1).unwrap().text,
            "In the beginning God created the heaven and the earth."
        );
        assert!(verse("Genesis", 0, 1).is_none());
        assert!(verse("Unknown", 1, 1).is_none());
        assert!(verse("Genesis", 1, 99).is_none());
        assert!(search("FOR GOD SO LOVED THE WORLD")
            .any(|(b, c, v)| b.name == "John" && c.number == 3 && v.number == 16));
        assert!(verse("Revelation", 22, 21).is_some());
    }
}
