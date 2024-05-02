import os
import time


from vcgencmd import Vcgencmd

def main():
    start_time = time.time()
    # fb = open("./readings.txt", "w+")
    # fb.write(
    #     "Elapsed Time (s),"
    #     "Temperature (°C),"
    #     "Clock Speed (MHz),"
    #     "Throttled (bool),"
    #     "PWM Duty (%),"
    #     "FAN Input (rpm)\n"
    # )
    vcgm = Vcgencmd()

    cooling_fan_path = "/sys/devices/platform/cooling_fan/hwmon/"
    fan_dir_contents = os.listdir(cooling_fan_path)
    cooling_fan_pwm = cooling_fan_path + fan_dir_contents[0] + "/pwm1"
    cooling_fan_input = cooling_fan_path + fan_dir_contents[0] + "/fan1_input"

    try:
        while True:
            temp = vcgm.measure_temp()
            clock = int(vcgm.measure_clock("arm") / 1000000)
            throttled = vcgm.get_throttled()["breakdown"]["2"]

            with open(cooling_fan_pwm, "r") as f:
                fan_pwm = f.read()
            with open(cooling_fan_input, "r") as f:
                fan_input = f.read()

            string = "%.0f,%s,%s,%s,%s,%s\n" % (
                (time.time() - start_time),
                temp,
                clock,
                throttled,
                int(fan_pwm),
                int(fan_input),
            )
            print(string, end="")
            # fb.write(string)
            time.sleep(1)
    except KeyboardInterrupt:
        print("program interrupted by Ctrl+C combination")
    except Exception as exc:
        print("program interrupted: ", exc)
    finally:
        # fb.close()
        print("file was closed...\ngraceful exit...")


if __name__ == "__main__":
    main()
