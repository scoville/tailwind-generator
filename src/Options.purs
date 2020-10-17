module Options (Options(..), options) where

import Prelude
import Data.Either (Either(..))
import Lang (Lang(..))
import Verbosity (Verbosity(..))
import Node.Globals (__dirname)
import Options.Applicative (Parser, ParserInfo, ReadM, eitherReader, header, help, helper, info, long, metavar, option, short, showDefault, strOption, value, (<**>))

type Options
  = { config :: String
    , lang :: Lang
    , output :: String
    , cssOutput :: String
    , cssInput :: String
    , verbosity :: Verbosity
    }

langParser :: ReadM Lang
langParser =
  eitherReader case _ of
    "purescript" -> Right PureScript
    "reasonml" -> Right ReasonML
    "elm" -> Right Elm
    "typescript" -> Right TypeScript
    "typescript-type-level" -> Right TypeScriptTypeLevel
    lang -> Left $ "\"" <> lang <> "\" is not a valid lang"

verbosityParser :: ReadM Verbosity
verbosityParser =
  eitherReader case _ of
    "silent" -> Right Silent
    "info" -> Right Info
    "debug" -> Right Debug
    mode -> Left $ "\"" <> mode <> "\" is not a valid verbosity level"

optionsParser :: Parser Options
optionsParser = ado
  config <-
    strOption
      ( long "config"
          <> short 'c'
          <> metavar "FILEPATH"
          <> value ""
          <> help "tailwind.config.js path"
      )
  lang <-
    option langParser
      ( long "lang"
          <> short 'l'
          <> metavar "LANG"
          <> help "Language used in generated code (elm|reasonml|typescript|typescript-type-level|purescript)"
      )
  output <-
    strOption
      ( long "output"
          <> short 'o'
          <> metavar "DIRPATH"
          <> showDefault
          <> value "./src"
          <> help "Directory for generated code"
      )
  cssOutput <-
    strOption
      ( long "cssOutput"
          <> metavar "FILEPATH"
          <> showDefault
          <> value "./tailwind.css"
          <> help "Provide full path (including file name) for generated css stylesheet"
      )
  cssInput <-
    strOption
      ( long "cssInput"
          <> metavar "FILEPATH"
          <> showDefault
          <> (value $ __dirname <> "/assets/input.css")
          <> help "Provide path of your css stylesheet which uses the @tailwind directive to inject Tailwind's preflight and utilities styles into your CSS"
      )
  verbosity <-
    option verbosityParser
      ( long "verbosity"
          <> short 'v'
          <> metavar "VERBOSITY"
          <> value Info
          <> showDefault
          <> help "Set verbosity level (silent|info|debug)"
      )
  in { config, lang, output, cssOutput, cssInput, verbosity }

options :: ParserInfo Options
options = info (optionsParser <**> helper) $ header "Generates code and css from a tailwind config file."
