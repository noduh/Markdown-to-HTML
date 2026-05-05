use std::collections::HashMap;
use std::fs;
use std::io;

fn main() {
    let mut file_path = String::new();

    println!("Please give the file path: ");
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read line");

    let file_path = file_path.trim();

    let file_contents = file_to_string(file_path);
    println!("The file reads:\n{file_contents}\nEOF");
}

fn file_to_string(file_path: &str) -> String {
    let file_contents = fs::read_to_string(file_path.trim()).expect("Failed to read the file");

    file_contents
}

// Nested Formatting Structure
enum Format {
    // Plain Text
    Raw,  // possibly contains more formatting
    Text, // contains text inside formatting

    // Headings
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,

    // Sectioning Text
    Paragraph,
    LineBreak,
    BlockQuote,

    // Emphasis
    Bold,
    Italic,

    // Lists
    OrderedList,
    OrderedListItem,
    UnorderedList,
    UnorderedListItem,
    ElementInList, // This is like adding a paragraph below an item while maintaining continuity. Important for markdown but probably ignored in HTML

    // Code
    CodeBlock(Option<String>), // the string holds the language for syntax highlighting
    InlineCode,

    // Horizontal Rule
    HorizontalRule,

    // Links
    HyperLink(String, Option<String>), // first string is href, second string is the title if the link has one
    Address,                           // a link or email address without regular text
    FootnoteLink(u32), // comes after the text that has a reference. u32 is the id (to match link to note)
    Footnote(u32), // is the data to be linked to as a footnote. u32 is the id (to match link to note)

    // Images
    Image(String, String, Option<String>), // first string is source, second is alt text, third is title
}
