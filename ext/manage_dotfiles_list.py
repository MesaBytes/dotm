import getpass
import json
import uuid
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

  def list(self): 
    with open(self.dotfiles_list_path) as fp:
     list = json.load(fp)
      
    print("{:<32} {:<25} {:<10}".format('id','source','dist'))
    for item in list:
      print("{:<32} {:<25} {:<10}".format(item['id'], item['source'], item['dist']))
    fp.close()
  
  def add(self, source: str, dist: str): 
    with open(self.dotfiles_list_path) as fp:
     listobj = json.load(fp)
    
    listobj.append({
      "id": uuid.uuid4().hex,
      "source": source,
      "dist": dist
    })

    with open(self.dotfiles_list_path, 'w') as json_file:
      json.dump(listobj, json_file, 
                  indent=2,  
                  separators=(',',': '))

    fp.close()
    json_file.close()
 
  def remove(self): pass