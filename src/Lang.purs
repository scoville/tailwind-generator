module Lang (Lang(..)) where

import Prelude
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)

data Lang
  = PureScript
  | Elm
  | ReasonML
  | TypeScript
  | TypeScriptTypeLevel
  | TypeScriptTypeLevel2

derive instance eqLang :: Eq Lang

derive instance genericLang :: Generic Lang _

instance showLang :: Show Lang where
  show = genericShow
