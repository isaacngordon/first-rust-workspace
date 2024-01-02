def main [] {
    let plugins: list<string> = (open Cargo.toml | get workspace.members)
    echo $"Building and installing plugins: ($plugins)"
    
    for plugin in $plugins {
        cd $plugin
        cargo install --path .
        echo $"Installed plugin: ($plugin)"
        cd .. 
    }

    for plugin in $plugins {
        echo $"run `register /target/release/($plugin)` to register ($plugin)"
    }

}