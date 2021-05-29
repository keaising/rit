## rit

### clone to arbitrary place

You can set a local folder, rit will clone Github/Gitlab repo into
this folder according to the repo's url.

For example, local folder is `$HOME/code`, use 

```shell
rit clone https://github.com/keaising/rit
rit clone git@github.com:keaising/rit.git
rit clone https://github.com/keaising/rit.git
```

all of them will execute commands below, no matter what $PWD is

```shell
git clone git@github.com:keaising/rit.git $HOME/code/github.com/keaising/rit
```

### open current folder in web

If $PWD is a git repo, open current folder or file in git repo's website.

Support these conditions:

1. newest commit in current/default branch, 
2. current commit,
3. blame info of current file.

WIP
