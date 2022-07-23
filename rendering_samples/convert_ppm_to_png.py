from asyncio import subprocess
from glob import glob
import subprocess

for ppm_file in glob("*.ppm"):
    file_name = ppm_file[:-4]
    subprocess.run("convert {0}.ppm {0}.png".format(file_name), shell=True)
