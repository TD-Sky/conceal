module completions {

  # A trash collector.
  export extern cnc [
    ...items: string          # The files to be discarded
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # [default] Throw the files in the trash bin
  export extern "cnc put" [
    ...items: string          # The files to be discarded
    --help(-h)                # Print help
  ]

  # List all the discarded files
  export extern "cnc list" [
    --help(-h)                # Print help
  ]

  # Restore files discarded under the current directory
  export extern "cnc restore" [
    --help(-h)                # Print help
  ]

  # Delete all the discarded files permanently
  export extern "cnc empty" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "cnc help" [
  ]

  # [default] Throw the files in the trash bin
  export extern "cnc help put" [
  ]

  # List all the discarded files
  export extern "cnc help list" [
  ]

  # Restore files discarded under the current directory
  export extern "cnc help restore" [
  ]

  # Delete all the discarded files permanently
  export extern "cnc help empty" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "cnc help help" [
  ]

}

use completions *
