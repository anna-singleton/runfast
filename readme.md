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

Personally, I use it with a tmux bind, since I am almost always in a tmux
session. See the `.tmux.config` in my
[dotfiles repo](https://github.com/anna-singleton/dotfiles) for an example.
You may see it referred to as `quickrun` in a few places, since that what it was
intially called.
