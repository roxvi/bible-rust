use std::io::{self, Write};

fn find(input: &str, out: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(());
    }
    if input.eq_ignore_ascii_case("books") {
        for b in bible_rust::BIBLE {
            writeln!(out, "{} ({} chapters)", b.name, b.chapters.len())?;
        }
    } else if let Some(query) = input.strip_prefix("search ") {
        print_search(query, out)?;
    } else if let Some((name, location)) =
        input.rsplit_once(' ').filter(|(_, loc)| loc.contains(':'))
    {
        let (chapter, number) = location
            .split_once(':')
            .ok_or("Use a reference like John 3:16")?;
        let chapter = chapter
            .parse::<u16>()
            .map_err(|_| "Chapter must be a positive number")?;
        let number = number
            .parse::<u16>()
            .map_err(|_| "Verse must be a positive number")?;
        let b = bible_rust::book(name.trim())
            .ok_or("Book not found. Type books to see available names.")?;
        let c = b
            .chapters
            .iter()
            .find(|c| c.number == chapter)
            .ok_or_else(|| format!("{} has chapters 1–{}", b.name, b.chapters.len()))?;
        let v = c
            .verses
            .iter()
            .find(|v| v.number == number)
            .ok_or_else(|| format!("{} {} has verses 1–{}", b.name, chapter, c.verses.len()))?;
        writeln!(out, "{} {}:{} {}", b.name, chapter, number, v.text)?;
    } else {
        print_search(input, out)?;
    }
    Ok(())
}

fn print_search(query: &str, out: &mut impl Write) -> Result<(), Box<dyn std::error::Error>> {
    if query.trim().is_empty() {
        return Err("Enter some words to search for".into());
    }
    let mut count = 0;
    for (b, c, v) in bible_rust::search(query.trim()) {
        writeln!(out, "{} {}:{} {}", b.name, c.number, v.number, v.text)?;
        count += 1;
    }
    if count == 0 {
        writeln!(
            out,
            "No verses found. Try a shorter phrase or different words (King James wording)."
        )?;
    } else {
        writeln!(out, "\n{count} matching verse(s).")?;
    }
    Ok(())
}

fn interactive(input: &mut impl io::BufRead, out: &mut impl Write) -> io::Result<()> {
    writeln!(out, "Bible verse finder — King James Version\n\nEnter a reference: John 3:16 or 1 Corinthians 13:4\nOr enter words to search: let there be light\nType books to list books, or quit to exit.\n")?;
    loop {
        write!(out, "Find a verse > ")?;
        out.flush()?;
        let mut line = String::new();
        if input.read_line(&mut line)? == 0 {
            break;
        }
        if matches!(line.trim().to_lowercase().as_str(), "quit" | "exit") {
            break;
        }
        if let Err(e) = find(&line, out) {
            if let Some(e) = e.downcast_ref::<io::Error>() {
                return Err(io::Error::new(e.kind(), e.to_string()));
            }
            writeln!(out, "{e}")?;
        }
        writeln!(out)?;
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut out = io::BufWriter::new(io::stdout().lock());
    match args.first().map(String::as_str) {
        Some("books") if args.len() == 1 => {
            for b in bible_rust::BIBLE {
                writeln!(out, "{} ({} chapters)", b.name, b.chapters.len())?;
            }
        }
        Some("verse") if args.len() == 4 => {
            let chapter = args[2].parse::<u16>()?;
            let number = args[3].parse::<u16>()?;
            let b = bible_rust::book(&args[1]).ok_or("Book not found")?;
            let v = bible_rust::verse(b.name, chapter, number).ok_or("Verse not found")?;
            writeln!(out, "{} {}:{} {}", b.name, chapter, number, v.text)?;
        }
        Some("search") if args.len() >= 2 => {
            let query = args[1..].join(" ");
            if query.trim().is_empty() {
                return Err("Search text cannot be empty".into());
            }
            for (b, c, v) in bible_rust::search(&query) {
                writeln!(out, "{} {}:{} {}", b.name, c.number, v.number, v.text)?;
            }
        }
        None => interactive(&mut io::stdin().lock(), &mut out)?,
        Some("--help" | "-h") => {
            writeln!(out, "Bible (King James Version)\n\nUsage:\n  bible-rust books\n  bible-rust verse <book> <chapter> <verse>\n  bible-rust search <text>\n\nExample: bible-rust verse \"1 Corinthians\" 13 4")?;
        }
        _ => find(&args.join(" "), &mut out)?,
    }
    out.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interactive_recovers_and_finds_references_and_words() {
        let mut input = io::Cursor::new("John 3:999\nJohn 3:16\n1 Corinthians 13:4\nlet there be light\nzzzznonexistent\nquit\nGenesis 1:1\n");
        let mut out = Vec::new();
        interactive(&mut input, &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        assert!(text.contains("John 3 has verses 1–36"));
        assert!(text.contains("John 3:16 For God so loved"));
        assert!(text.contains("1 Corinthians 13:4 Charity"));
        assert!(text.contains("Genesis 1:3"));
        assert!(text.contains("No verses found"));
        assert!(!text.contains("Genesis 1:1 In the beginning"));
    }

    #[test]
    fn eof_exits() {
        interactive(&mut io::Cursor::new(""), &mut Vec::new()).unwrap();
    }
}

fn main() {
    if let Err(e) = run() {
        if e.downcast_ref::<io::Error>()
            .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe)
        {
            return;
        }
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
