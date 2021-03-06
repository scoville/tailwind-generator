{ name = "tailwind-generator"
, dependencies =
  [ "aff-promise"
  , "console"
  , "effect"
  , "foreign-generic"
  , "generics-rep"
  , "handlebars"
  , "monad-logger"
  , "node-fs"
  , "node-path"
  , "optparse"
  , "promises"
  , "psci-support"
  , "string-parsers"
  , "strings-extra"
  ]
, packages = ./packages.dhall
, sources = [ "src/**/*.purs" ]
}
