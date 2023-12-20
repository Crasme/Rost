from mod.Colorprint import colorprint
import mod.Filesystem as Filesystem
import os

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])
command = lambda command : os.system(command)

DISK_SIZE = 1000 # in sectors
CHUNK_SIZE = 512
assert CHUNK_SIZE == 512, "Error in chunk size"

def make_filesystem():
    file_content = Filesystem.make_disk(DISK_SIZE, CHUNK_SIZE)

    # on recrée le vrai disque
    true_disk = open("./output/rost.iso", "rb")
    true_file_content = list(true_disk.read())
    true_file_content = [true_file_content[i:i+CHUNK_SIZE] for i in range(0, len(true_file_content), CHUNK_SIZE)]
    i = 0
    while true_file_content[i] != [ord("@")]*512:
        i += 1
    i += 1
    DIFF = i
    while i-DIFF < DISK_SIZE:
        true_file_content[i] = file_content[i-DIFF]
        i += 1

    file = open("./output/rost.iso", "wb")
    for thing in true_file_content:
        file.write(bytes(thing))
    file.close()

    log("Fin de mise des fichiers dans le disque", Log.INFO)

def build():
    command("clear")
    log("Lancement du build", Log.INFO)

    # TODO : passer sur les fichiers et vérifier le formatage

    command("rm -Rf ./output/disk/*")
    command("mkdir output")
    command("mkdir output/disk")
    # on lance clippy avant
    # TODO : setup
    command("cd rost && cargo clippy")
    # on compile
    command("cd rost && cargo build")
    command("rm ./output/rost.iso")
    command("cd rost && cargo bootimage")
    command("mv ./rost/target/x86_64-rost/debug/bootimage-rost.bin ./output/rost.iso")

    # TODO : add settings

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
    if file_content[-512:] == [ord("@")]*512: # type: ignore
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
    make_filesystem()

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