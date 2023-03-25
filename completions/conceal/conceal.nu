module completions {

  # Operate the recycle bin
  export extern conceal [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # List all the discarded entities
  export extern "conceal list" [
    --help(-h)                # Print help
  ]

  # Restore entities discarded under the current directory
  export extern "conceal restore" [
    --help(-h)                # Print help
  ]

  # Delete all the discarded entities permanently
  export extern "conceal clean" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "conceal help" [
  ]

  # List all the discarded entities
  export extern "conceal help list" [
  ]

  # Restore entities discarded under the current directory
  export extern "conceal help restore" [
  ]

  # Delete all the discarded entities permanently
  export extern "conceal help clean" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "conceal help help" [
  ]

}

use completions *
