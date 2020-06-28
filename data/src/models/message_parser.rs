use nom::branch::alt;
use nom::bytes::complete::{is_a, is_not, tag, tag_no_case};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::opt;
use nom::combinator::{map, not, peek, recognize, value};
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpeningTag {
    Bold,
    Italic,
    Underline,
    Strike,
    Superscript,
    Subscript,
    Code,
    CodeBlock,
    Spoiler,
    Color { color: String },
    Quote,
}

impl OpeningTag {
    pub fn is_pair(self: &OpeningTag, closing: &ClosingTag) -> bool {
        match (self, closing) {
            (OpeningTag::Bold, ClosingTag::Bold) => true,
            (OpeningTag::Italic, ClosingTag::Italic) => true,
            (OpeningTag::Underline, ClosingTag::Underline) => true,
            (OpeningTag::Strike, ClosingTag::Strike) => true,
            (OpeningTag::Superscript, ClosingTag::Superscript) => true,
            (OpeningTag::Subscript, ClosingTag::Subscript) => true,
            (OpeningTag::Code, ClosingTag::Code) => true,
            (OpeningTag::CodeBlock, ClosingTag::CodeBlock) => true,
            (OpeningTag::Spoiler, ClosingTag::Spoiler) => true,
            (OpeningTag::Color { color: _ }, ClosingTag::Color) => true,
            (OpeningTag::Quote, ClosingTag::Quote) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for OpeningTag {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            OpeningTag::Bold => String::from("[b]"),
            OpeningTag::Italic => String::from("[i]"),
            OpeningTag::Underline => String::from("[u]"),
            OpeningTag::Strike => String::from("[s]"),
            OpeningTag::Superscript => String::from("[sup]"),
            OpeningTag::Subscript => String::from("[sub]"),
            OpeningTag::Code => String::from("[code]"),
            OpeningTag::CodeBlock => String::from("[codeblock]"),
            OpeningTag::Spoiler => String::from("[spoiler]"),
            OpeningTag::Color { color } => format!("[color={}]", color),
            OpeningTag::Quote => String::from(""),
        };

        fmt.write_str(&s)?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosingTag {
    Bold,
    Italic,
    Underline,
    Strike,
    Superscript,
    Subscript,
    Code,
    CodeBlock,
    Spoiler,
    Color,
    Quote,
}

impl std::fmt::Display for ClosingTag {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str(match self {
            ClosingTag::Bold => "[/b]",
            ClosingTag::Italic => "[/i]",
            ClosingTag::Underline => "[/u]",
            ClosingTag::Strike => "[/s]",
            ClosingTag::Superscript => "[/sup]",
            ClosingTag::Subscript => "[/sub]",
            ClosingTag::Code => "[/code]",
            ClosingTag::CodeBlock => "[/codeblock]",
            ClosingTag::Spoiler => "[/spoiler]",
            ClosingTag::Color => "[/color]",
            ClosingTag::Quote => "",
        })?;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Text(String),
    RefLink(u32),
    Link(String),
    OpeningTag(OpeningTag),
    ClosingTag(ClosingTag),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum SegmentStyle {
    Bold,
    Italic,
    Underline,
    Strike,
    Superscript,
    Subscript,
    Code,
    CodeBlock,
    Spoiler,
    Color { color: String },
    RefLink { id: u32 },
    Link { url: String },
    Quote,
}

impl SegmentStyle {
    pub fn from_opening_tag(tag: &OpeningTag) -> SegmentStyle {
        match tag {
            OpeningTag::Bold => SegmentStyle::Bold,
            OpeningTag::Italic => SegmentStyle::Italic,
            OpeningTag::Underline => SegmentStyle::Underline,
            OpeningTag::Strike => SegmentStyle::Strike,
            OpeningTag::Superscript => SegmentStyle::Superscript,
            OpeningTag::Subscript => SegmentStyle::Subscript,
            OpeningTag::Code => SegmentStyle::Code,
            OpeningTag::CodeBlock => SegmentStyle::CodeBlock,
            OpeningTag::Spoiler => SegmentStyle::Spoiler,
            OpeningTag::Color { color } => SegmentStyle::Color {
                color: color.to_string(),
            },
            OpeningTag::Quote => SegmentStyle::Quote,
        }
    }

    pub fn is_pair(self: &SegmentStyle, closing: &ClosingTag) -> bool {
        match (self, closing) {
            (SegmentStyle::Bold, ClosingTag::Bold) => true,
            (SegmentStyle::Italic, ClosingTag::Italic) => true,
            (SegmentStyle::Underline, ClosingTag::Underline) => true,
            (SegmentStyle::Strike, ClosingTag::Strike) => true,
            (SegmentStyle::Superscript, ClosingTag::Superscript) => true,
            (SegmentStyle::Subscript, ClosingTag::Subscript) => true,
            (SegmentStyle::Code, ClosingTag::Code) => true,
            (SegmentStyle::CodeBlock, ClosingTag::CodeBlock) => true,
            (SegmentStyle::Spoiler, ClosingTag::Spoiler) => true,
            (SegmentStyle::Color { color: _ }, ClosingTag::Color) => true,
            (SegmentStyle::Quote, ClosingTag::Quote) => true,
            _ => false,
        }
    }

    pub fn is_ref_link(&self) -> bool {
        match self {
            SegmentStyle::RefLink { id: _ } => true,
            _ => false,
        }
    }

    pub fn get_ref_link(&self) -> Option<u32> {
        match self {
            SegmentStyle::RefLink { id } => Some(*id),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Segment {
    pub text: String,
    pub tags: Vec<SegmentStyle>,
}

impl Segment {
    pub fn has_ref_link(&self) -> bool {
        self.tags.iter().position(|tag| tag.is_ref_link()) != None
    }

    pub fn get_ref_link(&self) -> Option<u32> {
        let index = self.tags.iter().position(|tag| tag.is_ref_link());
        index.and_then(|i| self.tags[i].get_ref_link())
    }
}

pub struct MessageParser();

impl MessageParser {
    fn color(input: &str) -> IResult<&str, &str> {
        alt((
            // CSS Color Module Level 4.
            tag_no_case("rebeccapurple"),
            // CSS Color Module Level 3.
            alt((
                tag_no_case("yellowgreen"),
                tag_no_case("whitesmoke"),
                tag_no_case("wheat"),
                tag_no_case("violet"),
                tag_no_case("turquoise"),
                tag_no_case("tomato"),
                tag_no_case("thistle"),
                tag_no_case("tan"),
                tag_no_case("steelblue"),
                tag_no_case("springgreen"),
                tag_no_case("snow"),
                tag_no_case("slategrey"),
                tag_no_case("slategray"),
                tag_no_case("slateblue"),
                tag_no_case("skyblue"),
                tag_no_case("sienna"),
                tag_no_case("seashell"),
                tag_no_case("seagreen"),
                tag_no_case("sandybrown"),
                tag_no_case("salmon"),
                tag_no_case("saddlebrown"),
            )),
            alt((
                tag_no_case("royalblue"),
                tag_no_case("rosybrown"),
                tag_no_case("powderblue"),
                tag_no_case("plum"),
                tag_no_case("pink"),
                tag_no_case("peru"),
                tag_no_case("peachpuff"),
                tag_no_case("papayawhip"),
                tag_no_case("palevioletred"),
                tag_no_case("paleturquoise"),
                tag_no_case("palegreen"),
                tag_no_case("palegoldenrod"),
                tag_no_case("orchid"),
                tag_no_case("orangered"),
                tag_no_case("olivedrab"),
                tag_no_case("oldlace"),
                tag_no_case("navajowhite"),
                tag_no_case("moccasin"),
                tag_no_case("mistyrose"),
                tag_no_case("mintcream"),
                tag_no_case("midnightblue"),
            )),
            alt((
                tag_no_case("mediumvioletred"),
                tag_no_case("mediumturquoise"),
                tag_no_case("mediumspringgreen"),
                tag_no_case("mediumslateblue"),
                tag_no_case("mediumseagreen"),
                tag_no_case("mediumpurple"),
                tag_no_case("mediumorchid"),
                tag_no_case("mediumblue"),
                tag_no_case("mediumaquamarine"),
                tag_no_case("magenta"),
                tag_no_case("linen"),
                tag_no_case("limegreen"),
                tag_no_case("lightyellow"),
                tag_no_case("lightsteelblue"),
                tag_no_case("lightslategrey"),
                tag_no_case("lightslategray"),
                tag_no_case("lightskyblue"),
                tag_no_case("lightseagreen"),
                tag_no_case("lightsalmon"),
                tag_no_case("lightpink"),
                tag_no_case("lightgrey"),
            )),
            alt((
                tag_no_case("lightgreen"),
                tag_no_case("lightgray"),
                tag_no_case("lightgoldenrodyellow"),
                tag_no_case("lightcyan"),
                tag_no_case("lightcoral"),
                tag_no_case("lightblue"),
                tag_no_case("lemonchiffon"),
                tag_no_case("lawngreen"),
                tag_no_case("lavenderblush"),
                tag_no_case("lavender"),
                tag_no_case("khaki"),
                tag_no_case("ivory"),
                tag_no_case("indigo"),
                tag_no_case("indianred"),
                tag_no_case("hotpink"),
                tag_no_case("honeydew"),
                tag_no_case("grey"),
                tag_no_case("greenyellow"),
                tag_no_case("goldenrod"),
                tag_no_case("gold"),
                tag_no_case("ghostwhite"),
            )),
            alt((
                tag_no_case("gainsboro"),
                tag_no_case("forestgreen"),
                tag_no_case("floralwhite"),
                tag_no_case("firebrick"),
                tag_no_case("dodgerblue"),
                tag_no_case("dimgrey"),
                tag_no_case("dimgray"),
                tag_no_case("deepskyblue"),
                tag_no_case("deeppink"),
                tag_no_case("darkviolet"),
                tag_no_case("darkturquoise"),
                tag_no_case("darkslategrey"),
                tag_no_case("darkslategray"),
                tag_no_case("darkslateblue"),
                tag_no_case("darkseagreen"),
                tag_no_case("darksalmon"),
                tag_no_case("darkred"),
                tag_no_case("darkorchid"),
                tag_no_case("darkorange"),
                tag_no_case("darkolivegreen"),
                tag_no_case("darkmagenta"),
            )),
            alt((
                tag_no_case("darkkhaki"),
                tag_no_case("darkgrey"),
                tag_no_case("darkgreen"),
                tag_no_case("darkgray"),
                tag_no_case("darkgoldenrod"),
                tag_no_case("darkcyan"),
                tag_no_case("darkblue"),
                tag_no_case("cyan"),
                tag_no_case("crimson"),
                tag_no_case("cornsilk"),
                tag_no_case("cornflowerblue"),
                tag_no_case("coral"),
                tag_no_case("chocolate"),
                tag_no_case("chartreuse"),
                tag_no_case("cadetblue"),
                tag_no_case("burlywood"),
                tag_no_case("brown"),
                tag_no_case("blueviolet"),
                tag_no_case("blanchedalmond"),
                tag_no_case("bisque"),
                tag_no_case("beige"),
            )),
            alt((
                tag_no_case("azure"),
                tag_no_case("aquamarine"),
                tag_no_case("antiquewhite"),
                tag_no_case("aliceblue"),
            )),
            // CSS Level 2 (Revision 1).
            tag_no_case("orange"),
            // CSS Level 1.
            alt((
                tag_no_case("yellow"),
                tag_no_case("white"),
                tag_no_case("teal"),
                tag_no_case("silver"),
                tag_no_case("red"),
                tag_no_case("purple"),
                tag_no_case("olive"),
                tag_no_case("navy"),
                tag_no_case("maroon"),
                tag_no_case("lime"),
                tag_no_case("green"),
                tag_no_case("gray"),
                tag_no_case("fuchsia"),
                tag_no_case("blue"),
                tag_no_case("black"),
                tag_no_case("aqua"),
            )),
            // Hexadecimal notation.
            recognize(preceded(
                char('#'),
                alt((
                    many_m_n(8, 8, one_of("0123456789ABCDEFabcdef")),
                    many_m_n(6, 6, one_of("0123456789ABCDEFabcdef")),
                    many_m_n(4, 4, one_of("0123456789ABCDEFabcdef")),
                    many_m_n(3, 3, one_of("0123456789ABCDEFabcdef")),
                )),
            )),
        ))(input)
    }

    fn opening_tag(input: &str) -> IResult<&str, Token> {
        delimited(
            char('['),
            alt((
                map(tag("spoiler"), |_| Token::OpeningTag(OpeningTag::Spoiler)),
                map(
                    tuple((
                        tag("color="),
                        alt((
                            MessageParser::color,
                            delimited(char('"'), MessageParser::color, char('"')),
                        )),
                    )),
                    |(_, color)| {
                        Token::OpeningTag(OpeningTag::Color {
                            color: String::from(color),
                        })
                    },
                ),
                map(tag("codeblock"), |_| {
                    Token::OpeningTag(OpeningTag::CodeBlock)
                }),
                map(tag("code"), |_| Token::OpeningTag(OpeningTag::Code)),
                map(tag("sup"), |_| Token::OpeningTag(OpeningTag::Superscript)),
                map(tag("sub"), |_| Token::OpeningTag(OpeningTag::Subscript)),
                map(tag("b"), |_| Token::OpeningTag(OpeningTag::Bold)),
                map(tag("i"), |_| Token::OpeningTag(OpeningTag::Italic)),
                map(tag("u"), |_| Token::OpeningTag(OpeningTag::Underline)),
                map(tag("s"), |_| Token::OpeningTag(OpeningTag::Strike)),
            )),
            char(']'),
        )(input)
    }

    fn closing_tag(input: &str) -> IResult<&str, Token> {
        delimited(
            char('['),
            preceded(
                char('/'),
                alt((
                    map(tag("spoiler"), |_| Token::ClosingTag(ClosingTag::Spoiler)),
                    map(tag("color"), |_| Token::ClosingTag(ClosingTag::Color)),
                    map(tag("codeblock"), |_| {
                        Token::ClosingTag(ClosingTag::CodeBlock)
                    }),
                    map(tag("code"), |_| Token::ClosingTag(ClosingTag::Code)),
                    map(tag("sup"), |_| Token::ClosingTag(ClosingTag::Superscript)),
                    map(tag("sub"), |_| Token::ClosingTag(ClosingTag::Subscript)),
                    map(tag("b"), |_| Token::ClosingTag(ClosingTag::Bold)),
                    map(tag("i"), |_| Token::ClosingTag(ClosingTag::Italic)),
                    map(tag("u"), |_| Token::ClosingTag(ClosingTag::Underline)),
                    map(tag("s"), |_| Token::ClosingTag(ClosingTag::Strike)),
                )),
            ),
            char(']'),
        )(input)
    }

    fn link(input: &str) -> IResult<&str, Token> {
        map(
            tuple((
                alt((tag("http://"), tag("https://"))),
                is_not("/?# "),
                is_not("[?# "),
                opt(preceded(char('?'), is_not("[# "))),
                opt(preceded(char('#'), is_not("[ "))),
            )),
            |(scheme, host, path, query, fragment)| {
                let mut url: String = format!("{}{}{}", scheme, host, path);

                if let Some(query) = query {
                    url.push('?');
                    url.push_str(query);
                }

                if let Some(fragment) = fragment {
                    url.push('#');
                    url.push_str(fragment);
                }

                Token::Link(url)
            },
        )(input)
    }

    fn text(input: &str) -> IResult<&str, Token> {
        map(is_not("[> "), |s: &str| Token::Text(s.to_string()))(input)
    }

    fn ref_link(input: &str) -> IResult<&str, Token> {
        map(preceded(tag(">>"), is_a("0123456789")), |s: &str| {
            Token::RefLink(s.parse().unwrap())
        })(input)
    }

    fn inline(input: &str) -> IResult<&str, Vec<Token>> {
        many0(alt((
            MessageParser::closing_tag,
            MessageParser::opening_tag,
            MessageParser::ref_link,
            MessageParser::link,
            MessageParser::text,
            map(one_of("[> "), |c: char| Token::Text(c.to_string())),
        )))(input)
    }

    fn quote(input: &str) -> IResult<&str, Vec<Token>> {
        preceded(
            peek(alt((
                value((), pair(tag(">>"), not(digit1))),
                value((), pair(char('>'), not(char('>')))),
            ))),
            map(MessageParser::inline, |tokens| {
                let mut result = vec![Token::OpeningTag(OpeningTag::Quote)];
                result.append(&mut tokens.clone());
                result.push(Token::ClosingTag(ClosingTag::Quote));
                result
            }),
        )(input)
    }

    fn block(input: &str) -> IResult<&str, Vec<Token>> {
        alt((MessageParser::quote, MessageParser::inline))(input)
    }

    pub fn tokenize(input: &str) -> Result<Vec<Token>, &str> {
        let mut input = input.trim().to_string();
        input = input.replace("\r\n", "\n");

        let lines: Vec<Vec<Token>> = input
            .split("\n")
            .map(|s| match MessageParser::block(s) {
                Ok((_, o)) => o,
                _ => Vec::new(),
            })
            .map(|mut tokens| {
                tokens.push(Token::Text("\n".to_string()));
                tokens
            })
            .collect();

        let lines: Vec<Token> = lines.into_iter().flatten().collect();
        Ok(lines)
    }

    fn optimize_segments(segments: Vec<Segment>) -> Vec<Segment> {
        segments
            .into_iter()
            .fold(Vec::new(), |mut segments, segment| {
                let last = segments.pop();
                match last {
                    Some(last) => {
                        if last.tags == segment.tags && !last.has_ref_link() {
                            segments.push(Segment {
                                text: format!("{}{}", last.text, segment.text),
                                tags: last.tags,
                            });
                        } else {
                            segments.push(last);
                            segments.push(segment);
                        }
                    }
                    None => {
                        segments.push(segment);
                    }
                };

                segments
            })
    }

    pub fn to_segments(tokens: Vec<Token>) -> Vec<Segment> {
        let mut result = Vec::new();
        let mut active_styles = Vec::new();
        for token in tokens.into_iter() {
            match token {
                Token::Text(text) => {
                    if active_styles.contains(&SegmentStyle::CodeBlock) {
                        result.push(Segment {
                            text,
                            tags: vec![SegmentStyle::CodeBlock],
                        });
                    } else if active_styles.contains(&SegmentStyle::Code) {
                        result.push(Segment {
                            text,
                            tags: vec![SegmentStyle::Code],
                        });
                    } else {
                        result.push(Segment {
                            text,
                            tags: active_styles.clone(),
                        });
                    }
                }
                Token::RefLink(id) => {
                    let text = id.to_string();

                    if active_styles.contains(&SegmentStyle::CodeBlock) {
                        result.push(Segment {
                            text: format!(">>{}", text),
                            tags: vec![SegmentStyle::CodeBlock],
                        });
                    } else if active_styles.contains(&SegmentStyle::Code) {
                        result.push(Segment {
                            text: format!(">>{}", text),
                            tags: vec![SegmentStyle::Code],
                        });
                    } else {
                        let mut tags = active_styles.clone();
                        tags.push(SegmentStyle::RefLink { id });
                        result.push(Segment { text, tags });
                    }
                }
                Token::Link(url) => {
                    if active_styles.contains(&SegmentStyle::CodeBlock) {
                        result.push(Segment {
                            text: url,
                            tags: vec![SegmentStyle::CodeBlock],
                        });
                    } else if active_styles.contains(&SegmentStyle::Code) {
                        result.push(Segment {
                            text: url,
                            tags: vec![SegmentStyle::Code],
                        });
                    } else {
                        let mut tags = active_styles.clone();
                        tags.push(SegmentStyle::Link { url: url.clone() });
                        result.push(Segment { text: url, tags });
                    }
                }
                Token::OpeningTag(tag) => {
                    if active_styles.contains(&SegmentStyle::CodeBlock) {
                        result.push(Segment {
                            text: String::from(OpeningTag::to_string(&tag)),
                            tags: vec![SegmentStyle::CodeBlock],
                        });
                    } else if active_styles.contains(&SegmentStyle::Code) {
                        result.push(Segment {
                            text: String::from(OpeningTag::to_string(&tag)),
                            tags: vec![SegmentStyle::Code],
                        });
                    } else {
                        active_styles.push(SegmentStyle::from_opening_tag(&tag));
                    }
                }
                Token::ClosingTag(tag) => {
                    if active_styles.contains(&SegmentStyle::CodeBlock)
                        && tag != ClosingTag::CodeBlock
                    {
                        result.push(Segment {
                            text: String::from(ClosingTag::to_string(&tag)),
                            tags: vec![SegmentStyle::CodeBlock],
                        });
                    } else if active_styles.contains(&SegmentStyle::Code) && tag != ClosingTag::Code
                    {
                        result.push(Segment {
                            text: String::from(ClosingTag::to_string(&tag)),
                            tags: vec![SegmentStyle::Code],
                        });
                    } else {
                        if let Some(index) =
                            active_styles.iter().rposition(|item| item.is_pair(&tag))
                        {
                            active_styles.remove(index);
                        }
                    }
                }
            }
        }

        MessageParser::optimize_segments(result)
    }

    pub fn str_to_segments(input: &str) -> Vec<Segment> {
        let tokens = MessageParser::tokenize(input);
        match tokens {
            Ok(tokens) => MessageParser::to_segments(tokens),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosingTag, MessageParser, OpeningTag, SegmentStyle, Token};
    use crate::models::message_parser::Segment;

    #[test]
    fn tokenize_empty_string() {
        let input = "";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(Ok(vec!(Token::Text("\n".to_string()))), tokens);
    }

    #[test]
    fn tokenize_text() {
        let input = "Lorem ipsum dolor sit amet";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("dolor".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("sit".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("amet".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_quote() {
        let input = "> lorem ipsum";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Quote),
                Token::Text(">".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::ClosingTag(ClosingTag::Quote),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_ref_link() {
        let input = ">>12345";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(Token::RefLink(12345), Token::Text("\n".to_string()))),
            tokens
        );
    }

    #[test]
    fn tokenize_adjacent_ref_links() {
        let input = ">>123>>456";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::RefLink(123),
                Token::RefLink(456),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_adjacent_ref_link_and_text() {
        let input = ">>123 456";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::RefLink(123),
                Token::Text(" ".to_string()),
                Token::Text("456".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link() {
        let input = "http://localhost/";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_path() {
        let input = "http://localhost/path";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/path".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_query() {
        let input = "http://localhost/?query";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/?query".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_fragment() {
        let input = "http://localhost/#fragment";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/#fragment".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_path_and_query() {
        let input = "http://localhost/path?query";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/path?query".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_path_and_fragment() {
        let input = "http://localhost/path#fragment";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/path#fragment".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_query_and_fragment() {
        let input = "http://localhost/?query#fragment";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/?query#fragment".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_link_with_path_query_and_fragment() {
        let input = "http://localhost/path?query#fragment";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Link("http://localhost/path?query#fragment".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_opening_tag() {
        let input = "[spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Spoiler),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_closing_tag() {
        let input = "[/code]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::ClosingTag(ClosingTag::Code),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_tag_pair() {
        let input = "[b][/b]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Bold),
                Token::ClosingTag(ClosingTag::Bold),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_tag_pair_with_text() {
        let input = "[i]Lorem ipsum dolor sit amet[/i]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Italic),
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("dolor".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("sit".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("amet".to_string()),
                Token::ClosingTag(ClosingTag::Italic),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_adjacent_tags() {
        let input = "[spoiler][code]Lorem ipsum[/code][/spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Spoiler),
                Token::OpeningTag(OpeningTag::Code),
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::ClosingTag(ClosingTag::Code),
                Token::ClosingTag(ClosingTag::Spoiler),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_short_hex() {
        let input = "[color=#ABC]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Color {
                    color: "#ABC".to_string()
                }),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_short_hex_with_alpha() {
        let input = "[color=#ABCD]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Color {
                    color: "#ABCD".to_string()
                }),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_hex() {
        let input = "[color=#ABCDEF]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Color {
                    color: "#ABCDEF".to_string()
                }),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_hex_with_alpha() {
        let input = "[color=#ABCDEFAB]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Color {
                    color: "#ABCDEFAB".to_string()
                }),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_invalid_color_hex() {
        let input = "[color=#ABCDEFG]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("[".to_string()),
                Token::Text("color=#ABCDEFG]".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_unknown_opening_tag() {
        let input = "[lorem]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("[".to_string()),
                Token::Text("lorem]".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_unknown_closing_tag() {
        let input = "[/lorem]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("[".to_string()),
                Token::Text("/lorem]".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_unopened_tag() {
        let input = "spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("spoiler]".to_string()),
                Token::Text("\n".to_string())
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_unclosed_tag() {
        let input = "[spoiler";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("[".to_string()),
                Token::Text("spoiler".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_multiple_strings() {
        let input = "Lorem ipsum\ndolor sit amet";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::Text("\n".to_string()),
                Token::Text("dolor".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("sit".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("amet".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_multiple_strings_with_quote() {
        let input = "> Lorem ipsum\ndolor sit amet";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::OpeningTag(OpeningTag::Quote),
                Token::Text(">".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::ClosingTag(ClosingTag::Quote),
                Token::Text("\n".to_string()),
                Token::Text("dolor".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("sit".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("amet".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_multiple_strings_with_quote_and_ref_link() {
        let input = ">>12345\n> Lorem ipsum\ndolor sit amet";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::RefLink(12345),
                Token::Text("\n".to_string()),
                Token::OpeningTag(OpeningTag::Quote),
                Token::Text(">".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("Lorem".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("ipsum".to_string()),
                Token::ClosingTag(ClosingTag::Quote),
                Token::Text("\n".to_string()),
                Token::Text("dolor".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("sit".to_string()),
                Token::Text(" ".to_string()),
                Token::Text("amet".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_multiple_strings_with_ref_link() {
        let input = ">>12345\n12345";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(vec!(
                Token::RefLink(12345),
                Token::Text("\n".to_string()),
                Token::Text("12345".to_string()),
                Token::Text("\n".to_string()),
            )),
            tokens
        );
    }

    #[test]
    fn str_to_segments_text_without_tags() {
        let input = "Lorem ipsum dolor sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(Segment {
                text: "Lorem ipsum dolor sit amet\n".to_string(),
                tags: Vec::new(),
            }),
            tokens
        );
    }

    #[test]
    fn str_to_segments_tag_pair() {
        let input = "Lorem ip[b]sum dolor [/b]sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "Lorem ip".to_string(),
                    tags: Vec::new(),
                },
                Segment {
                    text: "sum dolor ".to_string(),
                    tags: vec!(SegmentStyle::Bold),
                },
                Segment {
                    text: "sit amet\n".to_string(),
                    tags: Vec::new(),
                }
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_nested_tag_pairs() {
        let input = "Lorem ip[b]sum [i]dolor[/i] sit[/b] amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "Lorem ip".to_string(),
                    tags: Vec::new(),
                },
                Segment {
                    text: "sum ".to_string(),
                    tags: vec!(SegmentStyle::Bold),
                },
                Segment {
                    text: "dolor".to_string(),
                    tags: vec!(SegmentStyle::Bold, SegmentStyle::Italic),
                },
                Segment {
                    text: " sit".to_string(),
                    tags: vec!(SegmentStyle::Bold),
                },
                Segment {
                    text: " amet\n".to_string(),
                    tags: Vec::new(),
                },
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_overlapping_tag_pairs() {
        let input = "Lorem ip[b]sum [i]dolor[/b] sit[/i] amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "Lorem ip".to_string(),
                    tags: Vec::new(),
                },
                Segment {
                    text: "sum ".to_string(),
                    tags: vec!(SegmentStyle::Bold),
                },
                Segment {
                    text: "dolor".to_string(),
                    tags: vec!(SegmentStyle::Bold, SegmentStyle::Italic),
                },
                Segment {
                    text: " sit".to_string(),
                    tags: vec!(SegmentStyle::Italic),
                },
                Segment {
                    text: " amet\n".to_string(),
                    tags: Vec::new(),
                },
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_unclosed_tag_pair() {
        let input = "Lorem ip[b]sum dolor sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "Lorem ip".to_string(),
                    tags: Vec::new(),
                },
                Segment {
                    text: "sum dolor sit amet\n".to_string(),
                    tags: vec!(SegmentStyle::Bold),
                },
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_text_with_quote() {
        let input = ">Lorem ipsum\ndolor sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: ">Lorem ipsum".to_string(),
                    tags: vec!(SegmentStyle::Quote),
                },
                Segment {
                    text: "\ndolor sit amet\n".to_string(),
                    tags: Vec::new(),
                }
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_text_with_ref_link() {
        let input = ">>12345\nLorem ipsum dolor sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "12345".to_string(),
                    tags: vec!(SegmentStyle::RefLink { id: 12345 }),
                },
                Segment {
                    text: "\nLorem ipsum dolor sit amet\n".to_string(),
                    tags: Vec::new(),
                }
            ),
            tokens
        );
    }

    #[test]
    fn str_to_segments_text_with_quote_and_ref_link() {
        let input = ">>12345\n>Lorem ipsum\ndolor sit amet";
        let tokens = MessageParser::str_to_segments(input);
        assert_eq!(
            vec!(
                Segment {
                    text: "12345".to_string(),
                    tags: vec!(SegmentStyle::RefLink { id: 12345 }),
                },
                Segment {
                    text: "\n".to_string(),
                    tags: Vec::new(),
                },
                Segment {
                    text: ">Lorem ipsum".to_string(),
                    tags: vec!(SegmentStyle::Quote),
                },
                Segment {
                    text: "\ndolor sit amet\n".to_string(),
                    tags: Vec::new(),
                }
            ),
            tokens
        );
    }
}
