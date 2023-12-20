from typing import List, Tuple
import sys
import os

command = lambda command : os.system(command)

FOLDER_S = 0x01
FOLDER_I = 0x02
FILE_H   = 0x04
FILE_I   = 0x08

FOLDER_NAME_SIZE = 16

def make_disk(disk_size: int, chunk_size: int) -> List[List[int]]:
    """
    Make the disk with the file content
    """

    disk = [[0]*chunk_size for _ in range(disk_size)]

    # on set le premier secteur
    value = 0xdeadbeef
    for i in range(4):
        disk[0][i] = (value & ((0xFF) << i*8)) >> i*8

    # on met le secteur 1, le root, qu'on set de toute façon
    NAME = "/"
    disk[1] = [
        FOLDER_S, 0, 0, 0,
        *[ord(x) for x in NAME] + [0]*(FOLDER_NAME_SIZE-len(NAME)),
        *[0 for _ in range(chunk_size-FOLDER_NAME_SIZE-4)]
    ]

    NAME = "test"
    disk[2] = [
        FOLDER_S, 0, 0, 0,
        *[ord(x) for x in NAME] + [0]*(FOLDER_NAME_SIZE-len(NAME)),
        *[0 for _ in range(chunk_size-FOLDER_NAME_SIZE-4)]
    ]

    # TODO : read output/disk/*

    return disk
