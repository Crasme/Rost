from mod.Colorprint import colorprint
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)

DISK_SIZE = 1000 # in sectors

def build():
    command("clear")
    log("Lancement du build", Log.INFO)

    # TODO : passer sur les fichiers et vérifier le formatage

    command("mkdir output")
    command("cd rost && cargo build")
    command("rm ./output/rost.iso")
    command("cd rost && cargo bootimage")
    command("mv ./rost/target/x86_64-rost/debug/bootimage-rost.bin ./output/rost.iso")

    log("Fin du build", Log.INFO)

def run():
    command("clear")
    log("Lancement de Rost", Log.INFO)

    # add some 0 to the end of the iso
    file = open("./output/rost.iso", "rb")
    file_content = list(file.read())
    print(f"{len(file_content) // 512} sectors before")
    # we check if we already added
    # todo : extract and insert disk
    if file_content[-512:] == [ord("@")]*512:
        print("Disk already made...")
    else:
        print("Adding disk...")
        while len(file_content) % 512:
            file_content.append(0)
        file_content.extend([ord("@")]*512)
        file_content.extend([0]*512*DISK_SIZE)
        file_content.extend([ord("@")]*512)
    file_as_text = bytes(file_content)
    print(f"{len(file_content) // 512} sectors after")
    file.close()
    file = open("./output/rost.iso", "wb")
    file.write(file_as_text)
    file.close()
    command("qemu-system-x86_64 -drive format=raw,file=output/rost.iso -serial stdio")

    log("Arret de Rost", Log.INFO)

def help():
    print("Commands :")
    print("  make build   // compiles the os")
    print("  make run     // build the disk and run the os")
    print("  make clear   // cleans everything")

import sys
match sys.argv[1]:
    case "build":
        build()
    case "run":
        run()
    case "help":
        help()
    case _:
        log("Commande inconnue", Log.ERROR)
        exit(1)