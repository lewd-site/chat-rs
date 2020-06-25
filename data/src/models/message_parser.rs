use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::{map, recognize};
use nom::multi::{many0, many_m_n};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "type")]
pub enum Tag {
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
    Quote,
}

impl Tag {
    pub fn is_ref_link(&self) -> bool {
        match self {
            Tag::RefLink { id: _ } => true,
            _ => false,
        }
    }

    pub fn get_ref_link(&self) -> Option<u32> {
        match self {
            Tag::RefLink { id } => Some(*id),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Tag::Bold => String::from("[b]"),
            Tag::Italic => String::from("[i]"),
            Tag::Underline => String::from("[u]"),
            Tag::Strike => String::from("[s]"),
            Tag::Superscript => String::from("[sup]"),
            Tag::Subscript => String::from("[sub]"),
            Tag::Code => String::from("[code]"),
            Tag::CodeBlock => String::from("[codeblock]"),
            Tag::Spoiler => String::from("[spoiler]"),
            Tag::Color { color } => format!("[color={}]", color),
            Tag::RefLink { id: _ } => String::from(">>"),
            Tag::Quote => String::from(">"),
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
}

impl ClosingTag {
    pub fn is_closing_for(&self, tag: &Tag) -> bool {
        match (tag, self) {
            (Tag::Bold, ClosingTag::Bold) => true,
            (Tag::Italic, ClosingTag::Italic) => true,
            (Tag::Underline, ClosingTag::Underline) => true,
            (Tag::Strike, ClosingTag::Strike) => true,
            (Tag::Superscript, ClosingTag::Superscript) => true,
            (Tag::Subscript, ClosingTag::Subscript) => true,
            (Tag::Code, ClosingTag::Code) => true,
            (Tag::CodeBlock, ClosingTag::CodeBlock) => true,
            (Tag::Spoiler, ClosingTag::Spoiler) => true,
            (Tag::Color { color: _ }, ClosingTag::Color) => true,
            _ => false,
        }
    }
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
        })?;

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Text(String),
    RefLink(u32),
    Quote(String),
    OpeningTag(Tag),
    ClosingTag(ClosingTag),
}

