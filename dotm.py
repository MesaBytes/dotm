#! /usr/bin/python

import sys
from functions.manage_config import Config
from functions.is_valid_dir import isValidDir
from library.color import color

args = sys.argv[1:]
config = Config()

def help():
  print("--help, -h\t help message")
  print("--create, -C\t create new config file")

def initial_setup():
  dotfiles_path = str(input(f"Your dotfiles directory path ({color.bold('Absolute Path')}): "))
  if isValidDir(dotfiles_path) == False:
    print(f"{color.selected(dotfiles_path)} is {color.light.red('not a valid directory!')}")
    sys.exit(1)

  if dotfiles_path[-1] != '/': dotfiles_path += '/'

  config.set("main", "dotfiles_path", dotfiles_path)
  sys.exit(0)

def main():
  if "--help" in args or "-h" in args: help(); sys.exit(0)
  if not config.file_exists(): initial_setup()
  elif "--create" in args or "-C" in args: config.create_file(); sys.exit(0)

if __name__ == "__main__":
  main()

