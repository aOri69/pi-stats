import sys
import time

from fan import Fan
from getters import get_power_consumption, get_temp, get_clock, get_throttled


class Settings:
    def __init__(self) -> None:
        self.period = 0.0

def main():
    settings = Settings()
    for opt in sys.argv[1:]:
        if opt in ("help", "--help", "-h"):
            print("Usage: %s <period>" % sys.argv[0])
            return
        else:
            settings.period = float(opt)
    print("--------------------------------------------------")

    fan = Fan()
    start_time = time.time()

    try:
        while True:
            vols, curs, pows = get_power_consumption()
            # print("%10f W" % sum(pows.values()))
            fan_pwm, fan_rps = fan.get_fan_data()
            total_power = sum(pows.values())
            soc_temp = get_temp()
            arm_clock = get_clock()
            throttle = get_throttled()["breakdown"]["2"]

            string = "%.0f,%s,%s,%s,%s,%s,%.6f\n" % (
                (time.time() - start_time),
                soc_temp,
                arm_clock,
                throttle,
                fan_pwm,
                fan_rps,
                total_power,
            )
            print(string, end="")

            if not settings.period:
                break
            else:
                time.sleep(settings.period)
    except KeyboardInterrupt:
        print("program interrupted by Ctrl+C combination")
    except Exception as exc:
        print("program interrupted: ", exc)
    finally:
        # fb.close()
        print("file was closed...\ngraceful exit...")


if __name__ == "__main__":
    main()
