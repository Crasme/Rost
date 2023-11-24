from mod.Colorprint import colorprint
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)

log("Lancement du build", Log.INFO)

command("clear")
command("cd rost && cargo build")
command("rm ./output/rost.iso")
command("cd rost && cargo bootimage")
command("mv ./rost/target/x86_64-rost/debug/bootimage-rost.bin ./output/rost.iso")

log("Build terminé", Log.INFO)