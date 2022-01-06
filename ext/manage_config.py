import getpass
from os import (makedirs, path)
from configparser import ConfigParser
from library.color import color 

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
      answer  = str(input(f"[{color.light.grey('?')}] You are going to overwrite your configuration, Do you want to continue [{color.bold('yes/no')}]: "))
      if answer == 'no' or answer == 'n': return
    
    parser["main"] = default_config
    with open(self.config_file_path, 'w') as configfile:
      parser.write(configfile)
    configfile.close()
    print(f"[{color.light.green('+')}] Config file '% s' created" % self.config_file_path)

  def create_dir(self): 
    if self.dir_exists(): return
    makedirs(self.config_dir_path)
    print(f"[{color.light.green('+')}] Directory '% s' created" % self.config_dir_path)

  def set(self, section, option, value): 
    if not self.file_exists(): self.create_file()

    parser.read_file(open(self.config_file_path))

    parser.set(section, option, value)
    parser.write(open(self.config_file_path, 'w'))

  def get(self, section, option): 
    if not self.file_exists(): self.create_file()

    parser.read_file(open(self.config_file_path))
    return parser.get(section, option)