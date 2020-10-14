module Assert where

import Prelude
import Control.Monad.Error.Class (class MonadThrow, throwError)
import Control.Monad.Except (runExcept)
import Data.Array (sort)
import Data.Either (Either(..))
import Data.Newtype (over)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Exception (Error, error)
import Foreign.Generic (decodeJSON)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Node.Path (FilePath)
import Parser (PseudoAst(..))
import Parser as Parser
import Test.Spec.Assertions (shouldEqual)

newtype Css
  = Css FilePath

newtype Ast
  = Ast FilePath

sortPseudoAst :: PseudoAst -> PseudoAst
sortPseudoAst = over PseudoAst sort

shouldBeParsedAs ::
  forall m.
  MonadThrow Error m =>
  MonadEffect m =>
  Css -> Ast -> m Unit
shouldBeParsedAs (Css cssPath) (Ast astPath) = do
  liftEffect $ readTextFile UTF8 astPath <#> runExcept <<< decodeJSON
    >>= case _ of
        Left _ -> throwError $ error "Not a valid AST file"
        Right ast ->
          Parser.parseFromFile cssPath
            >>= case _ of
                Left _ -> throwError $ error "Not a valid CSS file"
                Right ast' -> sortPseudoAst ast `shouldEqual` sortPseudoAst ast'
