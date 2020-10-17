module Printer (save) where

import Prelude
import Control.Monad.Reader (class MonadAsk, ask, asks)
import Control.Monad.Logger.Class (class MonadLogger)
import Data.Either (either)
import Data.Traversable (traverse)
import Effect.Class (class MonadEffect)
import Effect.Exception (error)
import Lang (Lang(..))
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile, writeTextFile)
import Node.Globals (__dirname)
import Node.Path (FilePath)
import Parser (PseudoAst(..), parseFromFile)
import Text.Handlebars as Handlebars
import Utils (catchAndKill, kill)

formatFromFile ::
  forall r m.
  MonadEffect m =>
  MonadAsk { cssOutput :: String | r } m =>
  FilePath -> m String
formatFromFile outputFile =
  asks _.cssOutput
    >>= parseFromFile
    >>= traverse formatFile
    >>= either (kill <<< error <<< show) pure
  where
  formatFile (PseudoAst nodes) = do
    template <- catchAndKill $ readTextFile UTF8 $ __dirname <> "/templates/" <> outputFile <> ".hbs"
    pure $ Handlebars.compile template { nodes }

-- FIXME: Normalize and resolve path
save ::
  forall r m.
  MonadEffect m =>
  MonadLogger m =>
  MonadAsk { cssOutput :: String, lang :: Lang, output :: String | r } m => m Unit
save = do
  { lang, output } <- ask
  case lang of
    PureScript -> formatFromFile "purs" >>= saveFile output "Tailwind.purs"
    Elm -> formatFromFile "elm" >>= saveFile output "Tailwind.elm"
    ReasonML -> do
      formatFromFile "re" >>= saveFile output "tailwind.re"
      formatFromFile "rei" >>= saveFile output "tailwind.rei"
    TypeScript -> formatFromFile "ts" >>= saveFile output "tailwind.ts"
    TypeScriptTypeLevel -> formatFromFile "ts-type-level" >>= saveFile output "tailwind.ts"
  where
  saveFile output path = catchAndKill <<< writeTextFile UTF8 (output <> "/" <> path)
