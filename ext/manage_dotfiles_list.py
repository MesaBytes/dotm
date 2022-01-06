import getpass
import json
from os import path
from ext.is_valid import is_valid_file
from ext.color import color 

username = getpass.getuser()
class Dotfiles_list_manager:
  ''' manage dotfiles list '''
  def __init__(self):
    self.dotfiles_list_path = f"/home/{username}/.config/dotm/dotfiles.json"
  def file_exists(self): 
    file_exists = path.exists(self.dotfiles_list_path)
    if not file_exists: return False
    return True

  def create_file(self): 
    if not path.exists(self.dotfiles_list_path):
      with open(self.dotfiles_list_path,'w') as f:
        json.dump([], f)
      f.close()
      print(f"[{color.light.green('+')}] dotfiles list file '% s' is created" % self.dotfiles_list_path)

  def list(self): pass
  def add(self, source: str, dist: str): pass
  def remove(self): pass
  def generate_id(self): pass