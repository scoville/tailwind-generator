module AppM (AppM, runAppM) where

import Prelude
import Control.Monad.Error.Class (class MonadError, class MonadThrow)
import Control.Monad.Logger.Class (class MonadLogger)
import Control.Monad.Logger.Trans (LoggerT, runLoggerT)
import Control.Monad.Reader (class MonadAsk, ReaderT, asks, runReaderT)
import Data.Log.Filter (minimumLevel)
import Data.Log.Formatter.Pretty (prettyFormatter)
import Data.Log.Level (LogLevel(..))
import Data.Newtype (class Newtype)
import Effect.Aff (Aff, Error)
import Effect.Aff.Class (class MonadAff)
import Effect.Class (class MonadEffect, liftEffect)
import Effect.Console as Console
import Options (Options)
import Type.Equality (class TypeEquals, from)
import Verbosity as Verbosity

newtype AppM a
  = AppM (LoggerT (ReaderT Options Aff) a)

derive instance newtypeAppM :: Newtype (AppM a) _

derive newtype instance functorAppM :: Functor AppM

derive newtype instance applyAppM :: Apply AppM

derive newtype instance applicativeAppM :: Applicative AppM

derive newtype instance bindAppM :: Bind AppM

derive newtype instance monadAppM :: Monad AppM

derive newtype instance monadThrowAppM :: MonadThrow Error AppM

derive newtype instance monadErrorAppM :: MonadError Error AppM

derive newtype instance monadEffectAppM :: MonadEffect AppM

derive newtype instance monadAffAppM :: MonadAff AppM

derive newtype instance monadLoggerAppM :: MonadLogger AppM

instance monadAskAppM :: TypeEquals o Options => MonadAsk o AppM where
  ask = AppM $ asks from

runAppM :: forall a. AppM a -> Options -> Aff a
runAppM (AppM m) options = runReaderT (runLoggerT m (filterByVerbosity logMessage)) options
  where
  filterByVerbosity = case options.verbosity of
    Verbosity.Info -> minimumLevel Info
    Verbosity.Debug -> minimumLevel Debug
    Verbosity.Silent -> const $ const $ pure unit

  logMessage message = liftEffect $ prettyFormatter message >>= Console.log
