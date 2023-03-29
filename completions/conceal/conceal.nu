module completions {

  # Operate the recycle bin
  export extern conceal [
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # List the discarded entities
  export extern "conceal list" [
    --all(-a)                 # All discarded entities. If not, only list the entities discarded from current directory
    --help(-h)                # Print help
  ]

  # Restore entities discarded from the current directory
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

  # List the discarded entities
  export extern "conceal help list" [
  ]

  # Restore entities discarded from the current directory
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
