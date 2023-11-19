from mod.Colorprint import colorprint
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)

log("Lancement du build", Log.INFO)

command("nasm src/os.asm -o output/os.bin")

log("Build terminé", Log.INFO)