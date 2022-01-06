from os import path

def isValidPath(path_):
  if not path.exists(path_): return False
  return True