module AppM (AppM, runAppM) where

import Prelude
import Control.Monad.Error.Class (class MonadError, class MonadThrow)
import Control.Monad.Reader (class MonadAsk, ReaderT, asks, runReaderT)
import Data.Newtype (class Newtype)
import Effect.Aff (Aff, Error)
import Effect.Aff.Class (class MonadAff)
import Effect.Class (class MonadEffect)
import Options (Options)
import Type.Equality (class TypeEquals, from)

newtype AppM a
  = AppM (ReaderT Options Aff a)

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

instance monadAskAppM :: TypeEquals o Options => MonadAsk o AppM where
  ask = AppM $ asks from

runAppM :: forall a. AppM a -> Options -> Aff a
runAppM (AppM m) options = runReaderT m options
