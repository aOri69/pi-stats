import os
import time

from typing import Optional

COOLING_FAN_PATH: str = "/sys/devices/platform/cooling_fan/hwmon/"
PWM_SUFFIX: str = "pwm1"
FAN1_INPUT_SUFFIX: str = "fan1_input"


class Fan:
    def __init__(self):
        """Initialize FanData object."""
        self.pwm: Optional[int] = None
        self.input: Optional[int] = None

    def get_fan_data(self) -> tuple[Optional[int], Optional[int]]:
        """Retrieve PWM and input values of the fan.

        Returns:
            Tuple[Optional[int], Optional[int]]:
                A tuple containing the PWM value and the fan input value.

        Note:
            This method assumes that the fan data is available
            in the file system under the path specified by
            COOLING_FAN_PATH constant.
            It reads the PWM and input values from the corresponding files.
        """
        fan_dir_contents: list[str] = os.listdir(COOLING_FAN_PATH)
        cooling_fan_pwm: str = os.path.join(
            COOLING_FAN_PATH, fan_dir_contents[0], PWM_SUFFIX
        )
        cooling_fan_input: str = os.path.join(
            COOLING_FAN_PATH, fan_dir_contents[0], FAN1_INPUT_SUFFIX
        )

        with open(cooling_fan_pwm, "r") as f_pwm, open(
            cooling_fan_input, "r"
        ) as f_input:
            self.pwm = int(f_pwm.read().strip())
            self.input = int(f_input.read().strip())

        return self.pwm, self.input


if __name__ == "__main__":
    fan = Fan()
    while True:
        pwm, fan_input = fan.get_fan_data()
        print(f"{pwm}% PWM duty, {fan_input}rps")
        time.sleep(1)
