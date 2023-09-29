#include "dropdowns.hpp"
#include <iostream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include <unistd.h>
#include <netdb.h>
#include <sys/socket.h>

extern "C" {
    std::string send_http_request(const std::string& path, const char* url) {
        int sockfd = socket(AF_INET, SOCK_STREAM, 0);
        if (sockfd < 0) {
            std::cerr << "ERROR opening socket" << std::endl;
            exit(1);
        }

        struct hostent* server = gethostbyname(url);
        if (server == NULL) {
            std::cerr << "ERROR, no such host" << std::endl;
            exit(1);
        }

        std::string request_string = "GET " + path + " HTTP/1.1\r\nHost: " + url + "\r\n\r\n";
        send(sockfd, request_string.c_str(), request_string.length(), 0);

        std::string response;
        char buffer[1024];
        int bytes_received;

        while ((bytes_received = recv(sockfd, buffer, sizeof(buffer), 0)) > 0) {
            response.append(buffer, bytes_received);
        }

        close(sockfd);
        return response;
    }
}
