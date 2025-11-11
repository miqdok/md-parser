use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MarkdownParser;

pub fn parse_to_html(markdown: &str) -> Result<String, String> {
    match MarkdownParser::parse(Rule::document, markdown) {
        Ok(pairs) => {
            let mut html = String::new();
            for pair in pairs {
                html.push_str(&convert_pair_to_html(pair));
            }
            Ok(html)
        }
        Err(e) => Err(format!("Parse error: {}", e))
    }
}

fn convert_inner_to_html(pair: pest::iterators::Pair<Rule>) -> String {
    let mut html = String::new();
    for child in pair.into_inner() {
        html.push_str(&convert_pair_to_html(child));
    }
    html
}

fn convert_pair_to_html(pair: pest::iterators::Pair<Rule>) -> String {
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
            let content = convert_pair_to_html(line_content);

            format!("<h{}>{}</h{}>\n", level, content, level)
        }
        Rule::unordered_list => {
            let items = convert_inner_to_html(pair);
            format!("<ul>\n{}\n</ul>\n", items.trim())
        }
        Rule::ordered_list => {
            let items = convert_inner_to_html(pair);
            format!("<ol>\n{}\n</ol>\n", items.trim())
        }
        Rule::unordered_list_point => {
            let mut inner = pair.into_inner();
            inner.next().unwrap();
            let line_content = inner.next().unwrap();
            let content = convert_pair_to_html(line_content);
            format!("<li>{}</li>\n", content)
        }
        Rule::ordered_list_point => {
            let mut inner = pair.into_inner();
            inner.next().unwrap();
            let line_content = inner.next().unwrap();
            let content = convert_pair_to_html(line_content);
            format!("<li>{}</li>\n", content)
        }
        Rule::paragraph => {
            let content = convert_inner_to_html(pair);
            format!("<p>{}</p>\n", content.trim())
        }
        Rule::bold_italic => {
            let inner_html = convert_inner_to_html(pair);
            format!("<strong><em>{}</em></strong>", inner_html)
        }
        Rule::bold => {
            let inner_html = convert_inner_to_html(pair);
            format!("<strong>{}</strong>", inner_html)
        }
        Rule::italic => {
            let inner_html = convert_inner_to_html(pair);
            format!("<em>{}</em>", inner_html)
        }
        Rule::char => {
            pair.as_str().to_string()
        }
        Rule::WHITESPACE => {
            pair.as_str().to_string()
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