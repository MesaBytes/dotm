#! /usr/bin/python

import sys
import getpass
from configparser import ConfigParser
from os import (makedirs, path)
from library.color import color

args = sys.argv[1:]
username = getpass.getuser()
parser = ConfigParser()
default_config = {'dotfiles_path': "None"}

class Config:
  ''' create config file and change options '''
  def __init__(self):
    self.config_file_path = f"/home/{username}/.config/dotm/dotm.conf"
    self.config_dir_path = f"/home/{username}/.config/dotm/"
  
  def file_exists(self):
    file_exists = path.exists(self.config_file_path)
    if not file_exists: return False
    return True

  def dir_exists(self): 
    dir_exists = path.exists(self.config_dir_path)
    if not dir_exists: return False
    return True

  def create_file(self): 
    if not self.dir_exists(): self.create_dir()
    if self.file_exists():
      answer  = str(input("You are going to overwrite your configuration, Do you want to continue [yes/no]: "))
      if answer == 'no' or answer == 'n': return
    
    parser["main"] = default_config
    with open(self.config_file_path, 'w') as configfile:
      parser.write(configfile)
    configfile.close()
    print("[+] Config file '% s' created" % self.config_file_path)

  def create_dir(self): 
    if self.dir_exists(): return
    makedirs(self.config_dir_path)
    print("[+] Directory '% s' created" % self.config_dir_path)

  def set(self, section, option, value): 
    if not self.file_exists(): self.create_file()

    parser.read_file(open(self.config_file_path))

    parser.set(section, option, value)
    parser.write(open(self.config_file_path, 'w'))

  def get(self, section, option): 
    if not self.file_exists(): self.create_file()

    parser.read_file(open(self.config_file_path))
    return parser.get(section, option)

config = Config()

def help():
  print("--help, -h\t help message")
  print("--create, -C\t create new config file")

def initial_setup():
  dotfiles_path = str(input("Your dotfiles directory path (Absolute Path): "))
  if not path.exists(dotfiles_path):
    print(f"'{dotfiles_path}' is not a valid directory!")
    sys.exit(1)

  config.set("main", "dotfiles_path", dotfiles_path)
  sys.exit(0)

def main():
  if "--help" in args or "-h" in args: help(); sys.exit(0)
  if not config.file_exists(): initial_setup()
  elif "--create" in args or "-C" in args: config.create_file(); sys.exit(0)

if __name__ == "__main__":
  main()

