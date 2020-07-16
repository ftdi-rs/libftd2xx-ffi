#include <stdio.h>
#include "../libftd2xx_src/linux/x64/ftd2xx.h"

/*
* This C program is equivalent to the `num_devices` rust example.
* This is useful to compare the native C implementation with the rust bindings.
*
* Compile with:
* clang num_devices.c -lftd2xx -L../libftd2xx_src/linux/x64/build -lpthread
*/
int main(void) {
    DWORD num_devs;
    FT_STATUS status = FT_ListDevices(&num_devs, NULL, FT_LIST_NUMBER_ONLY);
    printf("Status: %d\n", status);
    printf("Number of devices: %d\n", num_devs);
    return 0;
}