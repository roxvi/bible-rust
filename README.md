# Bible in Rust

The complete 66-book King James Bible stored as static `Book`, `Chapter`, and
`Verse` structs. No runtime downloads or external Rust dependencies.

Run `cargo run` to open the interactive verse finder. Enter `John 3:16`,
`1 Corinthians 13:4`, or words you remember, such as `let there be light`.
References are case-insensitive; phrase searches use King James wording.
Type `books` to see book names and `quit` to exit.

On macOS, first run `cargo build --release`, then double-click
`Open Bible.command` to launch the program. The executable is
`target/release/bible-rust`; it includes the entire Bible and works offline.
To share it with another computer, build on that computer's operating system
and architecture, then share the executable.

```sh
cargo run
cargo run -- "John 3:16"
cargo run -- verse Genesis 1 1
cargo run -- verse "John" 3 16
cargo run -- verse "1 Corinthians" 13 4
cargo run -- search "let there be light"
cargo run -- books
cargo test
```

Use the library via `bible_rust::BIBLE`, `bible_rust::verse("John", 3, 16)`,
or `bible_rust::search("light")`. Chapter and verse numbers start at 1.

Text source: https://github.com/aruljohn/Bible-kjv (King James Version).
The source dataset's MIT notice is preserved in `data/LICENSE`. Text is
converted without paraphrasing; this project does not independently certify
the source transcription.

To regenerate `src/bible.rs`, download that repository's ZIP and run:

```sh
python3 tools/import_bible.py /path/to/Bible-kjv-master.zip
```
