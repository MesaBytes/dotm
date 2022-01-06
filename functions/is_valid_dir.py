from os import path

def isValidDir(path_):
  if not path.exists(path_): return False
  return True