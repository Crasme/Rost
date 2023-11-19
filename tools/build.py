from mod.Colorprint import colorprint

class Log:
    INFO    = ("green", "INFO")
    WARNING = ("yellow", "WARNING")
    ERROR   = ("red", "ERROR")

log = lambda text, level : colorprint(f"[{level[1]}] {text}", color=level[0])

log("Lancement du build", Log.INFO)



log("Build terminé", Log.INFO)