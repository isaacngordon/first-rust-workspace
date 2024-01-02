def install_plugin [name: string] {
    echo $"Installing plugin: `($name)`"
    # cd $name
    # cargo install --path .
    # cd ..
}

def main [name?: string] {
    let plugins = open Cargo.toml | get workspace.members
    
    if ($name != null) {
        if ($name not-in $plugins) and ($"nu_plugin_($name)" not-in $plugins) {
            echo $"(ansi yellow)Neither plugin `($name)` nor `nu_plugin_($name)` were found in Carfo.toml members. Aborting...(ansi reset)"
            exit 1
        }
        # install single plugin
        if ($name in $plugins) {
            install_plugin $name
            echo $"Plugin ($name) installed."
            echo $"run `register /target/release/($name)` to register '($name)'."
            exit 0
        } else {
            install_plugin $"nu_plugin_($name)"
            echo $"Plugin `nu_plugin_($name)` installed."
            echo $"run `register /target/release/nu_plugin_($name)` to register 'nu_plugin_($name)'."
            exit 0
        }

    }
    
    # install all
    echo $"Building and installing all plugins: `($plugins)`"
    for plugin in $plugins { install_plugin $plugin }
    for plugin in $plugins {
        echo $"run `register /target/release/($plugin)` to register ($plugin)"
    }

}