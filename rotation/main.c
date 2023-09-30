#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <arpa/inet.h>

#define PROXY_PORT 8080

int main() {
    printf("Violet Proxy server is starting... \n");

    int proxy_sock, violet_client_sock;
    struct sockaddr_in proxy_addr, violet_client_addr;
    socklen_t violet_client_len = sizeof(violet_client_addr);

    proxy_sock = socket(AF_INET, SOCK_STREAM, 0);
    if(proxy_sock == -1) {
        perror("Socket connection error");
        exit(1);
    }

    proxy_addr.sin_family = AF_INET;
    proxy_addr.sin_addr.s_addr = INADDR_ANY;
    proxy_addr.sin_port = htons(PROXY_PORT);

    if(bind(proxy_sock, (struct sockaddr*)&proxy_addr, sizeof(proxy_addr)) == -1) {
        perror("Binding error");
        exit(1);
    }

    if(listen(proxy_sock, 5) == -1) {
        perror("Listening error");
        exit(1);
    }

    printf("Proxy server is listening on port %d for Violet IP rotation...", PROXY_PORT);

    while(1) {
        violet_client_sock = accept(proxy_sock, (struct sockaddr*)&violet_client_addr, &violet_client_len);
        if(violet_client_sock == -1) {
            perror("Accept error");
            continue;
        }

        printf("Connection accepted from %s:%d\n", inet_ntoa(violet_client_addr.sin_addr), ntohs(violet_client_addr.sin_port));

        char buffer[4096];
        int bytes_received = recv(violet_client_sock, buffer, sizeof(buffer), 0);
        if(bytes_received > 0) {
            buffer[bytes_received] = '\0';
            printf("HTTP Request Received! Information: \n %s \n", buffer);
        }

        close(violet_client_sock);
    }

    close(proxy_sock);
}