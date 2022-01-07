''' backup files '''

# code source: https://stackoverflow.com/a/51088330/14102610

import os
import shutil
import sys
import threading
import time

progressCOLOR = '\033[38;5;33;48;5;236m' #\033[38;5;33;48;5;236m# copy inside '' for colored progressbar| orange:#\033[38;5;208;48;5;235m
finalCOLOR =  '\033[38;5;33;48;5;33m' #\033[38;5;33;48;5;33m# copy inside '' for colored progressbar| orange:#\033[38;5;208;48;5;208m

BOLD    = '\033[1m'
UNDERLINE = '\033[4m'
CEND    = '\033[0m'

def getPERCECENTprogress(source_path, destination_path):
  time.sleep(.24)
  if os.path.exists(destination_path):
    while os.path.getsize(source_path) != os.path.getsize(destination_path):
      sys.stdout.write('\r')
      percentagem = int((float(os.path.getsize(destination_path))/float(os.path.getsize(source_path))) * 100)
      steps = int(percentagem/5)
      copiado = int(os.path.getsize(destination_path)/1000000)# Should be 1024000 but this get's equal to Thunar file manager report (Linux - Xfce)
      sizzz = int(os.path.getsize(source_path)/1000000)
      sys.stdout.write(("         {:d} / {:d} Mb   ".format(copiado, sizzz)) +  (BOLD + progressCOLOR + "{:20s}".format('|'*steps) + CEND) + ("   {:d}% ".format(percentagem))) # BG progress
      #STYLE#sys.stdout.write(("         {:d} / {:d} Mb   ".format(copiado, sizzz)) +  (BOLD + BlueCOLOR + "▐" + "{:s}".format('█'*steps) + CEND) + ("{:s}".format(' '*(20-steps))+ BOLD + BlueCOLOR + endBLOCK+ CEND) +("   {:d}% ".format(percentagem))) #STYLE# # BG progress# closer to GUI but less compatible (no block bar with xterm) # requires utf8 coding header
      sys.stdout.flush()
      time.sleep(.01)

def copy_files(source, destination):
  if os.path.isdir(destination):
    dst_file = os.path.join(destination, os.path.basename(source))
  else: dst_file = destination
  threading.Thread(name='progresso', target=getPERCECENTprogress, args=(source, dst_file)).start()
  if os.path.isfile(source):
    shutil.copy2(source, destination)
  elif os.path.isdir(source):
    shutil.copytree(source, destination)
    
  time.sleep(.02)
  sys.stdout.write('\r')
  sys.stdout.write(("         {:d} / {:d} Mb   ".format((int(os.path.getsize(dst_file)/1000000)), (int(os.path.getsize(source)/1000000)))) +  (BOLD + finalCOLOR + "{:20s}".format('|'*20) + CEND) + ("   {:d}% ".format(100))) # BG progress 100%
  #STYLE#sys.stdout.write(("         {:d} / {:d} Mb   ".format((int(os.path.getsize(dst_file)/1000000)), (int(os.path.getsize(source)/1000000)))) +  (BOLD + BlueCOLOR + "▐" + "{:s}{:s}".format(('█'*20), endBLOCK) + CEND) + ("   {:d}% ".format(100))) #STYLE# # BG progress 100%# closer to GUI but less compatible (no block bar with xterm) # requires utf8 coding header
  sys.stdout.flush()
  print(" ")
  print(" ")

def backup():
  ''' loop through dotfiles list and backup all files '''
  