#[derive(Debug, PartialEq, Eq, Serialize, Clone)]
pub struct Segment {
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Segment {
    pub fn is_ref_link(&self) -> bool {
        self.tags.iter().position(|tag| tag.is_ref_link()) != None
    }

    pub fn get_ref_link(&self) -> Option<u32> {
        let index = self.tags.iter().position(|tag| tag.is_ref_link());
        index.and_then(|i| self.tags[i].get_ref_link())
    }
}

pub struct MessageParser();

impl MessageParser {
    fn text(input: &str) -> IResult<&str, Token> {
        alt((
            map(preceded(tag(">>"), digit1), |s: &str| {
                Token::RefLink(s.parse().unwrap())
            }),
            map(preceded(char('>'), is_not("\r\n")), |s: &str| {
                Token::Quote(String::from(s))
            }),
            map(is_not("[>"), |s: &str| Token::Text(String::from(s))),
            // Parse incomplete tags as text.
            map(one_of("[>"), |ch: char| Token::Text(ch.to_string())),
        ))(input)
    }

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
                map(tag("spoiler"), |_| Token::OpeningTag(Tag::Spoiler)),
                map(
                    tuple((
                        tag("color="),
                        alt((
                            MessageParser::color,
                            delimited(char('"'), MessageParser::color, char('"')),
                        )),
                    )),
                    |(_, color)| {
                        Token::OpeningTag(Tag::Color {
                            color: String::from(color),
                        })
                    },
                ),
                map(tag("codeblock"), |_| Token::OpeningTag(Tag::CodeBlock)),
                map(tag("code"), |_| Token::OpeningTag(Tag::Code)),
                map(tag("sup"), |_| Token::OpeningTag(Tag::Superscript)),
                map(tag("sub"), |_| Token::OpeningTag(Tag::Subscript)),
                map(tag("b"), |_| Token::OpeningTag(Tag::Bold)),
                map(tag("i"), |_| Token::OpeningTag(Tag::Italic)),
                map(tag("u"), |_| Token::OpeningTag(Tag::Underline)),
                map(tag("s"), |_| Token::OpeningTag(Tag::Strike)),
                map(is_not("]"), |s| Token::Text(format!("[{}]", s))),
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
                    map(is_not("]"), |s| Token::Text(format!("[/{}]", s))),
                )),
            ),
            char(']'),
        )(input)
    }

    pub fn tokenize(i: &str) -> IResult<&str, Vec<Token>> {
        many0(alt((
            MessageParser::closing_tag,
            MessageParser::opening_tag,
            MessageParser::text,
        )))(i)
    }

    fn optimize_segments(segments: Vec<Segment>) -> Vec<Segment> {
        segments
            .into_iter()
            .fold(Vec::new(), |mut segments, segment| {
                let last = segments.pop();
                match last {
                    Some(last) => {
                        if last.tags == segment.tags && !last.is_ref_link() {
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
        let mut active_tags = Vec::new();
        for token in tokens.into_iter() {
            match token {
                Token::Text(text) => {
                    if active_tags.contains(&Tag::CodeBlock) {
                        result.push(Segment {
                            text,
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) {
                        result.push(Segment {
                            text,
                            tags: vec![Tag::Code],
                        });
                    } else {
                        result.push(Segment {
                            text,
                            tags: active_tags.clone(),
                        });
                    }
                }
                Token::RefLink(id) => {
                    let text = id.to_string();

                    if active_tags.contains(&Tag::CodeBlock) {
                        result.push(Segment {
                            text: format!(">>{}", text),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) {
                        result.push(Segment {
                            text: format!(">>{}", text),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        let mut tags = active_tags.clone();
                        tags.push(Tag::RefLink { id });
                        result.push(Segment { text, tags });
                    }
                }
                Token::Quote(text) => {
                    if active_tags.contains(&Tag::CodeBlock) {
                        result.push(Segment {
                            text: format!(">{}", text),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) {
                        result.push(Segment {
                            text: format!(">{}", text),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        let mut tags = active_tags.clone();
                        tags.push(Tag::Quote);
                        result.push(Segment { text, tags });
                    }
                }
                Token::OpeningTag(tag) => {
                    if active_tags.contains(&Tag::CodeBlock) {
                        result.push(Segment {
                            text: String::from(Tag::to_string(&tag)),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) {
                        result.push(Segment {
                            text: String::from(Tag::to_string(&tag)),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        active_tags.push(tag);
                    }
                }
                Token::ClosingTag(tag) => {
                    if active_tags.contains(&Tag::CodeBlock) && tag != ClosingTag::CodeBlock {
                        result.push(Segment {
                            text: String::from(ClosingTag::to_string(&tag)),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) && tag != ClosingTag::Code {
                        result.push(Segment {
                            text: String::from(ClosingTag::to_string(&tag)),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        if let Some(index) = active_tags
                            .iter()
                            .rposition(|item| tag.is_closing_for(item))
                        {
                            active_tags.remove(index);
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
            Ok((_, tokens)) => MessageParser::to_segments(tokens),
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosingTag, MessageParser, Tag, Token};
    use crate::models::message_parser::Segment;

    #[test]
    fn tokenize_empty_string() {
        let input = "";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(Ok(("", Vec::new())), tokens);
    }

    #[test]
    fn tokenize_text() {
        let input = "Lorem ipsum dolor sit amet";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::Text(String::from("Lorem ipsum dolor sit amet")))
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_opening_tag() {
        let input = "[spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(Ok(("", vec!(Token::OpeningTag(Tag::Spoiler)))), tokens);
    }

    #[test]
    fn tokenize_closing_tag() {
        let input = "[/code]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(Ok(("", vec!(Token::ClosingTag(ClosingTag::Code)))), tokens);
    }

    #[test]
    fn tokenize_tag_pair() {
        let input = "[b][/b]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(
                    Token::OpeningTag(Tag::Bold),
                    Token::ClosingTag(ClosingTag::Bold)
                )
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_tag_pair_with_text() {
        let input = "[i]Lorem ipsum dolor sit amet[/i]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(
                    Token::OpeningTag(Tag::Italic),
                    Token::Text(String::from("Lorem ipsum dolor sit amet")),
                    Token::ClosingTag(ClosingTag::Italic)
                )
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_adjacent_tags() {
        let input = "[spoiler][code][/code][/spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(
                    Token::OpeningTag(Tag::Spoiler),
                    Token::OpeningTag(Tag::Code),
                    Token::ClosingTag(ClosingTag::Code),
                    Token::ClosingTag(ClosingTag::Spoiler),
                )
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_short_hex() {
        let input = "[color=#ABC]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::OpeningTag(Tag::Color {
                    color: String::from("#ABC")
                }))
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_short_hex_with_alpha() {
        let input = "[color=#ABCD]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::OpeningTag(Tag::Color {
                    color: String::from("#ABCD")
                }))
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_hex() {
        let input = "[color=#ABCDEF]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::OpeningTag(Tag::Color {
                    color: String::from("#ABCDEF")
                }))
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_color_hex_with_alpha() {
        let input = "[color=#ABCDEFAB]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::OpeningTag(Tag::Color {
                    color: String::from("#ABCDEFAB")
                }))
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_invalid_color_hex() {
        let input = "[color=#ABCDEFG]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(
                    Token::Text(String::from("[")),
                    Token::Text(String::from("color=#ABCDEFG]"))
                )
            )),
            tokens
        );
    }

    #[test]
    fn tokenize_unknown_opening_tag() {
        let input = "[lorem]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(Ok(("", vec!(Token::Text(String::from("[lorem]"))))), tokens);
    }

    #[test]
    fn tokenize_unknown_closing_tag() {
        let input = "[/lorem]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(("", vec!(Token::Text(String::from("[/lorem]"))))),
            tokens
        );
    }

    #[test]
    fn tokenize_unopened_tag() {
        let input = "spoiler]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok(("", vec!(Token::Text(String::from("spoiler]"))))),
            tokens
        );
    }

    #[test]
    fn tokenize_unclosed_tag() {
        let input = "[spoiler";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(
                    Token::Text(String::from("[")),
                    Token::Text(String::from("spoiler"))
                )
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
                text: String::from("Lorem ipsum dolor sit amet"),
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
                    text: String::from("Lorem ip"),
                    tags: Vec::new(),
                },
                Segment {
                    text: String::from("sum dolor "),
                    tags: vec!(Tag::Bold),
                },
                Segment {
                    text: String::from("sit amet"),
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
                    text: String::from("Lorem ip"),
                    tags: Vec::new(),
                },
                Segment {
                    text: String::from("sum "),
                    tags: vec!(Tag::Bold),
                },
                Segment {
                    text: String::from("dolor"),
                    tags: vec!(Tag::Bold, Tag::Italic),
                },
                Segment {
                    text: String::from(" sit"),
                    tags: vec!(Tag::Bold),
                },
                Segment {
                    text: String::from(" amet"),
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
                    text: String::from("Lorem ip"),
                    tags: Vec::new(),
                },
                Segment {
                    text: String::from("sum "),
                    tags: vec!(Tag::Bold),
                },
                Segment {
                    text: String::from("dolor"),
                    tags: vec!(Tag::Bold, Tag::Italic),
                },
                Segment {
                    text: String::from(" sit"),
                    tags: vec!(Tag::Italic),
                },
                Segment {
                    text: String::from(" amet"),
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
                    text: String::from("Lorem ip"),
                    tags: Vec::new(),
                },
                Segment {
                    text: String::from("sum dolor sit amet"),
                    tags: vec!(Tag::Bold),
                },
            ),
            tokens
        );
    }
}
