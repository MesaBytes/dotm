import getpass
import json
from ext.is_valid import is_valid_file
from os import path

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

  def list(self): pass
  def add(self, source: str, dist: str): pass
  def remove(self): pass
  def generate_id(self): pass