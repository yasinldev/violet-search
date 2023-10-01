#include <stdio.h>
#include <stdlib.h>
#include <time.h>

char* create_rand_ip() {
    srand(time(NULL));

    int octet1 = rand() % 255;
    int octet2 = rand() % 255;
    int octet3 = rand() % 255;

    // create random ip
    char ip[16];
    sprintf(ip, "%d.%d.%d", octet1, octet2, octet3);

    //printf("Random IP: %s\n", ip);

    return ip;
}