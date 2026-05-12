// Nested Formatting Structure
pub enum Format {
    // Plain Text
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
    Bottom, // Signals that this is the deepest this branch goes
}

pub struct FormattedText {
    // Data in the struct
    format: Format,
    content: Vec<Content>, // content can be nested to contain more formatting
}

impl FormattedText {
    pub fn new(format: Format) -> Self {
        Self {
            format: format,
            content: Vec::new(),
        }
    }

    pub fn default() -> Self {
        Self {
            format: Format::Text,
            content: Vec::new(),
        }
    }

    pub fn append_format(format: Format) {
        // this adds a new branch
        // TODO: change this to do depth first using get_end()
        FormattedText::new(format);
    }

    pub fn append_text(&mut self, string_to_append: &str) {
        // TODO: change this to use get_end() to add using depth-first
        self.content
            .push(Content::JustText(string_to_append.to_string()));
    }

    fn get_end(&mut self) -> &mut FormattedText {
        // This will "go deep" and find the furthest it can go using depth first
        if matches!(self.content.last(), Some(Content::Bottom)) {
            // short circuit and end recursion
            return self;
        };
        // TODO: begin going deep
    }
}

impl Default for FormattedText {
    fn default() -> Self {
        Self::default()
    }
}
