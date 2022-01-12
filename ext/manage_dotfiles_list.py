import getpass
import json
import uuid
import sys
from tabulate import tabulate
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

  def empty_list(self):
    if path.exists(self.dotfiles_list_path):
      with open(self.dotfiles_list_path,'w') as f: json.dump([], f)
      f.close()
      print(f"[{color.light.green('+')}] dotfiles list file '% s' is emptied" % self.dotfiles_list_path)

  def print_list(self) -> int: 
    if not path.exists(self.dotfiles_list_path):
      print(f"can't read from dotfiles.json!")
      return 1

    with open(self.dotfiles_list_path) as fp:
     list = json.load(fp)
      
    if len(list) == 0:
      print("List is empty!")
      return 1

    print(tabulate(list, headers="keys"))
    fp.close()

  def get_dotfiles_list(self):
    with open(self.dotfiles_list_path) as fp:
     list = json.load(fp)
    
    if len(list) == 0: 
      print("Dotfiles list is empty, Add some files with '--add' option!")
      sys.exit(1)

    return list
  
  def add(self, sources, destination: str) -> int: 
    with open(self.dotfiles_list_path) as fp:
     list = json.load(fp)
    
    # if len(list) > 0:
    #   for item in list:
    #     for i in range(len(sources)):
    #       if item["source"] == sources[i]:
    #         print(sources[i] + color.light.red(" is already on the list!"))
    #         return 1

    for i in range(len(sources)):
      list.append({
        "id": str(uuid.uuid4())[:16],
        "source": sources[i],
        "destination": destination+path.basename(sources[i])
      })

    with open(self.dotfiles_list_path, 'w') as json_file:
      json.dump(list, json_file, 
                  indent=2,  
                  separators=(',',': '))

    fp.close()
    json_file.close()
 
  def remove(self, id_list) -> int: 
    with open(self.dotfiles_list_path) as fp:
     list = json.load(fp)
    
    if len(list) == 0: 
      print(color.light.red("List is already empty!"))
      return 1

    for item in list:
      for i in range(len(id_list)):
        if item["id"] == id_list[i]:
          list.remove(item)

    with open(self.dotfiles_list_path, 'w') as json_file:
      json.dump(list, json_file, 
                  indent=2,  
                  separators=(',',': '))
    
    fp.close()
    json_file.close()