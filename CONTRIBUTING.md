# How to contribute to Diskonaut
You can contribute to `diskonaut` in several ways, all of which would be very much appreciated.
Following are a few guidelines that might help you find your way around.

Please note that Diskonaut has a Code of Conduct (you can find it in the root of the repository).
It is there so that we can have a safe and pleasant environment that encourages acceptance, participation and kindness.

## Have you found a bug?
If Diskonaut is not working for you, or you found something that is not behaving as it should, it would be great if you could let us know about it.
To do this, please open an issue in the repository and provide as much relevant information as possible. Ideally, also a way to reproduce this bug.
If you would like to try and fix this bug yourself, please open a pull request (for more information, see the section of this document regarding code contributions).

## Do you have an idea for a new feature?
`diskonaut` can always be improved, and one of the best ways to improve it is by having its users suggest new features and different ways it can behave.
If you'd like to make such a suggestion, please open an issue detailing it. In this issue others would be able to comment on the suggestion.
Finally, you or someone else would be able to implement this suggestion if it is decided to do so.

## Would you like to make a code contribution?
Code contributions to `diskonaut` are very welcome and encouraged. If you're unsure what to work on, a good place to start is the "Help Wanted" or "Good First Issue" tags in the issues of this repository.

Following is some information you might find useful regarding the particularities of contributing to `diskonaut`:

### Testing
Diskonaut uses automated integration tests to make sure everything is working as it should.
If you're adding new functionality, you might want to add new tests or adjust the existing ones.

These tests work by creating textual "snapshots" of how the UI looks in certain situations. One test can have several snapshots.

An example test would be:
1. Create a temporary folder with a few files and subfolders.
2. Run diskonaut on that folder and take a snapshot of its UI. Make sure that snapshot is identical to the snapshot stored in this repository for that test.
3. Enter one of the subfolders, take another snapshot and make sure it is identical to the second stored snapshot for that test.

To store and compare the snapshots, we use [`insta`](https://docs.rs/insta/0.8.1/insta/)

For an improved experience, you can install `cargo-insta` on your computer, which will allow you to review new snapshots - approving or rejecting them. Please see the insta documentation for more details.

### Code formatting
`diskonaut` uses [rustfmt](https://github.com/rust-lang/rustfmt) for code formatting. In most (all?) cases, if you have it installed on your computer, you can run `cargo fmt` in the project folder, and it will auto fix your code.
