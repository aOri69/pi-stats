import subprocess

def get_power_consumption() -> (
    tuple[dict[str, float], dict[str, float], dict[str, float]]
):
    result = subprocess.run(["vcgencmd", "pmic_read_adc"], stdout=subprocess.PIPE)
    lines = result.stdout.decode("utf-8").split("\n")
    curs: dict[str, float] = {}
    vols: dict[str, float] = {}
    pows: dict[str, float] = {}
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
            # print("%10s %10f V %10f A %10f W" %
            #        (key, vols[key], curs[key], pows[key]))
            del curs[key]
            del vols[key]
    return (vols, curs, pows)


def get_temp() -> float:
    result = subprocess.run(["vcgencmd", "measure_temp"], stdout=subprocess.PIPE)
    value = result.stdout.decode("utf-8")
    value = value.removeprefix("temp=")
    value = value.removesuffix("'C\n")
    return float(value)


def get_clock() -> int:
    result = subprocess.run(
        ["vcgencmd", "measure_clock", "arm"], stdout=subprocess.PIPE
    )
    value = result.stdout.decode("utf-8").strip("\n").split("=")[1]
    return int(int(value) / 1000000)


def get_throttled() -> dict:
    out = subprocess.run(
        ["vcgencmd", "get_throttled"], stdout=subprocess.PIPE
    ).stdout.decode("utf-8")
    hex_val = out.split("=")[1].strip()
    binary_val = format(int(hex_val[2:], 16), "020b")
    test = binary_val[16:][3]
    test = binary_val[16:][2]
    test = binary_val[16:][1]

    def state(s):
        return True if s == "1" else False

    response = {}
    response["raw_data"] = hex_val
    response["binary"] = binary_val
    # print(hex_val)
    # print(binary_val)
    response["breakdown"] = {}
    response["breakdown"]["0"] = state(binary_val[16:][3])
    response["breakdown"]["1"] = state(binary_val[16:][2])
    response["breakdown"]["2"] = state(binary_val[16:][1])
    response["breakdown"]["3"] = state(binary_val[16:][0])
    response["breakdown"]["16"] = state(binary_val[0:4][3])
    response["breakdown"]["17"] = state(binary_val[0:4][2])
    response["breakdown"]["18"] = state(binary_val[0:4][1])
    response["breakdown"]["19"] = state(binary_val[0:4][0])
    return response