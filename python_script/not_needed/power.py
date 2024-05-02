#!/bin/python
import subprocess
import time
import sys


def printq(arg):
    if not quiet:
        print(arg)


quiet = False
period = 0
for opt in sys.argv[1:]:
    if opt in ("help", "--help", "-h"):
        print("Usage: %s [-q] <period>" % sys.argv[0])
        print(
            "Prints detailed power consumption details for the Raspberry Pi by parsing the output of `vcgencmd pmic_read_adc`.\n"
        )
        print(
            "If <period> is provided, the output is refreshed every <period> seconds."
        )
        print("-q quiet output (only print total power consumption)")
    elif opt in ("-q"):
        quiet = True
    else:
        period = float(opt)
while 1:
    result = subprocess.run(
        ["vcgencmd", "pmic_read_adc"], stdout=subprocess.PIPE)
    lines = result.stdout.decode("utf-8").split("\n")
    curs = {}
    vols = {}
    pows = {}
    for n in range(len(lines)):
        line = lines[n].strip()
        if not len(line):
            continue
        label = line.split(" ")[0]
        val = float(line.split("=")[1][0:-1])
        shortLabel = label[0:-2]
        if label[-1] == "A":
            curs[shortLabel] = val
        else:
            vols[shortLabel] = val
    leftover = []
    for key in list(curs.keys()):
        if key in vols:
            pows[key] = curs[key] * vols[key]
            printq("%10s %10f V %10f A %10f W" %
                   (key, vols[key], curs[key], pows[key]))
            del curs[key]
            del vols[key]
    # print any unmatched leftovers
    for key in vols:
        printq("%10s %10f V " % (key, vols[key]))
    for key in curs:
        printq("%10s            %10f A " % (key, curs[key]))
    if quiet:
        print("%.5f W" % sum(pows.values()))
    else:
        print("            Total power consumption: %10f W" %
              sum(pows.values()))
    if not period:
        break
    else:
        time.sleep(period)
        printq("--------------------------------------------------")

def get_power_consumption():
    result = subprocess.run(
        ["vcgencmd", "pmic_read_adc"], stdout=subprocess.PIPE)
    lines = result.stdout.decode("utf-8").split("\n")
    curs = {}
    vols = {}
    pows = {}
    for n in range(len(lines)):
        line = lines[n].strip()
        if not len(line):
            continue
        label = line.split(" ")[0]
        val = float(line.split("=")[1][0:-1])
        shortLabel = label[0:-2]
        if label[-1] == "A":
            curs[shortLabel] = val
        else:
            vols[shortLabel] = val
    leftover = []
    for key in list(curs.keys()):
        if key in vols:
            pows[key] = curs[key] * vols[key]
            printq("%10s %10f V %10f A %10f W" %
                   (key, vols[key], curs[key], pows[key]))
            del curs[key]
            del vols[key]
    # print any unmatched leftovers
    for key in vols:
        printq("%10s %10f V " % (key, vols[key]))
    for key in curs:
        printq("%10s            %10f A " % (key, curs[key]))
    if quiet:
        print("%.5f W" % sum(pows.values()))
    else:
        print("            Total power consumption: %10f W" %
              sum(pows.values()))