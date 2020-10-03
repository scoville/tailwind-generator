module Utils (catchAndKill, kill, tailwindBuild, mkdirp) where

import Prelude
import Control.Promise (Promise, toAffE)
import Effect (Effect)
import Effect.Aff (Error, catchError)
import Effect.Aff.Class (class MonadAff, liftAff)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Class.Console (log)
import Effect.Exception (catchException)
import Effect.Uncurried (EffectFn1, EffectFn3, runEffectFn1, runEffectFn3)
import Node.Process (exit)

kill :: forall a m. MonadEffect m => Error -> m a
kill error = do
  log $ "An error occured: " <> show error
  liftEffect $ exit 1

catchAndKill :: forall a m. MonadEffect m => Effect a -> m a
catchAndKill = liftEffect <<< catchException (liftEffect <<< kill)

foreign import _tailwindBuild :: EffectFn3 String String String (Promise Unit)

tailwindBuild :: forall m. MonadAff m => String -> String -> String -> m Unit
tailwindBuild config cssInput = liftAff <<< flip catchError kill <<< toAffE <<< runEffectFn3 _tailwindBuild config cssInput

foreign import _mkdirp :: EffectFn1 String Unit

mkdirp :: forall m. MonadEffect m => String -> m Unit
mkdirp = liftEffect <<< runEffectFn1 _mkdirp
