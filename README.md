### README
I don’t want to manually manage aliases in my bash or zsh shell.

For example, I don’t want to add something like `alias gcm='git checkout main'` in my ~/.bashrc.

There are many such aliases I use, so I built this tool to handle them automatically.

It creates those aliases as symlinks in the package.

### Installation
Currently, this package works only on Ubuntu systems. Please visit the [Releases](https://github.com/srinivasreddy/reddy/releases) section of this repository and download the version you want — preferably the latest. Then install it using:
```
sudo dpkg -i reddy_0.0.1.deb
```
