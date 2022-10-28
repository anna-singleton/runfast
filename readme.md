# runfast

## What is it?

This is a program intended to be run in a project directory to set up a project
run command, and remember it so we dont have to type it multiple times. It
should run in a single terminal window, and be lightweight. It should also be
easy for end-users to add more commands to the default list. There should also
be a custom option, for one-off situations.


## Why?

I hate typing things out multiple times, and doing more things that are not the
work increases my chance of getting distracted, therefore a single command
allows me to think less, do more.


## How do I use it?

add the binary to your path, and call `runfast` in your project directory.
It will automatically create a default config at
`~/.config/runfast/defaults.toml`. You can add your own runners in a similar
manner in `~/.config/runfast/runners.toml`. When running in a new directory,
runfast will get you to choose a runner, and cache what you choose. This means
you dont have to select the same one each time. However, you can also call
`runfast -f` to force runfast to re-choose and re-cache the runner for a
directory.

Additionally, I have mine firing off a `tmux` bind, see my dotfiles repo, also
hosted on github, for an example.
