# Contributing to MVPR DAO for Casper

The following is a set of rules and guidelines for contributing to this repo. Please feel free to propose changes to this document in a pull request.

## Submitting issues

If you have questions about how to use MVPR DAO for Casper, please direct these to the related channels on the [Casper Discord server](https://discord.gg/caspernetwork).

### Guidelines
* Please search the existing issues first, it's likely that your issue was already reported or even fixed.
  - Go to the main page of the repository, click "issues" and type any word in the top search/command bar.
  - You can also filter by appending e. g. "state:open" to the search string.
  - More info on [search syntax within GitHub](https://help.github.com/articles/searching-issues)

## Contributing to MVPR DAO for Casper

All contributions to this repository are considered to be licensed under Apache License 2.0.

Workflow for bug fixes:
* Check open issues and unmerged pull requests to make sure the topic is not already covered elsewhere
* Fork the repository
* Do your changes on your fork
* Make sure to add or update relevant test cases
* Create a pull request, with a suitable title and description, referring to the related issue

Workflow for new features or enhancements:
* Check open issues and unmerged pull requests to make sure the topic is not already covered elsewhere
* Create an issue including all the reasoning for the new feature or enhancement, and engage in discussion
* Then fork the repo
* Do your changes on your fork
* Make sure to add or update relevant test cases
* Create a pull request, with a suitable title and description, referring to the related issue and the enhancement proposal

### Sign your work

We use the Developer Certificate of Origin (DCO) as an additional safeguard
for the MVPR DAO for Casper project. This is a well established and widely used
mechanism to assure contributors have confirmed their right to license
their contribution under the project's license.
Please read [developer-certificate-of-origin](https://github.com/make-software/dao-contracts/blob/develop/.github/developer-certificate-of-origin).
If you can certify it, then just add a line to every git commit message:

````
  Signed-off-by: Random J Developer <random@developer.example.org>
````

Use your real name (sorry, no pseudonyms or anonymous contributions).
If you set your `user.name` and `user.email` git configs, you can sign your
commit automatically with `git commit -s`. You can also use git [aliases](https://git-scm.com/book/tr/v2/Git-Basics-Git-Aliases)
like `git config --global alias.ci 'commit -s'`. Now you can commit with
`git ci` and the commit will be signed.

