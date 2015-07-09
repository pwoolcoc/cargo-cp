# cargo-cp

Copy rust files to and from the playground/gists

## Example usage

Create a new project based off a playground link

```bash
$ cargo cp http://is.gd/deadbeef my-new-project
Creating `my-new-project`
Initializing with content from `http://is.gd/deadbeef`
$
```

or

```bash
$ cargo cp abcd1234beef my-new-project
Creating `my-new-project`
Initializing with content from `https://api.github.com/gists/abcd1234beef`
```

Copy a module in your current project to the playground

```bash
~/my-new-project $ cargo cp somemodule
Uploading `src/somemodule.rs` to play.rust-lang.org...http://is.gd/abc123
~/my-new-project $ # or to a gist instead
~/my-new-project $ cargo cp somemodule --gist
Uploading `src/somemodule.rs` to gist.github.com...https://gist.github.com/abc123
```

Copy lib.rs to a gist

```bash
~/my-new-project $ cargo cp --gist
Uploading `src/lib.rs` to gist.github.com...https://gist.github.com/123abc
```

