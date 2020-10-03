module Main where

import Prelude
import AppM (runAppM)
import Control.Monad.Reader (ask)
import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Class (liftEffect)
import Effect.Class.Console (log)
import Options (options)
import Options.Applicative (execParser)
import Printer (save) as Printer
import Utils (catchAndKill, mkdirp, tailwindBuild)

main :: Effect Unit
main = launchAff_ $ runAppM app =<< liftEffect (execParser options)
  where
  app = do
    { config, cssInput, cssOutput, output } <- ask
    tailwindBuild config cssInput cssOutput
    catchAndKill $ mkdirp output
    Printer.save
    log "Done"
