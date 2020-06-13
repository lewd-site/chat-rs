use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{char, digit1, one_of};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, preceded};
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

    pub fn opening(&self) -> &str {
        match self {
            Tag::Bold => "[b]",
            Tag::Italic => "[i]",
            Tag::Underline => "[u]",
            Tag::Strike => "[s]",
            Tag::Superscript => "[sup]",
            Tag::Subscript => "[sub]",
            Tag::Code => "[code]",
            Tag::CodeBlock => "[codeblock]",
            Tag::Spoiler => "[spoiler]",
            Tag::RefLink { id: _ } => ">>",
            Tag::Quote => ">",
        }
    }

    pub fn closing(&self) -> &str {
        match self {
            Tag::Bold => "[/b]",
            Tag::Italic => "[/i]",
            Tag::Underline => "[/u]",
            Tag::Strike => "[/s]",
            Tag::Superscript => "[/sup]",
            Tag::Subscript => "[/sub]",
            Tag::Code => "[/code]",
            Tag::CodeBlock => "[/codeblock]",
            Tag::Spoiler => "[/spoiler]",
            Tag::RefLink { id: _ } => "",
            Tag::Quote => "",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Text(String),
    RefLink(u32),
    Quote(String),
    OpeningTag(Tag),
    ClosingTag(Tag),
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Segment {
    pub text: String,
    pub tags: Vec<Tag>,
}

impl Segment {
    pub fn is_ref_link(&self) -> bool {
        self.tags.iter().position(|tag| tag.is_ref_link()) != None
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

    fn opening_tag(input: &str) -> IResult<&str, Token> {
        delimited(
            char('['),
            alt((
                map(tag("spoiler"), |_| Token::OpeningTag(Tag::Spoiler)),
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
                    map(tag("spoiler"), |_| Token::ClosingTag(Tag::Spoiler)),
                    map(tag("codeblock"), |_| Token::ClosingTag(Tag::CodeBlock)),
                    map(tag("code"), |_| Token::ClosingTag(Tag::Code)),
                    map(tag("sup"), |_| Token::ClosingTag(Tag::Superscript)),
                    map(tag("sub"), |_| Token::ClosingTag(Tag::Subscript)),
                    map(tag("b"), |_| Token::ClosingTag(Tag::Bold)),
                    map(tag("i"), |_| Token::ClosingTag(Tag::Italic)),
                    map(tag("u"), |_| Token::ClosingTag(Tag::Underline)),
                    map(tag("s"), |_| Token::ClosingTag(Tag::Strike)),
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
                            text: String::from(Tag::opening(&tag)),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) {
                        result.push(Segment {
                            text: String::from(Tag::opening(&tag)),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        active_tags.push(tag);
                    }
                }
                Token::ClosingTag(tag) => {
                    if active_tags.contains(&Tag::CodeBlock) && tag != Tag::CodeBlock {
                        result.push(Segment {
                            text: String::from(Tag::closing(&tag)),
                            tags: vec![Tag::CodeBlock],
                        });
                    } else if active_tags.contains(&Tag::Code) && tag != Tag::Code {
                        result.push(Segment {
                            text: String::from(Tag::closing(&tag)),
                            tags: vec![Tag::Code],
                        });
                    } else {
                        if let Some(index) = active_tags.iter().rposition(|item| *item == tag) {
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
    use super::{MessageParser, Tag, Token};
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
        assert_eq!(Ok(("", vec!(Token::ClosingTag(Tag::Code)))), tokens);
    }

    #[test]
    fn tokenize_tag_pair() {
        let input = "[b][/b]";
        let tokens = MessageParser::tokenize(input);
        assert_eq!(
            Ok((
                "",
                vec!(Token::OpeningTag(Tag::Bold), Token::ClosingTag(Tag::Bold))
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
                    Token::ClosingTag(Tag::Italic)
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
                    Token::ClosingTag(Tag::Code),
                    Token::ClosingTag(Tag::Spoiler),
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
            Ok(("", vec!(Token::Text(String::from("[spoiler"))))),
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
