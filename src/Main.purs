module Main where

import Prelude
import AppM (runAppM)
import Control.Monad.Reader (ask)
import Effect (Effect)
import Effect.Aff (launchAff_)
import Effect.Class (liftEffect)
import Control.Monad.Logger.Class (info)
import Options (options)
import Data.Log.Tag (tag)
import Options.Applicative (execParser)
import Printer (save) as Printer
import Data.Map (empty)
import Utils (catchAndKill, mkdirp, tailwindBuild)

main :: Effect Unit
main = launchAff_ $ runAppM app =<< liftEffect (execParser options)
  where
  app = do
    { config, cssInput, cssOutput, output } <- ask
    info
      ( tag "Tailwind config file" config
          <> tag "Css input file" cssInput
          <> tag "Css output file" cssOutput
      )
      "Generating Tailwind css..."
    tailwindBuild config cssInput cssOutput
    info empty "Done generating Tailwind css."
    catchAndKill $ mkdirp output
    Printer.save
    info empty "Done"
