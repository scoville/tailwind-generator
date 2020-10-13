module Verbosity (Verbosity(..)) where

import Prelude

data Verbosity
  = Silent
  | Info
  | Debug

instance showVerbosity :: Show Verbosity where
  show v = case v of
    Silent -> "silent"
    Info -> "info"
    Debug -> "debug"
