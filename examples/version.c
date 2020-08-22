#include <stdio.h>
#include "../vendor/linux/x64/ftd2xx.h"

/*
* This C program is equivalent to the `version` rust example.
* This is useful to compare the native C implementation with the rust bindings.
*
* Compile with:
* clang version.c -lftd2xx -L../vendor/linux/x64/build -lpthread
*/
int main(void) {
    DWORD version;
    FT_STATUS status = FT_GetLibraryVersion(&version);
    printf("Status: %d\n", status);
    printf("Version: %d\n", version);
    return 0;
}