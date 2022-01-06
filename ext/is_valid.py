from os import path

def is_valid_dir(path_):
  if not path.exists(path_): return False
  return True

def is_valid_file(path_):
  if not path.exists(path_): return False
  return True