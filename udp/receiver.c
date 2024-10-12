#include <stdio.h>
#include <sys/socket.h>
#include <netinet/ip.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>
#include <arpa/inet.h>

#define BUFF_LEN 1024
#define DEFAULT_PORT 2000

extern int errno;

void rcv_udp_msg(int sockfd) {
    char buff[BUFF_LEN];
    int ret;
    struct sockaddr_in src;
    socklen_t len;

    while (1) {
        memset(buff, 0, BUFF_LEN);
        memset(&src, 0, sizeof(src));
        len = sizeof(src);
        ret = recvfrom(sockfd, buff, BUFF_LEN, 0, (struct sockaddr*)&src, &len);
        if (ret < 0) {
            fprintf(stderr, "Error on receiving message: %d: %s.\n", errno, strerror(errno));
            exit(-1);
        }
        printf("Received: %sFrom:%s\n\n", buff, inet_ntoa(src.sin_addr));
        if (strncmp(buff, "Exit!", 6) == 0) {
            break;
        }
    }
}

int main(int argc, char** argv) {
    int rcv_sockfd, ret;
    struct sockaddr_in rcv_addr;
    rcv_sockfd = socket(AF_INET, SOCK_DGRAM, 0);
    if (rcv_sockfd < 0) {
        fprintf(stderr, "Can't create socket: %s\n", strerror(errno));
        exit(-1);
    }

    memset(&rcv_addr, 0, sizeof(rcv_addr));
    rcv_addr.sin_family = AF_INET;
    rcv_addr.sin_addr.s_addr = htonl(INADDR_ANY);
    rcv_addr.sin_port = htons(DEFAULT_PORT);

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            printf("Listening on specified port and receive messages.\n\
Argument:\n\t-p: Specify a port, \t\tdefault is \"2000\".\n");
            return 0;
        } 
        else if (strcmp(argv[i], "-p") == 0) {
            rcv_addr.sin_port = htons(atoi(argv[++i]));
        } else {
            printf("You entered a wrong argument! Use \"-h\" or \"--help\" to get help.\n");
            exit(-1);
        }
    }

    ret = bind(rcv_sockfd, (struct sockaddr*)&rcv_addr, sizeof(rcv_addr));
    if (ret < 0) {
        fprintf(stderr, "Error on binding: %s\n", strerror(errno));
        exit(-1);
    }

    rcv_udp_msg(rcv_sockfd);

    close(rcv_sockfd);
    return 0;
}