#!/usr/bin/env -S nu --stdin

def main [] {}

def 'main build' [
    --target: string,
] {
    if ((uname | get operating-system) =~ 'Linux') and (which '^zip' | is-empty) {
        apt-get update
        apt-get install -yq zip
    }

    let dir = $"conceal-($target)"
    mkdir -v $dir

    cargo build --target $target --release
    gen-completions --artifacts-dir $dir

    let exe_ext = if ((uname | get operating-system) =~ 'Windows') { '.exe' } else { '' }
    cp -v $"target/($target)/release/cnc($exe_ext)" $dir
    cp -v $"target/($target)/release/conceal($exe_ext)" $dir

    cp LICENSE $dir
}

def gen-completions [
    --artifacts-dir: path,
] {
    cargo build --release

    let dir = [$artifacts_dir, 'completions'] | path join
    mkdir -v $dir
    let targets = [
        { shell: 'bash', file: 'conceal.bash' },
        { shell: 'zsh', file: '_conceal' },
        { shell: 'nu', file: 'conceal.nu' },
        { shell: 'fish', file: 'conceal.fish' },
        { shell: 'powershell', file: '_conceal.ps1' },
    ]

    for target in $targets {
        ./target/release/conceal completion --shell $target.shell out> ($dir | path join $target.file)
    }
}
