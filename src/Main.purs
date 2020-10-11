module Main where

import Prelude
import AppM (runAppM)
import Control.Monad.Reader (ask)
import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Class (liftEffect)
import Control.Monad.Logger.Class (info)
import Options (options)
import Options.Applicative (execParser)
import Printer (save) as Printer
import Data.Map (empty)
import Utils (catchAndKill, mkdirp, tailwindBuild)

main :: Effect Unit
main = launchAff_ $ runAppM app =<< liftEffect (execParser options)
  where
  app = do
    { config, cssInput, cssOutput, output } <- ask
    info empty "Building Tailwind css"
    tailwindBuild config cssInput cssOutput
    catchAndKill $ mkdirp output
    Printer.save
    info empty "Done"
