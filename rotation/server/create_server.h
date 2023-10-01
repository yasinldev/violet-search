#ifndef CREATE_SERVER_H
#define CREATE_SERVER_H

#include <stdio.h>
#include <netdb.h>
#include <netinet/in.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>

#define MAX 80
#define SA struct sockaddr

/*
    Creating proxy server for rotation
*/

// Function to create server
int create_violet_server(int port) {
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

    for(;;) {
        bzero(buff, MAX);

        read(connfd, buff, sizeof(buff));
        printf("From client: %s\t To client: ", buff);
        bzero(buff, MAX);
        n = 0;

        while(buff[n++] = getchar() != '\n') 
            ;

        write(connfd, buff, sizeof(buff));
        if(strncmp("close", buff, 5) == 0) {
            printf("Server exit...\n");
            break;
        }
    }

    close(sockfd);
}
#endif;