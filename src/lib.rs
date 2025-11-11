use pest_derive::Parser;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkdownParser;

#[derive(Error, Debug)]
pub enum MarkdownError {
    #[error("Parse error at {line}:{column}: {source}")]
    ParseError {
        line: usize,
        column: usize,
        #[source]
        source: pest::error::Error<Rule>,
    },
    #[error("Invalid structure in {rule}: expected {expected} but got {actual}")]
    InvalidStructure {
        rule: String,
        expected: String,
        actual: String,
    },
}

pub fn parse_to_html(markdown: &str) -> Result<String, MarkdownError> {
    match MarkdownParser::parse(Rule::document, markdown) {
        Ok(pairs) => {
            let mut html = String::new();
            for pair in pairs {
                html.push_str(&convert_pair_to_html(pair)?);
            }
            Ok(html)
        }
        Err(e) => {
            match e.line_col {
                pest::error::LineColLocation::Pos((line, column)) => {                                                                         
                    Err(MarkdownError::ParseError {
                            line,
                            column,
                            source: e,
                        })
                    }
                pest::error::LineColLocation::Span((start_line, start_col), (_end_line, _end_col)) => {
                    Err(MarkdownError::ParseError {
                        line: start_line,
                        column: start_col,
                        source: e,
                    })
                }
            }
        }
    }
}

fn convert_inner_to_html(pair: pest::iterators::Pair<Rule>) -> Result<String, MarkdownError> {
    let mut html = String::new();
    for child in pair.into_inner() {
        html.push_str(&convert_pair_to_html(child)?);
    }
    Ok(html)
}

fn convert_pair_to_html(pair: pest::iterators::Pair<Rule>) -> Result<String, MarkdownError> {
    match pair.as_rule() {
        Rule::document |
        Rule::block |
        Rule::paragraph_line |
        Rule::line_content => {
            convert_inner_to_html(pair)
        }

        Rule::header => {
            let mut inner = pair.into_inner();
            let header_start = inner.next().unwrap(); 
            let line_content = inner.next().unwrap();

            let level = header_start.as_str().trim().len();
            let content = convert_pair_to_html(line_content)?;

            Ok(format!("<h{}>{}</h{}>\n", level, content, level))
        }
        Rule::unordered_list => {
            let items = convert_inner_to_html(pair)?;
            Ok(format!("<ul>\n{}\n</ul>\n", items.trim()))
        }
        Rule::ordered_list => {
            let items = convert_inner_to_html(pair)?;
            Ok(format!("<ol>\n{}\n</ol>\n", items.trim()))
        }
        Rule::unordered_list_point | Rule::ordered_list_point => {
            let content = convert_inner_to_html(pair)?;
            Ok(format!("<li>{}</li>\n", content.trim()))
        }
        Rule::paragraph => {
            let content = convert_inner_to_html(pair)?;
            Ok(format!("<p>{}</p>\n", content.trim()))
        }
        Rule::bold_italic => {
            let inner_html = convert_inner_to_html(pair)?;
            Ok(format!("<strong><em>{}</em></strong>", inner_html))
        }
        Rule::bold => {
            let inner_html = convert_inner_to_html(pair)?;
            Ok(format!("<strong>{}</strong>", inner_html))
        }
        Rule::italic => {
            let inner_html = convert_inner_to_html(pair)?;
            Ok(format!("<em>{}</em>", inner_html))
        }
        Rule::char => {
            Ok(pair.as_str().to_string())
        }
        Rule::WHITESPACE => {
            Ok(pair.as_str().to_string())
        }
        _ => {
            convert_inner_to_html(pair)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_header() {
        let markdown = "# Hello World\n";
        let html = parse_to_html(markdown).unwrap();
        assert_eq!(html, "<h1>Hello World</h1>\n");
    }

    #[test]
    fn test_parse_bold_text() {
        let markdown = "This is **bold** text\n";
        let html = parse_to_html(markdown).unwrap();
        assert_eq!(html, "<p>This is <strong>bold</strong> text</p>\n");
    }

    #[test]
    fn test_parse_italic_text() {
        let markdown = "This is *italic* text\n";
        let html = parse_to_html(markdown).unwrap();
        assert_eq!(html, "<p>This is <em>italic</em> text</p>\n");
    }

    #[test]
    fn test_parse_unordered_list() {
        let markdown = "- Point 1\n- Point 2\n";
        let html = parse_to_html(markdown).unwrap();
        assert!(html.contains("<ul>"));
        assert!(html.contains("</ul>"));
        assert!(html.contains("<li>Point 1</li>"));
        assert!(html.contains("<li>Point 2</li>"));
    }

    #[test]
    fn test_parse_ordered_list() {
        let markdown = "1. First point\n2. Second point\n";
        let html = parse_to_html(markdown).unwrap();
        assert!(html.contains("<ol>"));
        assert!(html.contains("</ol>"));
        assert!(html.contains("<li>First point</li>"));
        assert!(html.contains("<li>Second point</li>"));
    }

    #[test]
    fn test_parse_bold_italic_combined() {
        let markdown = "This is ***bold and italic*** text\n";
        let html = parse_to_html(markdown).unwrap();
        assert!(html.contains("<strong><em>bold and italic</em></strong>"));
    }
}