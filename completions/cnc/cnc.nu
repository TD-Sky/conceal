module completions {

  # Put entities into the recyle bin
  export extern cnc [
    ...entities: string       # The entities to be discarded
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

}

use completions *
