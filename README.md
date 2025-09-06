# Spellbook

A repository template for build comfort dev environments.

---

<!-- toc -->

- [Installation](#installation)
- [Usage](#usage)
- [Gitignore](#gitignore)
- [Dependencies](#dependencies)

<!-- tocstop -->

---

## Installation

Clone repository

```shell
git clone https://github.com/oberon-systems/spellbook
```

Navigate into it

```shell
cd spellbook
```

Run installer

```shell
./spell install
```

## Usage

Create a new directory

```shell
mkdir ./example
```

Init git repo in it

```shell
git init
```
Enable env, it will be created at firs time

```shell
spell
```

If there will be some conflicts then you will be asked!

Add required files and/or directories into `.gitignore` file
and commit changes.

Work normally...

## Gitignore

I stringly recommend to add next excludes to `.gitignore`

```.gitignore
.pre-commit
.venv
.spellbook
```

## Dependencies

- jq
- tar
- git
- gem
- bash
- curl
- ruby
- ruby-devel
- make

In a container you may have to install some system spec files, e.g.

```shell
dnf install redhat-rpm-config
```
