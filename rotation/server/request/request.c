#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <netinet/in.h>
#include <netdb.h>
#include <unistd.h>

#include "../rand/ip.h"

#define MAX 1024
#define SA struct sockaddr

void request_init(int client_sock, int port) {
    printf("========================================\n");
    printf("FILE: request/request.c\n");
    printf("========================================\n");

    int server_sock;
    struct sockaddr_in server_addr;

    server_sock = socket(AF_INET, SOCK_STREAM, 0);
    if (connect(server_sock, (SA*)&server_addr, sizeof(server_addr)) != 0) {
        perror("Connection with the server failed...\n");
        exit(1);
    }

    printf("Socket successfully connected..\n");
    bzero(&server_addr, sizeof(server_addr));

    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = inet_addr(create_rand_ip());
    server_addr.sin_port = htons(port);

    if(connect(server_sock, (SA*)&server_addr, sizeof(server_addr)) != 0) {
        perror("Connection with the server failed...\n");
        exit(1);
    }
    
    char buffer[MAX];
    int n;

    bzero(buffer, MAX);
    read(client_sock, buffer, sizeof(buffer));

    printf("Request: %s\n", buffer);

    write(server_sock, buffer, sizeof(buffer));

    bzero(buffer, MAX);
    while((n = read(server_sock, buffer, sizeof(buffer))) > 0) {
        write(client_sock, buffer, n);
        bzero(buffer, MAX);
    }

    close(server_sock);
    close(client_sock);
}