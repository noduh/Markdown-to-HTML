// Nested Formatting Structure
pub enum Format {
    Head, // used only at the beginning

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

pub enum Content {
    // This lets me nest things
    JustText(String),
    ChildNode(Box<FormattedText>),
}

pub struct FormattedText {
    // Data in the struct
    format: Format,
    content: Vec<Content>, // content can be nested to contain more formatting
}

impl FormattedText {
    pub fn new(format: Format, content: Vec<Content>) -> Self {
        Self {
            format: format,
            content: content,
        }
    }

    pub fn default() -> Self {
        Self {
            format: Format::Head, // This is the root of it all
            content: vec![],
        }
    }
}

impl Default for FormattedText {
    fn default() -> Self {
        Self::default()
    }
}
