use anyhow::{anyhow, Result};
use pest::Parser;

use md_parser::*;


#[test]
fn test_parse_digit() -> Result<()> {
    let input = "5";
    let pair = MarkdownParser::parse(Rule::digit, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_bold() -> Result<()> {
    let input = "**bold text**";
    let pair = MarkdownParser::parse(Rule::bold, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_italic() -> Result<()> {
    let input = "*italic text*";
    let pair = MarkdownParser::parse(Rule::italic, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_bold_and_italic() -> Result<()> {
    let input = "***bold and italic***";
    let pair = MarkdownParser::parse(Rule::bold_italic, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_fail_on_invalid_header() {
    let pair = MarkdownParser::parse(Rule::header, "####### Not a h1-h6\n");
    assert!(pair.is_err(), "Parser incorrectly accepted #######");
}

#[test]
fn test_fail_on_mismatched_bold() {
    let pair = MarkdownParser::parse(Rule::bold, "**missing closure");
    assert!(pair.is_err(), "Parser did not fail on unclosed **");
}


#[test]
fn test_parse_unordered() -> Result<()> {
    let input = "* ";
    let pair = MarkdownParser::parse(Rule::unordered, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_ordered() -> Result<()> {
    let input = "1. ";
    let pair = MarkdownParser::parse(Rule::ordered, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_list_start() -> Result<()> {
    let input = "- ";
    let pair = MarkdownParser::parse(Rule::list_start, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}


#[test]
fn test_parse_header_start() -> Result<()> {
    let input = "# ";
    let pair = MarkdownParser::parse(Rule::header_start, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_line_content() -> Result<()> {
    let input = "**bold** and *italic* text";
    let pair = MarkdownParser::parse(Rule::line_content, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_line_with_literal_asterisk() -> Result<()> {
    let input = "a * b and c#";
    let pair = MarkdownParser::parse(Rule::line_content, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_list_point() -> Result<()> {
    let input = "* List point\n";
    let pair = MarkdownParser::parse(Rule::list_point, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);
    Ok(())
}

#[test]
fn test_parse_header() -> Result<()> {
    let input = "# Hello World\n";
    let pair = MarkdownParser::parse(Rule::header, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_paragraph() -> Result<()> {
    let input = "This is a paragraph\nSecond line\n";
    let pair = MarkdownParser::parse(Rule::paragraph, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);

    Ok(())
}

#[test]
fn test_parse_document() -> Result<()> {
    let input = "# Hello World\n* List point\n";

    let pair = MarkdownParser::parse(Rule::document, input)?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    assert_eq!(pair.as_str(), input);
    assert_eq!(pair.as_span().end(), input.len());
    
    Ok(())
}
