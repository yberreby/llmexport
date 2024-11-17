# `llmexport`

## What is this?

A tool to help you flatten (a subset of) a Git repository into a LLM-friendly text format.

I mostly wrote it for my own use; however, improvements and feature requests are welcome.

## Usage

```shell
# Assuming the `llmexport` binary is in your $PATH

# Export to file
llmexport > flattened.txt

# Exclude some files using a glob
llmexport -i '**/*.log' > flattened.txt

# Export a subset of the repository
llmexport src > flattened.txt

# Export to clipboard: pipe stdout to your favorite utility
llmexport | wl-copy
```

## Why?

LLMs are great, but not magical. If you want them to help you effectively with a codebase / task, you need to feed them the right info. Since they deal with text, and not files, this means that one must take a codebase and its many files, and convert it into a sequential, text-only representation that is easy to digest by LLMs.

As a LLM power user, I've naturally been doing this for a long time in one way or another - with bash oneliners, with [gpt-repository-loader](https://github.com/mpoon/gpt-repository-loader), with a .zshrc function supporting extra features... but it got messy as I needed to tailor it to various usecases (excluding big files even if tracked by git; excluding binary/non-UTF8 content; providing more helpful context beyond the mere file contents; formatting things in a LLM-friendly way; etc).

So I took a few hours to rewrite the mess that was lying in my `.zshrc`, and as a bonus, **made it compatible with Windows** and automated the generation of binaries using GitHub CI/CD! You can grab Windows binaries from the [Releases tab](https://github.com/yberreby/llmexport/releases/).
