from mod.Colorprint import colorprint
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)

def build():
    command("clear")
    log("Lancement du build", Log.INFO)

    # TODO : passer sur les fichiers et vérifier le formatage

    command("cd rost && cargo build")
    command("rm ./output/rost.iso")
    command("cd rost && cargo bootimage")
    command("mv ./rost/target/x86_64-rost/debug/bootimage-rost.bin ./output/rost.iso")

    log("Fin du build", Log.INFO)

def test():
    command("clear")
    log("Lancement des tests", Log.INFO)

    command("cd rost && cargo test")

    log("Fin des tests", Log.INFO)

import sys
match sys.argv[1]:
    case "build":
        build()
    case "test":
        test()
    case _:
        log("Commande inconnue", Log.ERROR)
        exit(1)