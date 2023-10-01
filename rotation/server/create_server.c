#include <stdio.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#include "rand/ip.h"

#define MAX 80
#define SA struct sockaddr

/*
    Creating proxy server for rotation
*/

// Function to create server
void create_violet_server(int port) {
    int sockfd, connfd, len;
    struct sockaddr_in server_addr, cli;

    sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if(sockfd == -1) {
        perror("Socket creation failed...\n");
        exit(0);
    }

    printf("Socket successfully created..\n");
    bzero(&server_addr, sizeof(server_addr));

    server_addr.sin_family = AF_INET;
    server_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    server_addr.sin_port = htons(port);

    if(bind(sockfd, (SA*)&server_addr, sizeof(server_addr)) != 0) {
        perror("Socket bind failed...\n");
        exit(0);
    }

    printf("Socket successfully binded..\n");

    if(listen(sockfd, 5) != 0) {
        perror("Listen failed...\n");
        exit(0);
    }

    printf("Server listening..\n");

    len = sizeof(cli);

    connfd = accept(sockfd, (SA*)&cli, &len);
    if(connfd < 0) {
        perror("Server accept failed...\n");
        exit(0);
    }

    printf("Server accept the client...\n");

    char buff[MAX];
    int n;

    bzero(buff, MAX);
    read(connfd, buff, sizeof(buff));
    printf("From client: %s\n", buff);

    // Creating random ip
    char* created_ip = create_rand_ip();
    printf("Created ip: %d\n", created_ip);

    close(sockfd);
}