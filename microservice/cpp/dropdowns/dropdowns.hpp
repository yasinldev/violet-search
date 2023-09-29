#ifndef DROPDOWNS_H
#define DROPDOWNS_H

#include <iostream>
#include <cstring>
#include <cstdio>
#include <cstdlib>
#include <unistd.h>
#include <netdb.h>
#include <sys/socket.h>

extern "C" {
    std::string send_http_request(const std::string& path, const char* url);
}

#endif
