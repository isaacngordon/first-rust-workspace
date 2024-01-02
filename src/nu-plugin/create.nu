# Generate a .nu script that takes in a string `name`, runs "cargo new nu_plugin_${name}", move sinto the new directory and runs "cargo add nu-protocol nu-plugin", and then sets the contents of the generated main.ts to some template
def main [ name: string ] = {
    # Run "cargo new nu_plugin_${name}"
    cargo new $"nu_plugin_($name)" 

    # Move into the new directory
    cd $"nu_plugin_($name)"

    # add required dependencies
    cargo add nu-protocol nu-plugin

    # Set the contents of main.ts to a template
    cd src
    rm main.rs
    "Your template contents here" | save main.rs
    cd ..

    echo $"New plugin `nu_plugin_($name)` from default template."

    # verify that the plugin has been added to the workspace
    let members = (open ../Cargo.toml | get workspace.members)
    if ( $"nu_plugin_($name)" in $members == false) {
        echo $"(ansi red)Plugin not added to workspace(ansi reset)"
    }

    # return absolute path to the plugin
    $"(pwd)"
}
