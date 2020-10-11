module Parser (Node, PseudoAst(..), parseFromFile) where

import Prelude
import Data.Either (Either)
import Data.Foldable (fold)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.Maybe (maybe)
import Data.Newtype (class Newtype)
import Data.String.CodeUnits (singleton)
import Data.String.Extra (camelCase)
import Data.Traversable (sequence)
import Effect.Class (class MonadEffect, liftEffect)
import Foreign.Generic (class Decode, defaultOptions, genericDecode)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Node.Path (FilePath)
import Text.Parsing.StringParser (ParseError, runParser)
import Text.Parsing.StringParser.CodeUnits (alphaNum, char, string)
import Text.Parsing.StringParser.Combinators (choice, many1, optionMaybe)
import Utils (getClassNames)

type Node
  = { className :: String, name :: String }

newtype PseudoAst
  = PseudoAst (Array Node)

derive newtype instance eqPseudoAst :: Eq PseudoAst

derive newtype instance ordPseudoAst :: Ord PseudoAst

derive instance newtypePseudoAst :: Newtype PseudoAst _

derive instance genericPseudoAst :: Generic PseudoAst _

instance showPseudoAst :: Show PseudoAst where
  show = genericShow

instance decodePseudoAst :: Decode PseudoAst where
  decode = genericDecode defaultOptions { unwrapSingleConstructors = true }

nodeFromClassName :: String -> Either ParseError Node
nodeFromClassName className = ({ className, name: _ }) <$> camelCase <$> runParser nameParser className
  where
  nameParser = ado
    neg <- maybe "" (const "neg-") <$> optionMaybe (char '-')
    name <- fold <$> many1 anyValidAndSpecialChar
    in neg <> name

  anyValidAndSpecialChar =
    choice
      [ const "-neg-" <$> string "\\\\:-"
      , const "-over-" <$> string "\\\\/"
      , const "-" <$> string "\\\\:"
      , singleton <$> alphaNum
      , string "-"
      ]

parseFromFile :: forall m. MonadEffect m => FilePath -> m (Either ParseError PseudoAst)
parseFromFile cssPath = do
  cssContent <- liftEffect $ readTextFile UTF8 cssPath
  pure $ PseudoAst <$> (sequence $ nodeFromClassName <$> getClassNames cssContent cssPath)
