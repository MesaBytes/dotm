#! /usr/bin/python

import sys
from ext.manage_config import Config
from os import path
from ext.color import color
from ext.manage_dotfiles_list import Dotfiles_list_manager

args = sys.argv[1:]
config = Config()
dotfiles_list_manager = Dotfiles_list_manager()

def help_message():
  print(f'''usage: dotm [options]\n
{color.light.yellow('options:')}
  {color.light.green('-h, --help')}\t help message
  {color.light.green('-C, --create')}\t create new config file
  {color.light.green('-l, --list')}\t print dotfiles list
  {color.light.green('-a, --add')}\t add new dotfile path
  {color.light.green('-r, --remove')}\t remove dotfile from the list using it's id
''')

def initial_setup():
  if not path.exists(config.config_file_path):
    dotfiles_path = str(input(f"Your dotfiles directory path ({color.bold('Absolute Path')}): "))
    if not path.exists(dotfiles_path):
      print(f"{color.selected(dotfiles_path)} is {color.light.red('not a valid directory!')}")
      sys.exit(1)
    if dotfiles_path[-1] != '/': dotfiles_path += '/'
    config.set("main", "dotfiles_path", dotfiles_path)

  if not path.exists(dotfiles_list_manager.dotfiles_list_path):
    dotfiles_list_manager.create_file()

  sys.exit(0)

def main():
  if not config.file_exists() or not dotfiles_list_manager.file_exists(): initial_setup()
  if "--help" in args or "-h" in args: help_message(); sys.exit(0)
  elif "--create" in args or "-C" in args: config.create_file(); sys.exit(0)

if __name__ == "__main__":
  main()
