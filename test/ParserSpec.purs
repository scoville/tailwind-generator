module ParserSpec where

import Prelude
import Assert (Css(..), Ast(..), shouldBeParsedAs)
import Test.Spec (Spec, describe, it)

spec :: Spec Unit
spec =
  describe "Parser" do
    it "should parse CSS and generate proper pseudo ast" do
      Css "./test/snapshots/all-classes.css"
        `shouldBeParsedAs`
          Ast "./test/snapshots/all-classes.ast"
