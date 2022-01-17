''' backup files '''

import os
import shutil
import sys
import threading
import time
from tqdm import tqdm
from ext.manage_dotfiles_list import Dotfiles_list_manager
from ext.manage_config import Config
from ext.color import color

config = Config()

def create_destination_folders(list):
  for item in list:
    item_source = item["source"]
    item_copy_to_destination = item["destination"]
    dirname = os.path.dirname(item_copy_to_destination)
    dotfiles_dir_path = config.get("main", "dotfiles_path")
    
    if not os.path.exists(dotfiles_dir_path+dirname):
      answer = input(f"{dotfiles_dir_path+dirname} directory does not exists, \nDo you want to create it? [{color.bold('yes/no')}]: ")
      if answer == "yes" or answer == "y":
        os.mkdir(dotfiles_dir_path+dirname) 
        print(f"[{color.light.green('+')}] Directory '% s' is created" % dirname)

def copy_files(source, destination):
  if os.path.isfile(source):
    shutil.copy2(source, destination)
  elif os.path.isdir(source):
    shutil.copytree(source, destination, dirs_exist_ok=True)

def backup():
  ''' loop through dotfiles list and backup all files '''
  dotfiles_list_manager = Dotfiles_list_manager()
  dotfiles_list = dotfiles_list_manager.get_dotfiles_list()
  dotfiles_dir_path = config.get("main", "dotfiles_path")

  create_destination_folders(dotfiles_list)

  with tqdm(total=len(dotfiles_list), desc="copying files", ascii=".#") as bar:
    for item in dotfiles_list:
      item_source = item["source"]
      item_copy_to_destination = item["destination"]
      dirname = os.path.dirname(item_copy_to_destination)

      copy_files(item_source, os.path.join(dotfiles_dir_path, item_copy_to_destination))
      bar.update(1)