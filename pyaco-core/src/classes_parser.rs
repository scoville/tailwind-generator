use std::collections::HashSet;

use compact_str::CompactString;
use cssparser::{
    AtRuleParser, BasicParseError, BasicParseErrorKind, CowRcStr, ParseError, Parser, ParserState,
    QualifiedRuleParser, Token,
};

use crate::Result;

pub struct ClassesParser;

impl<'i> QualifiedRuleParser<'i> for ClassesParser {
    type Prelude = Option<HashSet<CompactString>>;
    type QualifiedRule = Option<HashSet<CompactString>>;
    type Error = ();

    fn parse_prelude<'t>(
        &mut self,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        let mut classes = HashSet::new();

        loop {
            match input.next() {
                // Match a new potential class
                Ok(Token::Delim('.')) => {
                    if let Ok(Token::Ident(ident)) = input.next() {
                        classes.insert(ident.as_ref().into());
                    } else {
                        // TODO: We should provide a better error here and let the developer know
                        // that the css is probably ill-formatted.
                        return Err(input.new_error(BasicParseErrorKind::QualifiedRuleInvalid));
                    }
                }
                // Match any other token and ignore
                Ok(_) => continue,
                // Match end of input, break and return found classes if any
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => break,
                // Match any other error and return it
                Err(error) => return Err(error.into()),
            }
        }

        Ok(Some(classes))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::QualifiedRule, ParseError<'i, Self::Error>> {
        // Consume the block input
        while input.next().is_ok() {
            continue;
        }

        Ok(prelude)
    }
}

impl<'i> AtRuleParser<'i> for ClassesParser {
    // `true` if the @rule body should be parsed, `false` otherwise
    type Prelude = bool;
    type AtRule = Option<HashSet<CompactString>>;
    type Error = ();

    #[allow(clippy::type_complexity)]
    fn parse_prelude<'t>(
        &mut self,
        name: CowRcStr<'i>,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
        // Consume the rest of the input
        while input.next().is_ok() {
            continue;
        }

        // "media" is the only @rule we'll parse the body in the `parse_block` function.
        Ok(matches!(name.as_ref(), "media"))
    }

    fn parse_block<'t>(
        &mut self,
        prelude: Self::Prelude,
        _start: &ParserState,
        input: &mut Parser<'i, 't>,
    ) -> Result<Self::AtRule, ParseError<'i, ()>> {
        // The @rule should not be parsed any further
        if !prelude {
            // Consume the input
            while input.next().is_ok() {
                continue;
            }

            return Ok(None);
        }

        let mut ret = HashSet::new();

        loop {
            match input.next() {
                // Match a new potential class
                Ok(Token::Delim('.')) => {
                    if let Ok(Token::Ident(ident)) = input.next() {
                        ret.insert(ident.as_ref().into());
                    } else {
                        return Err(input.new_error(BasicParseErrorKind::AtRuleBodyInvalid));
                    }
                }
                // Match any other token and ignore
                Ok(_) => continue,
                // Match end of input, break and return found classes if any
                Err(BasicParseError {
                    kind: BasicParseErrorKind::EndOfInput,
                    ..
                }) => break,
                // Match any other error and return it
                Err(error) => return Err(error.into()),
            }
        }

        Ok(Some(ret))
    }

    // Simply ignores @rules without blocks, the implementation of this function
    // is required by the `cssparser` crate if a `AtRuleType::WithoutBlock` value
    // is returned at runtime by the `parse_prelude` function.
    fn rule_without_block(
        &mut self,
        _prelude: Self::Prelude,
        _start: &ParserState,
    ) -> Result<Self::AtRule, ()> {
        Ok(None)
    }
}
