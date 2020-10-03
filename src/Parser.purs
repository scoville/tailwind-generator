module Parser (PseudoAst(..), parseFromFile) where

import Prelude
import Control.Alt ((<|>))
import Data.Array (sort)
import Data.Either (Either)
import Data.Foldable (fold)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.List.NonEmpty (catMaybes)
import Data.Maybe (Maybe(..), isJust)
import Data.Newtype (class Newtype, over)
import Data.Set as Set
import Data.String (Pattern(..), Replacement(..), replace)
import Data.String.CodeUnits (singleton)
import Data.String.Extra (camelCase)
import Effect.Class (class MonadEffect, liftEffect)
import Foreign.Generic (class Decode, defaultOptions, genericDecode)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Node.Path (FilePath)
import Text.Parsing.StringParser (ParseError, Parser, runParser)
import Text.Parsing.StringParser.CodeUnits (alphaNum, anyChar, char, eof, oneOf, string)
import Text.Parsing.StringParser.Combinators (choice, lookAhead, many1, many1Till, manyTill, optionMaybe)

newtype PseudoAst
  = PseudoAst (Array { className :: String, name :: String })

derive newtype instance eqPseudoAst :: Eq PseudoAst

derive newtype instance ordPseudoAst :: Ord PseudoAst

derive instance newtypePseudoAst :: Newtype PseudoAst _

derive instance genericPseudoAst :: Generic PseudoAst _

instance showPseudoAst :: Show PseudoAst where
  show = genericShow

instance decodePseudoAst :: Decode PseudoAst where
  decode = genericDecode defaultOptions { unwrapSingleConstructors = true }

pseudoAstParser :: Parser PseudoAst
pseudoAstParser = PseudoAst <<< Set.toUnfoldable <<< Set.fromFoldable <$> nodesParser
  where
  nodesParser = catMaybes <$> nodeParser `many1Till` eof

  nodeParser = ado
    skipNotClass
    -- TODO: Cheating with camelCase here, let's move the logic into the name parser!
    name <- map camelCase <$> lookAhead nameParser
    -- TODO: It seems that handlebars doesn't keep the characters escaped, so we have to escape them manually
    className <- map (replace (Pattern "\\") (Replacement "\\\\")) <$> classNameParser
    in { className: _, name: _ } <$> className <*> name

  skipNotClass = anyChar `manyTill` (void (char '.') <|> eof)

  classNameParser = ado
    className <- parseFOrEof $ many1 anyValidChar
    end <- classNameEnd
    in if isJust end then className else Nothing

  nameParser = ado
    neg <- map (const "neg-") <$> optionMaybe (char '-')
    name <- parseFOrEof $ many1 anyValidAndSpecialChar
    end <- classNameEnd
    in if isJust end then neg <> name else Nothing
    where
    anyValidAndSpecialChar =
      choice
        [ const "-over-" <$> string "\\/"
        , const "-" <$> string "\\:"
        , anyValidChar
        ]

  anyValidChar =
    choice
      [ singleton <$> alphaNum
      , string "\\:"
      , string "-"
      , string "\\/"
      ]

  parseFOrEof parser = (Just <<< fold <$> parser) <|> (const Nothing <$> eof)

  classNameEnd = optionMaybe $ oneOf [ ':', ',', '{', ' ', '\n' ]

parseFromFile :: forall m. MonadEffect m => FilePath -> m (Either ParseError PseudoAst)
parseFromFile cssPath = runParser pseudoAstParser <$> (liftEffect $ readTextFile UTF8 cssPath)
