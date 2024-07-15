# Bstow

A _bash_ stow is a _bogged down_ stow, but for my very personal use cases its a _better_ stow!

For now this script only lets you create a symlink farm with `bstow <source> <target>` where _source_ and _target_ are directories that will behave like so

cane

| source                                        | target                                            |
| --------------------------------------------- | ------------------------------------------------- |
| is where you keep your files                  | is where the files are linked to                  |
| wont get updated with nasty stuff from target | will be a battlefiled                             |
| can easily be a git repository                | has also other stuff that you dont want to commit |

Let's take the usual `dotfiles` and `.config` stow as a real world example:

### With gnu stow you're symlinking the subdirectories in dotfiles
- [x] you get "automatic updates" for new files in already stow'd subdirs
- [ ] you don't get "automatic updates" for new subdirs
- [ ] you also have bi-directional copies of the junk that certain programs (GIMP, retroarch, ...) put there
- [ ] you need to .gitignore the files with keys and hashes if you want to share your dots publicly

### With bstow you're creating directories and symlinking all the single files
- [ ] you've got to remember to update your dotfiles at every new file insertion...
- [ ] you still don't get "automatic updates" for new subdirs (that's unless you're up with file watcher of some sort)
- [x] there's no surprises: programs aren't gonna save stuff in your `dotfiles` so they'll stay clean and lean
- [x] forget about .gitignore


The only thing left to do is just making it faster with a REAL programming language 😤
