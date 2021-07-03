use anyhow::Result;
use cssparser::{
    AtRuleParser, AtRuleType, BasicParseErrorKind, CowRcStr, ParseError, Parser, ParserState,
    QualifiedRuleParser, Token,
};

pub struct ClassesParser;

impl<'i> QualifiedRuleParser<'i> for ClassesParser {
    type Prelude = String;
    type QualifiedRule = String;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let ret = if let Ok(Token::Delim('.')) = input.next() {
            if let Ok(Token::Ident(ident)) = input.next() {
                Ok(ident.to_string())
            } else {
                Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
            }
        } else {
            Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
        };

        // Consume the rest of the input
        while let Ok(_token) = input.next() {
            continue;
        }

        ret
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        // Consume the block input
        while let Ok(_token) = input.next() {
            continue;
        }

        Ok(prelude)
    }
}

impl<'i> AtRuleParser<'i> for ClassesParser {
    type PreludeNoBlock = ();
    type PreludeBlock = ();
    type AtRule = String;
    type Error = ();

    #[allow(clippy::type_complexity)]
    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<AtRuleType<Self::PreludeNoBlock, Self::PreludeBlock>, ParseError<'i, Self::Error>>
    {
        match (name.to_string().as_str(), input.next()) {
            ("media", Ok(Token::ParenthesisBlock)) => Ok(AtRuleType::WithBlock(())),
            _ => Ok(AtRuleType::WithoutBlock(())),
        }
    }

    fn parse_block<'t>(
        &mut self,
        mut _prelude: Self::PreludeBlock,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, ()>> {
        let ret = if let Ok(Token::Delim('.')) = input.next() {
            if let Ok(Token::Ident(ident)) = input.next() {
                Ok(ident.to_string())
            } else {
                Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
            }
        } else {
            Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid))
        };

        while let Ok(_token) = input.next() {
            continue;
        }

        ret
    }
}
