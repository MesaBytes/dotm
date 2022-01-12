#! /usr/bin/python

import sys
import getpass
from ext.manage_config import Config
from os import path
from ext.color import color
from ext.manage_dotfiles_list import Dotfiles_list_manager
from ext.core_dotm import backup

username = getpass.getuser()
args = sys.argv[1:]
config = Config()
dotfiles_list_manager = Dotfiles_list_manager()

def help_message():
  print(f'''usage: dotm [options]\n
{color.light.yellow('options:')}
  {color.light.green('-h, --help')}\t help message
  {color.light.green('-v, --version')}\t print dotm version number
  {color.light.green('-b, --backup')}\t backup files listed in dotfiles list
  {color.light.green('-c, --change')}\t change dotfiles directory 
  {color.light.green('-C, --create')}\t create new config file
  {color.light.green('-l, --list')}\t print dotfiles list
  {color.light.green('-e, --empty-list')}\t empty dotfiles list
  {color.light.green('-a, --add')}\t add new dotfile path
  {color.light.green('-r, --remove')}\t remove dotfile from the list using it's id
''')

def initial_setup():
  if not path.exists(config.config_file_path):
    dotfiles_path = str(input(f"Your dotfiles directory path ({color.bold('Absolute Path')}): "))
    if "~/" in dotfiles_path: dotfiles_path = dotfiles_path.replace("~/", f"/home/{username}/")
    if not path.exists(dotfiles_path):
      print(f"{color.selected(dotfiles_path)} is {color.light.red('not a valid directory!')}")
      return 1
    if dotfiles_path[-1] != '/': dotfiles_path += '/'
    config.set("main", "dotfiles_path", dotfiles_path)

  if not path.exists(dotfiles_list_manager.dotfiles_list_path):
    dotfiles_list_manager.create_file()

def main() -> int:
  def parse_args(option, alias): return '--'+option in args or '-'+alias in args

  if not config.file_exists() or not dotfiles_list_manager.file_exists(): initial_setup()
  if parse_args("help", "h") or len(args) == 0: help_message()
  elif parse_args("version", "v"): print("dotm 1.1.0")
  elif parse_args("backup", "b"): backup()
  elif parse_args("change", "c"): 
    dotfiles_path = str(input(f"Your dotfiles directory path ({color.bold('Absolute Path')}): "))
    if "~/" in dotfiles_path: dotfiles_path = dotfiles_path.replace("~/", f"/home/{username}/")
    if not path.exists(dotfiles_path):
      print(f"{color.selected(dotfiles_path)} is {color.light.red('not a valid directory!')}")
      return 1
    if dotfiles_path[-1] != '/': dotfiles_path += '/'
    config.set("main", "dotfiles_path", dotfiles_path)
  elif parse_args("create", "C"): config.create_file()
  elif parse_args("list", "l"): dotfiles_list_manager.print_list()
  elif parse_args("empty-list", "e"): dotfiles_list_manager.empty_list()
  elif parse_args("remove", "r"): 
    for arg in args:
      if arg.startswith('-'):
        args.remove(arg)

    if not args:
      print("Please provide id!")
      return 1
      
    return dotfiles_list_manager.remove(args)
  elif parse_args("add", "a"): 
    for arg in args:
      if arg.startswith('-'):
        args.remove(arg)

    try:
      destination = args[-1]
      sources = args[:-1]
    except IndexError:
      print(color.light.red("You need to provide a source and a destination!")+"\n")
      print(f"{color.light.grey('Example')}: dotm --add {color.light.green('path/to/source')} {color.light.green('path/to/destination')}")
      return 1

    for i in range(len(sources)):
      if not path.exists(sources[i]):
        print(f"[{color.light.red('!')}] {sources[i]} {color.light.red('does not exists!')}")
        return 1
      
      if "~/" in sources[i]: sources[i].replace("~/", f"/home/{username}/")
      if sources[i][-1] == '/': sources[i] = sources[i][:-1]

    if destination[-1] != '/': destination += "/"    
    return dotfiles_list_manager.add(sources, destination)

if __name__ == "__main__":
  sys.exit(main())
